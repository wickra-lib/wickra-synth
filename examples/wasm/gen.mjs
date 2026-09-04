// A runnable WebAssembly example: generate synthetic microstructure through the
// wasm build and print the first three candles.
//
//   cd bindings/wasm && wasm-pack build --target nodejs
//   node examples/wasm/gen.mjs
//
// Every language example uses the same seed and prints the same candles. The
// wasm build has no default features to switch off and runs the same core code
// as the native one, which is why the numbers below are the numbers Rust,
// Python, Node, C, C++, C#, Go, Java and R print.
import { createRequire } from "node:module";
import path from "node:path";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const here = path.dirname(fileURLToPath(import.meta.url));
const pkg = path.resolve(here, "..", "..", "bindings", "wasm", "pkg", "wickra_synth_wasm.js");

let wasm;
try {
  wasm = require(pkg);
} catch {
  console.error("build the wasm package first: cd bindings/wasm && wasm-pack build --target nodejs");
  process.exit(1);
}

const SPEC = JSON.stringify({
  seed: 42,
  bars: 20,
  start_price: 100.0,
  regimes: [{ kind: "trend", len: 20, drift: 0.002, vol: 0.01 }],
  microstructure: {
    book_depth: 5,
    spread_bps: 4.0,
    trade_rate: 8.0,
    funding: { interval_bars: 8, base_rate: 0.0001, sensitivity: 0.5 },
  },
});

const synth = new wasm.Synth(SPEC);
const out = JSON.parse(synth.command(JSON.stringify({ cmd: "generate" })));

console.log(`wickra-synth ${wasm.version()}`);
console.log(`bars: ${out.candles.length}`);
console.log("first 3 candles:");
for (const candle of out.candles.slice(0, 3)) {
  console.log(`  ${JSON.stringify(candle)}`);
}
