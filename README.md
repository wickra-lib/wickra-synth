<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Synth — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed, byte-identical across ten languages" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-synth)
[![CI](https://github.com/wickra-lib/wickra-synth/actions/workflows/ci.yml/badge.svg)](https://github.com/wickra-lib/wickra-synth/actions/workflows/ci.yml)
[![CodeQL](https://github.com/wickra-lib/wickra-synth/actions/workflows/codeql.yml/badge.svg)](https://github.com/wickra-lib/wickra-synth/actions/workflows/codeql.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![OpenSSF Scorecard](https://img.shields.io/badge/OpenSSF-Scorecard-3b82f6)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-synth)
[![Deterministic across 10 languages](https://img.shields.io/badge/deterministic%20across-10%20languages-3b82f6)](#use-in-any-language)
[![Docs](https://img.shields.io/badge/docs-wickra.org-3b82f6)](https://wickra.org)

---

# Wickra Synth

**Deterministic synthetic market microstructure — OHLCV, order book, trades and funding from a single seed, byte-identical across ten languages.**

> **Part of the [Wickra ecosystem](https://github.com/wickra-lib):** the same
> data-driven core and ten-language binding surface also power
> [wickra-backtest](https://github.com/wickra-lib/wickra-backtest),
> [wickra-screener](https://github.com/wickra-lib/wickra-screener),
> [wickra-feature-store](https://github.com/wickra-lib/wickra-feature-store) and
> [wickra-gym](https://github.com/wickra-lib/wickra-gym).

Wickra Synth is one data-driven core, `synth-core`: a serde **`GenSpec`**
describes a market regime, a fixed **portable PRNG** (SplitMix64 seeding
xoshiro256++) drives it, and the core emits **OHLCV candles** plus **order-book
snapshots**, **trades** and **funding samples** — realistic synthetic
microstructure for tests, training and demos. Because the RNG lives **only in
the Rust core** and is portable-deterministic, a given seed yields the
**byte-for-byte identical** stream on every platform and through every language
binding. The output mirrors the JSON shapes of the rest of the ecosystem, so it
drops straight into backtests, screeners and RL environments.

Because the spec is **data, not code**, the exact same generation crosses the C
ABI and WASM unchanged. The core is exposed as a **JSON-over-C-ABI data API**
(`Synth::command_json`) in **Rust, Python, Node.js, WASM, C, C++, C#, Go, Java
and R**, so a developer in any language draws the same synthetic market.

## Status

Early development (0.1.0, unreleased). Built out in phases; this scaffold pins
the repository, governance and supply-chain configuration ahead of the core
engine. The generation model and command protocol are being settled and will be
pinned by golden tests.

## Documentation

Deep-dive documentation lands with the later phases. See
[ARCHITECTURE.md](ARCHITECTURE.md) for the workspace layout and the PRNG
determinism model.

## Use in any language

The ten-language binding surface lands in a later phase; every binding forwards
the same `command_json` string to the Rust core, so all ten draw byte-identical
synthetic markets for a given seed.

## Building from source

```bash
cargo build --workspace
cargo test  --workspace
```

## Requirements

- Rust 1.86+ (MSRV).

## Benchmarks

Per-generation throughput is tracked in [BENCHMARKS.md](BENCHMARKS.md); the
numbers land with the bench crate.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). Note the
PRNG is a fast non-cryptographic generator for reproducible simulation — it is
**not** suitable for any security or cryptographic purpose.

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in this work, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.

## Disclaimer

`wickra-synth` generates **synthetic** market data for testing, training and
demonstration. It is not real market data and is not financial advice; it comes
with no warranty.
