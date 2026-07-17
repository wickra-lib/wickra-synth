"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const { Synth } = require("../index.js");

const SPEC = {
  seed: 42,
  bars: 8,
  start_price: 100.0,
  regimes: [{ kind: "trend", len: 8, drift: 0.002, vol: 0.01 }],
  microstructure: { book_depth: 3, spread_bps: 4.0, trade_rate: 3.0 },
};

function generateCmd() {
  return JSON.stringify({ cmd: "generate" });
}

test("generate returns the expected candle/book counts", () => {
  const synth = new Synth(JSON.stringify(SPEC));
  const out = JSON.parse(synth.command(generateCmd()));
  assert.strictEqual(out.candles.length, 8);
  assert.strictEqual(out.book_snapshots.length, 8);
  out.trades.forEach((trade, i) => assert.strictEqual(trade.seq, i));
});

test("generate_stream candles match generate byte-for-byte", () => {
  const synth = new Synth(JSON.stringify(SPEC));
  const batch = JSON.parse(synth.command(generateCmd()));
  const events = JSON.parse(
    synth.command(JSON.stringify({ cmd: "generate_stream" })),
  ).events;
  const streamed = events.filter((e) => e.type === "candle").map((e) => e.candle);
  assert.deepStrictEqual(streamed, batch.candles);
});

test("the same seed yields byte-identical output", () => {
  const a = new Synth(JSON.stringify(SPEC)).command(generateCmd());
  const b = new Synth(JSON.stringify(SPEC)).command(generateCmd());
  assert.strictEqual(a, b);
});

test("an unknown command returns an in-band error", () => {
  const synth = new Synth(JSON.stringify(SPEC));
  const out = JSON.parse(synth.command(JSON.stringify({ cmd: "nope" })));
  assert.strictEqual(out.ok, false);
  assert.match(out.error, /unknown cmd/);
});

test("an invalid spec throws", () => {
  assert.throws(() => new Synth("{ not valid json"));
});

test("version is a string", () => {
  const synth = new Synth(JSON.stringify(SPEC));
  assert.strictEqual(typeof synth.version(), "string");
});

module.exports = { SPEC, generateCmd };
