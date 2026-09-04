"use strict";

// The cross-language golden invariant seen from Node: the same command yields
// byte-identical output across calls, and the blessed golden corpus re-matches
// byte-for-byte. The response bytes are what every other binding produces too.

const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const { Synth } = require("../index.js");
const { SPEC, generateCmd } = require("./synth.test.js");

const GOLDEN = path.resolve(__dirname, "..", "..", "..", "golden");

test("generate is byte-identical across calls", () => {
  const a = new Synth(JSON.stringify(SPEC)).command(generateCmd());
  const b = new Synth(JSON.stringify(SPEC)).command(generateCmd());
  assert.strictEqual(a, b);
});

test(
  "generate matches the committed golden byte-for-byte",
  { skip: !fs.existsSync(GOLDEN) ? "golden fixtures not present yet" : false },
  () => {
    const specs = fs
      .readdirSync(path.join(GOLDEN, "specs"))
      .filter((f) => f.endsWith(".json"))
      .sort();
    for (const specFile of specs) {
      const spec = fs.readFileSync(path.join(GOLDEN, "specs", specFile), "utf8");
      const expected = fs
        .readFileSync(path.join(GOLDEN, "expected", specFile), "utf8")
        .trim();
      const got = new Synth(spec).command(generateCmd());
      assert.strictEqual(got, expected, `${specFile} must be byte-identical to the Rust golden`);
    }
  },
);
