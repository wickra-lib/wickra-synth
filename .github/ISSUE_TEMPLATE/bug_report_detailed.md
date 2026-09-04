---
name: Bug report (detailed)
about: The long form — use it when the short template is not enough to reproduce
title: "[bug] "
labels: bug
---

<!--
Use this form for anything that needs more than a paragraph: a determinism
break, a cross-language mismatch, a crash inside a binding. The short form
(Bug report) is the right one for everything else.
-->

## Summary

<!-- One sentence: what is wrong. -->

## The spec

<!--
The complete GenSpec, including the seed. A synthetic generator takes no market
data — the spec is the entire input, so this section alone should let a
maintainer reproduce the run.
-->

```json
{
  "seed": 42,
  "bars": 20,
  "start_price": 100.0,
  "regimes": [{ "kind": "trend", "len": 20, "drift": 0.002, "vol": 0.01 }],
  "microstructure": { "book_depth": 5, "spread_bps": 4.0, "trade_rate": 8.0 }
}
```

## The command

<!-- The exact command JSON, e.g. {"cmd":"generate"} or {"cmd":"generate_stream"}. -->

## Expected vs actual

- Expected:
- Actual:

<!--
For a numeric difference, paste both values in full rather than rounding. A
mismatch in the last decimal place is a different defect from a mismatch in the
first.
-->

## Is it cross-language?

<!--
Does the same spec produce the same bytes in another binding? If you have only
tried one, say so — it is still a useful report, and narrowing it is our job.
-->

- Bindings tried:
- Bindings that agree:
- Bindings that disagree:

## Is it deterministic?

<!-- Does the same seed reproduce it every run, or only sometimes? -->

## Environment

- `wickra-synth` version:
- Binding: `Rust / Python / Node.js / WASM / C / C++ / C# / Go / Java / R`
- Language runtime version:
- OS and architecture:
- Installed from: `registry / source / release asset`

## Additional context

<!-- Logs, ABI error codes, backtraces, anything else. -->
