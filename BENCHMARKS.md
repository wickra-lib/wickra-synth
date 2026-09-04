# Benchmarks

Wickra Synth generates synthetic microstructure from a seed; the number that
matters is **generation throughput** — how fast the core turns a `GenSpec` into
candles plus order-book / trade / funding output.

## Methodology

A [criterion](https://github.com/bheisler/criterion.rs) bench in
`crates/synth-bench` times a full `generate` over a fixed `GenSpec`, sweeping the
bar count, order-book depth and trade rate. Every scenario uses seed 42, a single
trend regime (`drift 0.001`, `vol 0.01`), a 4 bps spread and an 8-bar funding
cycle; only the three swept dimensions change. Throughput below is derived from
the criterion median.

- **Measured on:** Windows 11 Pro 26200, AMD Ryzen 9 9950X, 64 GB DDR5.
- **Build:** `cargo bench -p synth-bench` (release, `lto = true`,
  `codegen-units = 1`), full criterion sample, no other load on the machine.
- **Toolchain:** stable Rust, the workspace MSRV floor is 1.86.

There is one engine. Generation is sequential by construction — one PRNG stream
drawn in one fixed order — so there is no parallel variant to compare against,
and the numbers are the same on every feature combination.

The nightly `bench.yml` workflow tracks drift over time. Treat the table as a
ballpark for a machine of this class, not as a spec: a different memory
subsystem moves the 100 000-bar rows the most, because that is where the output
stops fitting in cache.

## Results

The dominant cost is the microstructure, not the price walk. A shallow book with
little trade flow generates candles at roughly **4.3 million per second**;
raising the trade rate to 50 per bar costs about 6× that, and deepening the book
to 20 levels per side about 2×.

| Bars | Book depth | Trades/bar | Time (median) | Candles/s |
|------|-----------|------------|---------------|-----------|
| 1 000 | 5 | 1 | 216 µs | ~4.63 M |
| 1 000 | 5 | 50 | 897 µs | ~1.12 M |
| 1 000 | 20 | 1 | 401 µs | ~2.49 M |
| 1 000 | 20 | 50 | 1.10 ms | ~0.91 M |
| 10 000 | 5 | 1 | 2.34 ms | ~4.28 M |
| 10 000 | 5 | 50 | 14.4 ms | ~0.70 M |
| 10 000 | 20 | 1 | 4.43 ms | ~2.26 M |
| 10 000 | 20 | 50 | 16.1 ms | ~0.62 M |
| 100 000 | 5 | 1 | 25.8 ms | ~3.87 M |
| 100 000 | 5 | 50 | 140 ms | ~0.71 M |
| 100 000 | 20 | 1 | 79.6 ms | ~1.26 M |
| 100 000 | 20 | 50 | 200 ms | ~0.50 M |

Scaling in the bar count is linear up to 10 000 bars. At 100 000 bars the
deep-book rows fall off that line — 100 000 × depth 20 is roughly 20× the
working set of 10 000 × depth 20, and the deepest row is also the widest
run-to-run spread in the whole sweep. That row is a memory-bandwidth
measurement as much as a generation one.

Serializing the result is a separate, comparable cost: a 10 000-bar `GenOutput`
(depth 5, 8 trades per bar) serializes to JSON in **18.1 ms**, so a pipeline that
generates and then writes JSON spends more time in `serde_json` than in the
generator.

Generation is fully deterministic, so for a given seed and spec these runs
produce identical bytes every time; only the timing varies.

## Reproducing

```bash
cargo bench -p synth-bench
```
