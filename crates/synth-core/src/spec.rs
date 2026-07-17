//! The generation spec — the complete, data-driven generator order.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// The complete generator order. All substreams are derived deterministically
/// from `seed`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GenSpec {
    /// Master seed; every draw derives from this.
    pub seed: u64,
    /// Total number of OHLCV bars (the regime lengths must sum to this).
    pub bars: usize,
    /// Starting price (must be `> 0`).
    pub start_price: f64,
    /// Timestamp of the first bar.
    #[serde(default = "GenSpec::default_start_ts")]
    pub start_ts: i64,
    /// Seconds per bar (the timeframe).
    #[serde(default = "GenSpec::default_bar_secs")]
    pub bar_secs: i64,
    /// Regimes, processed in order (must be non-empty).
    pub regimes: Vec<Regime>,
    /// Order-book / trade / funding parameters.
    pub microstructure: Microstructure,
}

/// One market regime: a run of `len` bars with a given behaviour.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Regime {
    pub kind: RegimeKind,
    /// Number of bars in this regime (`> 0`).
    pub len: usize,
    /// Expected per-bar log-return drift (its meaning depends on `kind`).
    pub drift: f64,
    /// Per-bar log-return volatility (`>= 0`).
    pub vol: f64,
}

/// The kind of a regime — determines the deterministic price-walk effect.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RegimeKind {
    /// Directed drift with moderate volatility.
    Trend,
    /// Mean-reverting around the regime's start price.
    Range,
    /// A strong negative jump with a down-skew and widened ranges.
    Crash,
    /// Drift-free with high volatility (choppy / vol-cluster).
    Vol,
}

/// Order-book, trade and funding parameters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Microstructure {
    /// Levels per side per snapshot (`> 0`).
    pub book_depth: usize,
    /// Target spread in basis points (`>= 0`; 1 bps = 0.01%).
    pub spread_bps: f64,
    /// Expected trades per bar (`>= 0`; Poisson-drawn).
    pub trade_rate: f64,
    /// Optional periodic funding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub funding: Option<FundingSpec>,
}

/// Periodic funding-rate parameters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FundingSpec {
    /// Emit a funding sample every `interval_bars` bars (`> 0`).
    pub interval_bars: usize,
    /// Base funding rate.
    pub base_rate: f64,
    /// How strongly funding reacts to recent drift.
    pub sensitivity: f64,
}

impl GenSpec {
    fn default_start_ts() -> i64 {
        1_700_000_000
    }
    fn default_bar_secs() -> i64 {
        3600
    }

    /// Parse a `GenSpec` from JSON.
    ///
    /// # Errors
    /// Returns [`Error::Parse`] on malformed JSON and [`Error::BadSpec`] if the
    /// parsed spec violates an invariant.
    pub fn from_json(s: &str) -> Result<Self> {
        let spec: GenSpec = serde_json::from_str(s)?;
        spec.validate()?;
        Ok(spec)
    }

    /// Parse a `GenSpec` from TOML.
    ///
    /// # Errors
    /// Returns [`Error::Parse`] on malformed TOML and [`Error::BadSpec`] if the
    /// parsed spec violates an invariant.
    pub fn from_toml(s: &str) -> Result<Self> {
        let spec: GenSpec = toml::from_str(s).map_err(|e| Error::Parse(e.to_string()))?;
        spec.validate()?;
        Ok(spec)
    }

    /// Validate the spec's invariants.
    ///
    /// # Errors
    /// Returns [`Error::BadSpec`] describing the first invariant violated.
    pub(crate) fn validate(&self) -> Result<()> {
        if self.bars == 0 {
            return Err(Error::BadSpec("bars must be > 0".into()));
        }
        if !(self.start_price.is_finite() && self.start_price > 0.0) {
            return Err(Error::BadSpec("start_price must be finite and > 0".into()));
        }
        if self.bar_secs <= 0 {
            return Err(Error::BadSpec("bar_secs must be > 0".into()));
        }
        if self.regimes.is_empty() {
            return Err(Error::BadSpec("regimes must not be empty".into()));
        }
        let mut total = 0usize;
        for r in &self.regimes {
            if r.len == 0 {
                return Err(Error::BadSpec("regime len must be > 0".into()));
            }
            if !r.drift.is_finite() {
                return Err(Error::BadSpec("regime drift must be finite".into()));
            }
            if !(r.vol.is_finite() && r.vol >= 0.0) {
                return Err(Error::BadSpec("regime vol must be finite and >= 0".into()));
            }
            total += r.len;
        }
        if total != self.bars {
            return Err(Error::BadSpec(
                "sum of regime lengths must equal bars".into(),
            ));
        }
        let m = &self.microstructure;
        if m.book_depth == 0 {
            return Err(Error::BadSpec("book_depth must be > 0".into()));
        }
        if !(m.spread_bps.is_finite() && m.spread_bps >= 0.0) {
            return Err(Error::BadSpec("spread_bps must be finite and >= 0".into()));
        }
        if !(m.trade_rate.is_finite() && m.trade_rate >= 0.0) {
            return Err(Error::BadSpec("trade_rate must be finite and >= 0".into()));
        }
        if let Some(f) = &m.funding {
            if f.interval_bars == 0 {
                return Err(Error::BadSpec("funding.interval_bars must be > 0".into()));
            }
            if !(f.base_rate.is_finite() && f.sensitivity.is_finite()) {
                return Err(Error::BadSpec(
                    "funding base_rate and sensitivity must be finite".into(),
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{GenSpec, Microstructure, Regime, RegimeKind};

    fn base() -> GenSpec {
        GenSpec {
            seed: 42,
            bars: 20,
            start_price: 100.0,
            start_ts: 1_700_000_000,
            bar_secs: 3600,
            regimes: vec![Regime {
                kind: RegimeKind::Trend,
                len: 20,
                drift: 0.002,
                vol: 0.01,
            }],
            microstructure: Microstructure {
                book_depth: 5,
                spread_bps: 4.0,
                trade_rate: 8.0,
                funding: None,
            },
        }
    }

    #[test]
    fn valid_spec_passes() {
        assert!(base().validate().is_ok());
    }

    #[test]
    fn regime_sum_must_equal_bars() {
        let mut s = base();
        s.bars = 21;
        assert!(s.validate().is_err());
    }

    #[test]
    fn start_price_must_be_positive() {
        let mut s = base();
        s.start_price = 0.0;
        assert!(s.validate().is_err());
        s.start_price = -1.0;
        assert!(s.validate().is_err());
    }

    #[test]
    fn regimes_must_not_be_empty() {
        let mut s = base();
        s.regimes.clear();
        s.bars = 0;
        assert!(s.validate().is_err());
    }

    #[test]
    fn zero_book_depth_rejected() {
        let mut s = base();
        s.microstructure.book_depth = 0;
        assert!(s.validate().is_err());
    }

    #[test]
    fn defaults_apply_when_missing() {
        let json = r#"{ "seed": 1, "bars": 2, "start_price": 100.0,
            "regimes": [ { "kind": "vol", "len": 2, "drift": 0.0, "vol": 0.02 } ],
            "microstructure": { "book_depth": 3, "spread_bps": 5.0, "trade_rate": 1.0 } }"#;
        let s = GenSpec::from_json(json).unwrap();
        assert_eq!(s.start_ts, 1_700_000_000);
        assert_eq!(s.bar_secs, 3600);
    }

    #[test]
    fn json_roundtrip() {
        let s = base();
        let json = serde_json::to_string(&s).unwrap();
        let back = GenSpec::from_json(&json).unwrap();
        assert_eq!(back, s);
    }
}
