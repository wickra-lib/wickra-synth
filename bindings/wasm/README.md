# Wickra Synth — WASM

WebAssembly bindings for the Wickra synthetic-microstructure generator, compiled
from Rust with [wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/). A
`Synth` is built from a spec JSON and driven by command JSONs over a JSON
boundary, so a browser front-end runs against the exact same core as every other
Wickra Synth binding.

## Build

```bash
wasm-pack build --target web      # for a browser bundler
wasm-pack build --target nodejs   # for node:test / Node.js
```

The output lands in `pkg/`.

## Usage

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

## Surface

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

## Determinism

The generator runs single-threaded in the browser sandbox (no worker thread
pool), which is byte-identical to the native run — the exact cross-language
golden invariant. The response bytes match every other binding for a given seed.

## See also

- The main project: <https://github.com/wickra-lib/wickra-synth>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](https://github.com/wickra-lib/wickra-synth/blob/main/LICENSE-MIT) or
[Apache-2.0](https://github.com/wickra-lib/wickra-synth/blob/main/LICENSE-APACHE), at your option.
