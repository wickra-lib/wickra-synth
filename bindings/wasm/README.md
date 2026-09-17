<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Synth — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed, byte-identical across ten languages" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/ci.svg)](https://github.com/wickra-lib/wickra-synth/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-synth)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/npm.svg)](https://www.npmjs.com/package/wickra-synth-wasm)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/license.svg)](https://github.com/wickra-lib/wickra-synth#license)

# Wickra Synth — WASM

---

**Deterministic synthetic market microstructure — for WASM. `npm install wickra-synth-wasm` — pure WebAssembly, runs anywhere a modern JS engine does.**

WebAssembly bindings for the Wickra synthetic-microstructure generator, compiled
from Rust with [wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/). A
`Synth` is built from a spec JSON and driven by command JSONs over a JSON
boundary, so a browser front-end runs against the exact same core as every other
Wickra Synth binding.

## Install

```bash
npm install wickra-synth-wasm
```

### Building from this repository (contributors)

```bash
wasm-pack build --target web      # for a browser bundler
wasm-pack build --target nodejs   # for node:test / Node.js
```

The output lands in `pkg/`.

## Quick start

```js
import init, { Synth } from "./pkg/wickra_synth_wasm.js";

await init();

const spec = JSON.stringify({
  seed: 42,
  bars: 20,
  start_price: 100.0,
  regimes: [{ kind: "trend", len: 20, drift: 0.002, vol: 0.01 }],
  microstructure: { book_depth: 5, spread_bps: 4.0, trade_rate: 8.0 },
});

const synth = new Synth(spec);
const out = JSON.parse(synth.command(JSON.stringify({ cmd: "generate" })));
console.log(out.candles.length); // 20
```

### Surface

- **`new Synth(specJson)`** — build a synth from a spec JSON (throws if the spec
  is invalid).
- **`synth.command(cmdJson) -> string`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `set_spec`,
  `generate`, `generate_stream`, `version`.
- **`synth.version() -> string`** and the module-level **`version()`** — the
  crate version.

An invalid spec throws from the constructor. A malformed command or an unknown
command name is reported in-band as `{"ok":false,"error":...}` (the response
JSON), not thrown.

### Determinism

The generator runs single-threaded in the browser sandbox (no worker thread
pool), which is byte-identical to the native run — the exact cross-language
golden invariant. The response bytes match every other binding for a given seed.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of wasm-bindgen, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-synth/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-synth>
- **Docs** (guides, spec reference, cookbook): <https://synth.wickra.org>
- **Runnable example:** [`examples/wasm/`](https://github.com/wickra-lib/wickra-synth/tree/main/examples/wasm)

- The main project: <https://github.com/wickra-lib/wickra-synth>
- Documentation: <https://wickra.org>

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
