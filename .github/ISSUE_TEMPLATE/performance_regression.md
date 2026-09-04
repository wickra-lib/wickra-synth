---
name: Performance regression
about: Generation got slower between two versions
title: "[perf] "
labels: performance
---

<!--
Generation throughput is tracked in BENCHMARKS.md and by the bench workflow. If
you have a number that moved, this is the form.
-->

## What got slower

<!-- e.g. `generate` at 100 000 bars, or `command_json` round-trip in the Go binding. -->

## Versions

- Fast version:
- Slow version:
- First version you saw it in (if known):

## The spec

<!-- The complete GenSpec. The seed is part of it; timing varies, output does not. -->

```json
{
  "seed": 42,
  "bars": 100000,
  "start_price": 100.0,
  "regimes": [{ "kind": "trend", "len": 100000, "drift": 0.001, "vol": 0.01 }],
  "microstructure": { "book_depth": 5, "spread_bps": 4.0, "trade_rate": 8.0 }
}
```

## Measurements

<!--
Medians, not single runs, and say how many. `cargo bench -p synth-bench` prints
criterion medians with a confidence interval; a paste of that is ideal.
-->

| Version | Median | Notes |
|---------|--------|-------|
|         |        |       |

## Machine

- CPU:
- RAM:
- OS and architecture:
- Other load during the run:

<!--
Worth checking before filing: the deep-book rows at 100 000 bars are
memory-bandwidth bound, so they move with the machine rather than with the code.
BENCHMARKS.md says which rows those are.
-->

## Binding

- Binding: `Rust / Python / Node.js / WASM / C / C++ / C# / Go / Java / R`
- Build profile: `release / debug`
