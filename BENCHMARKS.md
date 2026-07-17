# Benchmarks

Wickra Synth generates synthetic microstructure from a seed; the number that
matters is **generation throughput** — how fast the core turns a `GenSpec` into
candles plus order-book / trade / funding output.

## Methodology

A [criterion](https://github.com/bheisler/criterion.rs) bench in
`crates/synth-bench` (phase P-SYN-5) times a full `generate` over a fixed
`GenSpec` on x86-64, reporting candles per second and total generation time. The
nightly `bench.yml` workflow tracks drift.

## Results

Numbers land as the bench crate is built out (phase P-SYN-5). This file is the
placeholder the release checklist fills in.

| Scenario | Bars | Generation time | Candles/s |
|----------|------|-----------------|-----------|
| _pending_ | _pending_ | _pending_ | _pending_ |

## Reproducing

```bash
cargo bench -p synth-bench
```
