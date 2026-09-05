# Architecture

Wickra Synth is one data-driven core with a thin binding surface on top. Its
whole reason to exist is a single guarantee: **for a given seed, the synthetic
market it generates is byte-for-byte identical on every platform and through
every language binding.**

## The layers

```
CONSUMERS  Python · Node.js · WASM · C/C++/C#/Go/Java/R (via the C ABI hub)  ·  Rust directly
      ▲ command_json (JSON in → JSON out)
CORE  crates/synth-core: GenSpec (serde) → portable PRNG (SplitMix64 → xoshiro256++)
                       → price walk + order-book / trade / funding synthesis → GenOutput
      ▼ reference consumer
CLI   crates/synth-cli: generate from a GenSpec file or inline JSON
```

- **`wickra-synth-core`** is the engine. A serde `GenSpec` describes the market regime;
  a fixed portable PRNG drives the price walk and the microstructure synthesis;
  the result is a `GenOutput` of OHLCV candles plus order-book snapshots, trades
  and funding samples. The whole surface is reachable through a single
  `command_json(&str) -> String` boundary.
- **`synth-cli`** is the reference consumer — it loads a `GenSpec` and writes the
  generated data out.
- **The bindings** each forward the same `command_json` string to the core and
  return its response verbatim, which is what makes the ten languages
  byte-identical.

## Determinism is the core property

The one place randomness is allowed is the PRNG inside `wickra-synth-core`, and it is a
**fixed, portable, deterministic** generator: a SplitMix64 seed expander feeding
a xoshiro256++ stream, implemented in the Rust core with explicit `u64`
arithmetic so it produces the identical bit sequence on every target. No binding
ever draws its own randomness — every language calls `command_json`, so every
language sees the same seed produce the same stream. Golden fixtures pin this
across platforms and languages.

The PRNG is a **non-cryptographic** generator chosen for speed and
reproducibility. It must never be used for keys, tokens, or any security
purpose; see [THREAT_MODEL.md](THREAT_MODEL.md).

## Data-driven, not code-driven

Because the generation is described by a serde `GenSpec` (data), the identical
build crosses the C ABI and WASM unchanged — there is no per-language generation
logic to drift. The output mirrors the JSON shapes used across the Wickra
ecosystem so it drops straight into backtests, screeners and RL environments.
