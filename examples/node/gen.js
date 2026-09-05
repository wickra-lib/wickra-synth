// A runnable Node.js example: generate synthetic microstructure and print the
// first three candles.
//
//   npm install
//   node examples/node/gen.js
//
// Every language example uses the same seed and prints the same candles.
"use strict";

const { Synth } = require("wickra-synth");

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

const synth = new Synth(SPEC);
const out = JSON.parse(synth.command(JSON.stringify({ cmd: "generate" })));

console.log(`wickra-synth ${synth.version()}`);
console.log(`bars: ${out.candles.length}`);
console.log("first 3 candles:");
for (const candle of out.candles.slice(0, 3)) {
  console.log(`  ${JSON.stringify(candle)}`);
}
