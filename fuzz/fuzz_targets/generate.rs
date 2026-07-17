#![no_main]
//! Fuzz the generator with a bounded spec derived from arbitrary bytes. Sizes
//! (bars, book depth, trade rate) are hard-clamped so the fuzzer cannot ask for
//! an out-of-memory allocation; within those bounds `generate` must never panic
//! and must always produce a finite, well-formed output.

use libfuzzer_sys::fuzz_target;
use synth_core::{generate, FundingSpec, GenSpec, Microstructure, Regime, RegimeKind};

fn byte(data: &[u8], i: usize) -> u8 {
    data.get(i).copied().unwrap_or(0)
}

fuzz_target!(|data: &[u8]| {
    // Seed from the first 8 bytes; the rest steer bounded parameters.
    let mut seed = [0u8; 8];
    for (i, s) in seed.iter_mut().enumerate() {
        *s = byte(data, i);
    }
    let seed = u64::from_le_bytes(seed);

    let bars = 1 + (byte(data, 8) as usize % 64); // 1..=64
    let book_depth = 1 + (byte(data, 9) as usize % 16); // 1..=16
    let trade_rate = f64::from(byte(data, 10) % 40); // 0..=39
    let kind = match byte(data, 11) % 4 {
        0 => RegimeKind::Trend,
        1 => RegimeKind::Range,
        2 => RegimeKind::Crash,
        _ => RegimeKind::Vol,
    };
    let drift = (f64::from(byte(data, 12)) - 128.0) / 5000.0;
    let vol = f64::from(byte(data, 13)) / 5000.0;
    let with_funding = byte(data, 14) & 1 == 1;

    let spec = GenSpec {
        seed,
        bars,
        start_price: 100.0,
        start_ts: 1_700_000_000,
        bar_secs: 3600,
        regimes: vec![Regime {
            kind,
            len: bars,
            drift,
            vol,
        }],
        microstructure: Microstructure {
            book_depth,
            spread_bps: 4.0,
            trade_rate,
            funding: with_funding.then_some(FundingSpec {
                interval_bars: 4,
                base_rate: 0.0001,
                sensitivity: 0.5,
            }),
        },
    };

    if let Ok(out) = generate(&spec) {
        assert_eq!(out.candles.len(), bars);
        for c in &out.candles {
            assert!(c.close.is_finite(), "non-finite close");
        }
    }
});
