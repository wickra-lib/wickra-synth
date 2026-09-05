//! The generation entry points.
//!
//! [`generate`] (batch) and [`generate_stream`] (event list) both run the same
//! single per-bar loop over one PRNG, so for a given seed they produce the
//! byte-identical data — only grouped differently. Per bar the draw order is:
//! candle (price walk), then order book, then trades, then funding when due.

use crate::error::Result;
use crate::microstructure::{build_book, build_funding, build_trades, effective_spread};
use crate::output::{BookSnapshot, Candle, Event, FundingSample, GenOutput, Trade};
use crate::rng::DetRng;
use crate::spec::GenSpec;
use crate::walk::{candle_step, WalkState};

/// Everything generated for a single bar, in the canonical order.
struct BarResult {
    candle: Candle,
    book: BookSnapshot,
    trades: Vec<Trade>,
    funding: Option<FundingSample>,
}

/// The shared per-bar generation loop. Runs one PRNG through the whole spec,
/// interleaving the candle and microstructure draws in the fixed order.
fn generate_core(spec: &GenSpec) -> Result<Vec<BarResult>> {
    spec.validate()?;
    let mut rng = DetRng::from_seed(spec.seed);
    let mut state = WalkState::new(spec.start_price);
    let mut seq: u64 = 0;
    let mut log_rets: Vec<f64> = Vec::with_capacity(spec.bars);
    let mut bars: Vec<BarResult> = Vec::with_capacity(spec.bars);

    let micro = &spec.microstructure;
    let mut bar_ts = spec.start_ts;
    let mut regime_idx = 0usize;
    let mut bars_into_regime = 0usize;

    for i in 0..spec.bars {
        let regime = &spec.regimes[regime_idx];
        let is_regime_start = bars_into_regime == 0;

        let (candle, log_ret) = candle_step(&mut rng, &mut state, regime, bar_ts, is_regime_start)?;
        log_rets.push(log_ret);

        // The exact (un-rounded) close is the mid for this bar's microstructure.
        let mid = state.prev_close;
        let rv = log_ret.abs();
        let spread = effective_spread(mid, micro.spread_bps, rv, regime.vol);

        let book = build_book(&mut rng, mid, spread, bar_ts, micro.book_depth);
        let trades = build_trades(&mut rng, mid, spread, bar_ts, &mut seq, micro.trade_rate);

        let funding = match &micro.funding {
            Some(f) if (i + 1) % f.interval_bars == 0 => {
                let n = f.interval_bars.min(log_rets.len());
                let recent: f64 = log_rets[log_rets.len() - n..].iter().sum::<f64>() / n as f64;
                Some(build_funding(bar_ts, f.base_rate, f.sensitivity, recent))
            }
            _ => None,
        };

        bars.push(BarResult {
            candle,
            book,
            trades,
            funding,
        });

        // Only while there is a next bar to date. Advancing after the last
        // one computed a timestamp nothing reads and, at the end of i64,
        // overflowed on it -- so the bound that has to hold is over `bars - 1`
        // steps, which is what `GenSpec::validate` checks.
        if i + 1 < spec.bars {
            bar_ts += spec.bar_secs;
        }
        bars_into_regime += 1;
        if bars_into_regime == regime.len {
            regime_idx += 1;
            bars_into_regime = 0;
        }
    }
    Ok(bars)
}

/// Generate the complete batch output for a spec.
///
/// # Errors
/// Returns [`crate::Error::BadSpec`] for an invalid spec or
/// [`crate::Error::Numeric`] if the walk produces a non-finite value.
pub fn generate(spec: &GenSpec) -> Result<GenOutput> {
    let bars = generate_core(spec)?;
    let mut candles = Vec::with_capacity(bars.len());
    let mut book_snapshots = Vec::with_capacity(bars.len());
    let mut trades = Vec::new();
    let mut funding = Vec::new();
    for b in bars {
        candles.push(b.candle);
        book_snapshots.push(b.book);
        trades.extend(b.trades);
        if let Some(f) = b.funding {
            funding.push(f);
        }
    }
    Ok(GenOutput {
        candles,
        book_snapshots,
        trades,
        funding,
    })
}

/// Generate the same data as [`generate`], as an ordered event stream. Per bar:
/// all trades (in `seq` order), then the book, then the candle, then funding.
///
/// # Errors
/// Same as [`generate`].
pub fn generate_stream(spec: &GenSpec) -> Result<Vec<Event>> {
    let bars = generate_core(spec)?;
    let mut events = Vec::new();
    for b in bars {
        for trade in b.trades {
            events.push(Event::Trade { trade });
        }
        events.push(Event::Book { snapshot: b.book });
        events.push(Event::Candle { candle: b.candle });
        if let Some(sample) = b.funding {
            events.push(Event::Funding { sample });
        }
    }
    Ok(events)
}

/// Run the wickra-core indicators over a generated stream as a sanity check:
/// every candle field and every indicator output must be finite.
///
/// # Errors
/// Returns [`crate::Error::Numeric`] if any value is non-finite.
#[cfg(feature = "validate")]
pub fn sanity_check(out: &GenOutput) -> Result<()> {
    use crate::error::Error;
    use wickra_core::{Indicator, Sma};

    let mut sma = Sma::new(5).map_err(|e| Error::Numeric(e.to_string()))?;
    for c in &out.candles {
        if !(c.open.is_finite()
            && c.high.is_finite()
            && c.low.is_finite()
            && c.close.is_finite()
            && c.volume.is_finite())
        {
            return Err(Error::Numeric("candle has a non-finite field".into()));
        }
        if let Some(v) = sma.update(c.close) {
            if !v.is_finite() {
                return Err(Error::Numeric(
                    "indicator produced a non-finite value".into(),
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{generate, generate_stream};
    use crate::output::Event;
    use crate::spec::{FundingSpec, GenSpec, Microstructure, Regime, RegimeKind};

    fn trend_spec() -> GenSpec {
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
                funding: Some(FundingSpec {
                    interval_bars: 8,
                    base_rate: 0.0001,
                    sensitivity: 0.5,
                }),
            },
        }
    }

    #[test]
    fn batch_output_shapes() {
        let out = generate(&trend_spec()).unwrap();
        assert_eq!(out.candles.len(), 20);
        assert_eq!(out.book_snapshots.len(), 20);
        // interval_bars = 8 over 20 bars -> samples at bar 7 and 15.
        assert_eq!(out.funding.len(), 2);
        assert_eq!(out.funding[0].ts, 1_700_000_000 + 7 * 3600);
        assert_eq!(out.funding[1].ts, 1_700_000_000 + 15 * 3600);
        // trades are globally seq-ordered and unique.
        for (i, t) in out.trades.iter().enumerate() {
            assert_eq!(t.seq, i as u64);
        }
    }

    #[test]
    fn deterministic_for_same_seed() {
        let a = generate(&trend_spec()).unwrap();
        let b = generate(&trend_spec()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn different_seed_differs() {
        let mut s = trend_spec();
        let a = generate(&s).unwrap();
        s.seed = 43;
        let b = generate(&s).unwrap();
        assert_ne!(a.candles, b.candles);
    }

    #[test]
    fn stream_matches_batch() {
        let spec = trend_spec();
        let batch = generate(&spec).unwrap();
        let events = generate_stream(&spec).unwrap();

        let mut candles = Vec::new();
        let mut books = Vec::new();
        let mut trades = Vec::new();
        let mut funding = Vec::new();
        for e in events {
            match e {
                Event::Candle { candle } => candles.push(candle),
                Event::Book { snapshot } => books.push(snapshot),
                Event::Trade { trade } => trades.push(trade),
                Event::Funding { sample } => funding.push(sample),
            }
        }
        assert_eq!(candles, batch.candles);
        assert_eq!(books, batch.book_snapshots);
        assert_eq!(trades, batch.trades);
        assert_eq!(funding, batch.funding);
    }

    #[test]
    fn no_funding_when_unset() {
        let mut spec = trend_spec();
        spec.microstructure.funding = None;
        let out = generate(&spec).unwrap();
        assert!(out.funding.is_empty());
    }

    #[test]
    fn output_is_all_finite() {
        let out = generate(&trend_spec()).unwrap();
        for c in &out.candles {
            for v in [c.open, c.high, c.low, c.close, c.volume] {
                assert!(v.is_finite());
            }
        }
    }

    #[cfg(feature = "validate")]
    #[test]
    fn sanity_check_passes_on_generated_output() {
        let out = generate(&trend_spec()).unwrap();
        super::sanity_check(&out).unwrap();
    }
}
