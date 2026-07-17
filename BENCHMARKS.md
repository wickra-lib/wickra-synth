# Benchmarks

Wickra Synth generates synthetic microstructure from a seed; the number that
matters is **generation throughput** — how fast the core turns a `GenSpec` into
candles plus order-book / trade / funding output.

## Methodology

A [criterion](https://github.com/bheisler/criterion.rs) bench in
`crates/synth-bench` times a full `generate` over a fixed `GenSpec`, sweeping the
bar count, order-book depth and trade rate. Throughput is reported in candles per
second. The nightly `bench.yml` workflow tracks drift over time.

The numbers below are indicative medians measured locally on an x86-64
development machine with a reduced criterion sample; treat them as ballpark, not
a spec. Reproduce them (with the full sample) via the command at the bottom.

## Results

The dominant cost is the microstructure: a shallow book with little trade flow
generates candles at roughly **1.3 million per second** and scales linearly with
the bar count. Deepening the book or raising the trade rate trades throughput for
detail.

| Scenario | Bars | Book depth | Trades/bar | Time (median) | Candles/s |
|----------|------|-----------|------------|---------------|-----------|
| Light | 1,000 | 5 | 1 | 0.79 ms | ~1.27 M |
| Light | 10,000 | 5 | 1 | 7.6 ms | ~1.31 M |
| Deep book | 10,000 | 20 | 1 | 20.1 ms | ~0.50 M |
| Heavy flow | 10,000 | 5 | 50 | 46.1 ms | ~0.22 M |
| Full | 10,000 | 20 | 50 | 59.0 ms | ~0.17 M |

Generation is fully deterministic, so these times are the same run to run for a
given seed and spec.

## Reproducing

```bash
cargo bench -p synth-bench
```
