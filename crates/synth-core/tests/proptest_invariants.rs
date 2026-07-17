//! Property tests over random specs. Whatever the seed and regime, the output
//! must stay well-formed (no panic, no NaN) and honour the structural invariants
//! every downstream consumer relies on — and the same seed must always produce
//! byte-identical output.

use proptest::prelude::*;
use synth_core::{generate, GenSpec, Microstructure, Regime, RegimeKind};

fn kind_strategy() -> impl Strategy<Value = RegimeKind> {
    prop_oneof![
        Just(RegimeKind::Trend),
        Just(RegimeKind::Range),
        Just(RegimeKind::Crash),
        Just(RegimeKind::Vol),
    ]
}

prop_compose! {
    fn arb_spec()(
        seed in any::<u64>(),
        bars in 1usize..40,
        start_price in 1.0f64..10_000.0,
        kind in kind_strategy(),
        drift in -0.05f64..0.05,
        vol in 0.0f64..0.05,
        book_depth in 1usize..8,
        spread_bps in 0.0f64..20.0,
        trade_rate in 0.0f64..15.0,
        with_funding in any::<bool>(),
    ) -> GenSpec {
        GenSpec {
            seed,
            bars,
            start_price,
            start_ts: 1_700_000_000,
            bar_secs: 3600,
            regimes: vec![Regime { kind, len: bars, drift, vol }],
            microstructure: Microstructure {
                book_depth,
                spread_bps,
                trade_rate,
                funding: with_funding.then_some(synth_core::FundingSpec {
                    interval_bars: 2,
                    base_rate: 0.0001,
                    sensitivity: 0.5,
                }),
            },
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn output_is_well_formed(spec in arb_spec()) {
        let out = generate(&spec).unwrap();

        // One candle and one book snapshot per bar.
        prop_assert_eq!(out.candles.len(), spec.bars);
        prop_assert_eq!(out.book_snapshots.len(), spec.bars);

        for c in &out.candles {
            for v in [c.open, c.high, c.low, c.close, c.volume] {
                prop_assert!(v.is_finite(), "non-finite candle field: {}", v);
            }
            prop_assert!(c.high >= c.open && c.high >= c.close, "high below open/close");
            prop_assert!(c.low <= c.open && c.low <= c.close, "low above open/close");
            prop_assert!(c.high >= c.low, "high below low");
            prop_assert!(c.close > 0.0, "non-positive close");
        }

        for snap in &out.book_snapshots {
            prop_assert_eq!(snap.bids.len(), spec.microstructure.book_depth);
            prop_assert_eq!(snap.asks.len(), spec.microstructure.book_depth);
            // Bids are ordered high→low, asks low→high, and the book is not
            // crossed (a zero spread or rounded ticks can tie adjacent levels).
            for w in snap.bids.windows(2) {
                prop_assert!(w[0].price >= w[1].price, "bids not descending");
            }
            for w in snap.asks.windows(2) {
                prop_assert!(w[0].price <= w[1].price, "asks not ascending");
            }
            prop_assert!(snap.bids[0].price <= snap.asks[0].price, "crossed book");
        }

        // Trade seq is a global strictly-increasing counter.
        for w in out.trades.windows(2) {
            prop_assert!(w[1].seq > w[0].seq, "trade seq not strictly increasing");
            prop_assert!(w[1].ts >= w[0].ts, "trade ts went backwards");
        }
    }

    #[test]
    fn same_seed_is_byte_identical(spec in arb_spec()) {
        let a = serde_json::to_string(&generate(&spec).unwrap()).unwrap();
        let b = serde_json::to_string(&generate(&spec).unwrap()).unwrap();
        prop_assert_eq!(a, b);
    }
}
