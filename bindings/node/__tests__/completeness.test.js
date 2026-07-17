"use strict";

// Parity guard: the Node binding must expose the full public surface, so an
// export dropped in a refactor fails loudly here.

const { test } = require("node:test");
const assert = require("node:assert");
const wickra = require("../index.js");

test("module exposes Synth", () => {
  assert.strictEqual(typeof wickra.Synth, "function");
});

test("Synth exposes command and version", () => {
  for (const name of ["command", "version"]) {
    assert.strictEqual(
      typeof wickra.Synth.prototype[name],
      "function",
      `Synth is missing ${name}`,
    );
  }
});

test("Synth surface is exactly {command, version}", () => {
  const methods = Object.getOwnPropertyNames(wickra.Synth.prototype)
    .filter((name) => name !== "constructor")
    .sort();
  assert.deepStrictEqual(methods, ["command", "version"]);
});
