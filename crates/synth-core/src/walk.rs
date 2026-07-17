//! The deterministic price walk (§6.5).
//!
//! For each bar the walk draws, in this fixed order: (1) a standard-normal
//! `z` (two uniforms via Box-Muller), (2) an intrabar-range uniform, (3) a
//! volume uniform. The microstructure draws (book, trades) follow per bar — see
//! [`crate::microstructure`] and [`crate::generate`]. This order is the
//! determinism contract; see `docs/DETERMINISM.md`.

use crate::error::{Error, Result};
use crate::output::{round_to, Candle};
use crate::rng::DetRng;
use crate::spec::{GenSpec, Regime, RegimeKind};

/// Baseline per-bar volume, scaled by `|z|` and a uniform.
const BASE_VOL: f64 = 1000.0;
/// How strongly volume responds to the magnitude of the return.
const K_VOL: f64 = 0.5;

/// Rolling state threaded through the per-bar walk.
pub(crate) struct WalkState {
    /// Close of the previous bar (the next bar's open). Starts at `start_price`.
    pub prev_close: f64,
    /// Reference price at the start of the current regime (for mean reversion).
    pub regime_start_price: f64,
}

impl WalkState {
    pub(crate) fn new(start_price: f64) -> Self {
        Self {
            prev_close: start_price,
            regime_start_price: start_price,
        }
    }
}

/// Compute one bar's candle and its log-return.
///
/// Draws `z` (2 uniforms), `u_range` (1), `vol_u` (1) — in that order. Updates
/// `state.prev_close` to the exact (un-rounded) close so the walk stays on its
/// exact path; the returned candle carries the rounded, serialized values.
pub(crate) fn candle_step(
    rng: &mut DetRng,
    state: &mut WalkState,
    regime: &Regime,
    bar_ts: i64,
    is_regime_start: bool,
) -> Result<(Candle, f64)> {
    if is_regime_start {
        state.regime_start_price = state.prev_close;
    }
    let open = state.prev_close;

    let z = rng.next_normal();
    let log_ret = match regime.kind {
        RegimeKind::Trend => regime.drift + regime.vol * z,
        RegimeKind::Range => {
            -regime.drift * (state.prev_close / state.regime_start_price).ln() + regime.vol * z
        }
        RegimeKind::Crash => -regime.drift.abs() + regime.vol * (z - 0.5 * z.abs()),
        RegimeKind::Vol => regime.vol * z,
    };
    let close = open * log_ret.exp();

    let u_range = rng.next_f64();
    let range = log_ret.abs().max(regime.vol) * (0.5 + u_range);
    let high = open.max(close) * (range / 2.0).exp();
    let low = open.min(close) * (-range / 2.0).exp();

    let vol_u = rng.next_f64();
    let volume = BASE_VOL * (1.0 + K_VOL * z.abs()) * (0.5 + vol_u);

    if !(open.is_finite()
        && high.is_finite()
        && low.is_finite()
        && close.is_finite()
        && volume.is_finite())
    {
        return Err(Error::Numeric(
            "price walk produced a non-finite value".into(),
        ));
    }

    state.prev_close = close;

    let candle = Candle {
        ts: bar_ts,
        open: round_to(open),
        high: round_to(high),
        low: round_to(low),
        close: round_to(close),
        volume: round_to(volume),
    };
    Ok((candle, log_ret))
}

/// Standalone candle-only walk over the whole spec, used to check OHLC
/// invariants per regime kind. (The batch [`crate::generate`] path interleaves
/// microstructure draws between bars, so its candles differ from these.)
///
/// # Errors
/// Returns [`Error::Numeric`] if the walk produces a non-finite value.
pub fn walk(spec: &GenSpec, rng: &mut DetRng) -> Result<Vec<Candle>> {
    let mut state = WalkState::new(spec.start_price);
    let mut candles = Vec::with_capacity(spec.bars);
    let mut bar_ts = spec.start_ts;
    let mut regime_idx = 0usize;
    let mut bars_into_regime = 0usize;
    for _ in 0..spec.bars {
        let regime = &spec.regimes[regime_idx];
        let is_start = bars_into_regime == 0;
        let (candle, _log_ret) = candle_step(rng, &mut state, regime, bar_ts, is_start)?;
        candles.push(candle);
        bar_ts += spec.bar_secs;
        bars_into_regime += 1;
        if bars_into_regime == regime.len {
            regime_idx += 1;
            bars_into_regime = 0;
        }
    }
    Ok(candles)
}

#[cfg(test)]
mod tests {
    use super::walk;
    use crate::rng::DetRng;
    use crate::spec::{GenSpec, Microstructure, Regime, RegimeKind};

    fn spec_with(kind: RegimeKind, drift: f64, vol: f64) -> GenSpec {
        GenSpec {
            seed: 42,
            bars: 30,
            start_price: 100.0,
            start_ts: 1_700_000_000,
            bar_secs: 3600,
            regimes: vec![Regime {
                kind,
                len: 30,
                drift,
                vol,
            }],
            microstructure: Microstructure {
                book_depth: 3,
                spread_bps: 4.0,
                trade_rate: 2.0,
                funding: None,
            },
        }
    }

    fn assert_ohlc_invariants(kind: RegimeKind, drift: f64, vol: f64) {
        let spec = spec_with(kind, drift, vol);
        let mut rng = DetRng::from_seed(spec.seed);
        let candles = walk(&spec, &mut rng).unwrap();
        assert_eq!(candles.len(), spec.bars);
        for c in &candles {
            assert!(c.high >= c.open.max(c.close), "high below body: {c:?}");
            assert!(c.low <= c.open.min(c.close), "low above body: {c:?}");
            assert!(c.high.is_finite() && c.low.is_finite());
            assert!(c.open > 0.0 && c.close > 0.0, "price went non-positive");
            assert!(c.volume >= 0.0);
        }
    }

    #[test]
    fn trend_invariants() {
        assert_ohlc_invariants(RegimeKind::Trend, 0.002, 0.01);
    }

    #[test]
    fn range_invariants() {
        assert_ohlc_invariants(RegimeKind::Range, 0.1, 0.01);
    }

    #[test]
    fn crash_invariants() {
        assert_ohlc_invariants(RegimeKind::Crash, 0.02, 0.03);
    }

    #[test]
    fn vol_invariants() {
        assert_ohlc_invariants(RegimeKind::Vol, 0.0, 0.05);
    }

    #[test]
    fn timestamps_advance_by_bar_secs() {
        let spec = spec_with(RegimeKind::Trend, 0.001, 0.01);
        let mut rng = DetRng::from_seed(spec.seed);
        let candles = walk(&spec, &mut rng).unwrap();
        for (i, c) in candles.iter().enumerate() {
            assert_eq!(
                c.ts,
                spec.start_ts + i64::try_from(i).unwrap() * spec.bar_secs
            );
        }
    }

    #[test]
    fn extreme_drift_yields_numeric_error() {
        // A finite-but-enormous drift makes exp() overflow to +inf, which must
        // surface as Error::Numeric rather than a silent non-finite candle.
        let spec = spec_with(RegimeKind::Trend, 1.0e6, 0.0);
        let mut rng = DetRng::from_seed(spec.seed);
        assert!(walk(&spec, &mut rng).is_err());
    }
}
