//! Criterion benchmarks for `wickra-synth-core::generate`.
//!
//! Generation is sequential by construction, so there is one engine and one
//! measurement. Vary bar count, book depth and trade rate to see how each
//! dimension scales.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use wickra_synth_core::{generate, FundingSpec, GenSpec, Microstructure, Regime, RegimeKind};

fn spec(bars: usize, book_depth: usize, trade_rate: f64) -> GenSpec {
    GenSpec {
        seed: 42,
        bars,
        start_price: 100.0,
        start_ts: 1_700_000_000,
        bar_secs: 3600,
        regimes: vec![Regime {
            kind: RegimeKind::Trend,
            len: bars,
            drift: 0.001,
            vol: 0.01,
        }],
        microstructure: Microstructure {
            book_depth,
            spread_bps: 4.0,
            trade_rate,
            funding: Some(FundingSpec {
                interval_bars: 8,
                base_rate: 0.0001,
                sensitivity: 0.5,
            }),
        },
    }
}

fn bench_generate(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate");
    for &bars in &[1_000usize, 10_000, 100_000] {
        for &book_depth in &[5usize, 20] {
            for &trade_rate in &[1.0f64, 50.0] {
                let s = spec(bars, book_depth, trade_rate);
                group.throughput(Throughput::Elements(bars as u64));
                let id = BenchmarkId::from_parameter(format!(
                    "bars={bars}/depth={book_depth}/rate={trade_rate}"
                ));
                group.bench_with_input(id, &s, |b, s| b.iter(|| generate(s).unwrap()));
            }
        }
    }
    group.finish();
}

fn bench_serialize(c: &mut Criterion) {
    let out = generate(&spec(10_000, 5, 8.0)).unwrap();
    c.bench_function("serialize/10k", |b| {
        b.iter(|| serde_json::to_string(&out).unwrap());
    });
}

criterion_group!(benches, bench_generate, bench_serialize);
criterion_main!(benches);
