"use strict";

// Golden test over the wasm-pack (nodejs target) output: the WebAssembly build
// generates byte-identically to the native run. Skips cleanly when `pkg/` has
// not been built yet (`wasm-pack build --target nodejs`).

const { test } = require("node:test");
const assert = require("node:assert");
const path = require("node:path");

let wasm = null;
try {
  wasm = require(path.resolve(__dirname, "..", "pkg", "wickra_synth_wasm.js"));
} catch {
  wasm = null;
}

const SPEC = JSON.stringify({
  seed: 42,
  bars: 8,
  start_price: 100.0,
  regimes: [{ kind: "trend", len: 8, drift: 0.002, vol: 0.01 }],
  microstructure: { book_depth: 3, spread_bps: 4.0, trade_rate: 3.0 },
});

function generateCmd() {
  return JSON.stringify({ cmd: "generate" });
}

test("wasm build present or skipped", (t) => {
  if (!wasm) t.skip("run `wasm-pack build --target nodejs` first");
});

if (wasm) {
  test("wasm generate produces the expected candle count", () => {
    const out = JSON.parse(new wasm.Synth(SPEC).command(generateCmd()));
    assert.strictEqual(out.candles.length, 8);
    assert.strictEqual(out.book_snapshots.length, 8);
  });

  test("wasm generate is byte-identical across calls", () => {
    const a = new wasm.Synth(SPEC).command(generateCmd());
    const b = new wasm.Synth(SPEC).command(generateCmd());
    assert.strictEqual(a, b);
  });

  test("wasm generate_stream candles match generate byte-for-byte", () => {
    const batch = JSON.parse(new wasm.Synth(SPEC).command(generateCmd()));
    const events = JSON.parse(
      new wasm.Synth(SPEC).command(JSON.stringify({ cmd: "generate_stream" })),
    ).events;
    const streamed = events.filter((e) => e.type === "candle").map((e) => e.candle);
    assert.deepStrictEqual(streamed, batch.candles);
  });

  test("wasm version matches the module export", () => {
    assert.strictEqual(new wasm.Synth(SPEC).version(), wasm.version());
  });

  test("wasm throws on an invalid spec", () => {
    assert.throws(() => new wasm.Synth("{ not valid json"));
  });

  test("wasm returns an in-band error on an unknown command", () => {
    const out = JSON.parse(new wasm.Synth(SPEC).command('{"cmd":"nope"}'));
    assert.strictEqual(out.ok, false);
    assert.match(out.error, /unknown cmd/);
  });
}
