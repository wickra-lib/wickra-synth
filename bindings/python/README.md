<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Synth — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed, byte-identical across ten languages" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/ci.svg)](https://github.com/wickra-lib/wickra-synth/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-synth)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/pypi.svg)](https://pypi.org/project/wickra-synth/)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/license.svg)](https://github.com/wickra-lib/wickra-synth#license)

# Wickra Synth — Python

---

**Deterministic synthetic market microstructure — for Python. `pip install wickra-synth` — prebuilt wheels for Linux, macOS and Windows, nothing to compile.**

Python bindings for [wickra-synth](https://github.com/wickra-lib/wickra-synth),
the deterministic synthetic-microstructure generator. Build a `Synth` from a
spec JSON, drive it with command JSONs, and read back the generated OHLCV, order
book, trades and funding — the same command protocol every language binding
speaks, byte-identical for a given seed.

## Install

```bash
pip install wickra-synth
```

Pre-built wheels ship for Linux, macOS and Windows — there is nothing to
compile and no C library to track down.

### Building from this repository (contributors)

```sh
maturin develop --release
pytest -q
```

## Quick start

```python
import json
from wickra_synth import Synth

spec = json.dumps({
    "seed": 42,
    "bars": 20,
    "start_price": 100.0,
    "regimes": [{"kind": "trend", "len": 20, "drift": 0.002, "vol": 0.01}],
    "microstructure": {"book_depth": 5, "spread_bps": 4.0, "trade_rate": 8.0},
})

synth = Synth(spec)

out = json.loads(synth.command(json.dumps({"cmd": "generate"})))
print(len(out["candles"]))   # 20
print(out["candles"][0])     # {'ts': 1700000000, 'open': 100.0, ...}

# The same seed yields byte-identical output in every language binding.
```

### API

| Method | Description |
|--------|-------------|
| `Synth(spec_json)` | Build a synth from a spec JSON (raises `ValueError` if invalid). |
| `synth.command(cmd_json) -> str` | Apply a command JSON, return the response JSON. Commands: `set_spec`, `generate`, `generate_stream`, `version`. |
| `Synth.version() -> str` | The library version. |

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of PyO3, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-synth/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-synth>
- **Docs** (guides, spec reference, cookbook): <https://synth.wickra.org>
- **Runnable example:** [`examples/python/`](https://github.com/wickra-lib/wickra-synth/tree/main/examples/python)

Wickra Synth ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-synth/blob/main/SECURITY.md>.

## Disclaimer

`wickra-synth` generates **synthetic** market data for testing, training and
demonstration. It is not real market data and is not financial advice; it comes
with no warranty.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-synth/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-synth/blob/main/LICENSE-MIT) at your option.
