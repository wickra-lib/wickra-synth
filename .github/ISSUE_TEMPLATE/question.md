---
name: Question / usage help
about: Ask how to generate something. For open-ended discussion prefer Discussions
title: "[question] "
labels: question
---

> [!NOTE]
> Open-ended questions ("what regime looks most like a real crash?") belong in
> [Discussions](https://github.com/wickra-lib/wickra-synth/discussions). Issues
> are for things with an answer that closes them.

## What are you trying to generate?

<!-- The market, not the API call. -->

## What have you tried?

<!-- The spec you wrote and what it produced. -->

```json
{
  "seed": 42,
  "bars": 20,
  "start_price": 100.0,
  "regimes": [{ "kind": "trend", "len": 20, "drift": 0.002, "vol": 0.01 }],
  "microstructure": { "book_depth": 5, "spread_bps": 4.0, "trade_rate": 8.0 }
}
```

## What is blocking you?

<!--
A specific question beats "does not work". "Why does `range` drift upward when I
raise `drift`?" is answerable; the docs pages for the regimes and the
determinism contract are docs/REGIMES.md and docs/DETERMINISM.md.
-->

## Environment (only if relevant)

- `wickra-synth` version:
- Binding: `Rust / Python / Node.js / WASM / C / C++ / C# / Go / Java / R`
