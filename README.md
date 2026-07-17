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

Early development (0.1.0, unreleased). The core, the reference CLI, the
ten-language binding surface, the golden corpus and the full CI matrix are in
place; the generation model and command protocol are pinned by golden tests.

## Documentation

- [ARCHITECTURE.md](docs/ARCHITECTURE.md) — workspace layout and the
  core-only-randomness design.
- [GENSPEC.md](docs/GENSPEC.md) — the input specification and its fields.
- [REGIMES.md](docs/REGIMES.md) — the trend / range / crash / vol price-path formulas.
- [MICROSTRUCTURE.md](docs/MICROSTRUCTURE.md) — order book, trades and funding.
- [DETERMINISM.md](docs/DETERMINISM.md) — the PRNG and the fixed draw-order contract.
- [Cookbook.md](docs/Cookbook.md) — practical recipes.

## Use in any language

Every binding forwards the same `command_json` string to the Rust core, so all
ten draw **byte-identical** synthetic markets for a given seed.

```python
from wickra_synth import Synth
import json

synth = Synth('{"seed":42,"bars":20,"start_price":100.0,'
              '"regimes":[{"kind":"trend","len":20,"drift":0.002,"vol":0.01}],'
              '"microstructure":{"book_depth":5,"spread_bps":4.0,"trade_rate":8.0}}')
out = json.loads(synth.command('{"cmd":"generate"}'))
print(len(out["candles"]))  # 20
```

Runnable examples for all ten languages — each printing the same first three
candles — live in [`examples/`](examples/).

## Building from source

```bash
cargo build --workspace
cargo test  --workspace
```

## Requirements

- Rust 1.86+ (MSRV); the Node binding needs Rust 1.88+.

## Benchmarks

Per-generation throughput is tracked in [BENCHMARKS.md](BENCHMARKS.md) and
measured by the `synth-bench` crate (`cargo bench -p synth-bench`).

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
