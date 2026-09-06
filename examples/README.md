# Examples

A runnable "generate a synthetic market" example in every language. Each one
builds a generator from the **same seeded spec** — seed `42`, 20 bars starting
at price `100.0`, a single trend regime (`drift 0.002`, `vol 0.01`), a 5-level
book at `4 bps` spread and `8` trades/bar, plus an 8-bar funding cycle — and
prints the version and the first three candles.

Because the generator is fully deterministic, **every language prints the exact
same candles**. That is the cross-language guarantee: swap Rust for Go for
Python and the bytes do not move.

The examples are self-contained: the spec is inline, so no external files are
loaded. The same specs are mirrored as loadable fixtures under
[`data/specs/`](data/specs/) for adapting the examples to file input, and the
blessed cross-language golden corpus lives in [`../golden/`](../golden).

| Language | Path | Run |
|----------|------|-----|
| Rust | [`rust/`](rust/) | `cargo run -p wickra-synth-example` |
| Python | [`python/gen.py`](python/gen.py) | `pip install wickra-synth && python examples/python/gen.py` |
| Node.js | [`node/`](node/) | `cd examples/node && npm install && node gen.js` |
| C / C++ | [`c/`](c/) | see below |
| Go | [`go/`](go/) | `cd examples/go && go run .` |
| C# | [`csharp/Gen/`](csharp/Gen/) | `dotnet run --project examples/csharp/Gen` |
| Java | [`java/Gen.java`](java/Gen.java) | see the header comment |
| R | [`r/gen.R`](r/gen.R) | `Rscript examples/r/gen.R` |
| WebAssembly | [`wasm/gen.mjs`](wasm/gen.mjs) | see below |

The native bindings (Python, Node.js) load their own compiled library. The
bindings that go through the C ABI (Go, C#, Java, R, and the C/C++ example
itself) need the C ABI library built first:

```bash
cargo build --release -p wickra-synth-c
```

## C / C++

The C and C++ examples build with CMake and run under ctest:

```bash
cargo build --release -p wickra-synth-c
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

On Windows the build copies `wickra_synth.dll` next to each executable, since
there is no rpath.

The `golden_test` and `golden_testpp` targets are not demos: they replay the
whole [`../golden`](../golden) corpus through the C ABI and the C++ hull and
fail if a single byte moves.

## WebAssembly

The wasm example runs the browser build under Node, so build the package for the
`nodejs` target first:

```bash
cd bindings/wasm && wasm-pack build --target nodejs
node examples/wasm/gen.mjs
```

## Expected output

Six of the ten — Rust, Python, Node.js, WebAssembly, Go and C# — print the
version, the bar count, and the first three candles:

```text
wickra-synth 0.1.1
bars: 20
first 3 candles:
  {"ts":1700000000,"open":100.0,"high":100.74470633,"low":99.19272168,"close":99.93141616,"volume":1362.45253483}
  {"ts":1700003600,"open":99.93141616,"high":102.8484968,"low":98.85044064,"close":101.73596671,"volume":2313.08136017}
  {"ts":1700007200,"open":101.73596671,"high":102.46794824,"low":101.14606306,"close":101.87380028,"volume":918.90818804}
```

The other four — C, C++, Java and R — print the full `generate` response
(candles plus the book, trade and funding streams) instead. Slicing three
candles out of it needs a JSON parser, and none of those four has one as a
dependency; showing the raw response is the honest thing for a language whose
consumer will be parsing it themselves anyway. The candle values are the same,
and that is what the golden corpus pins — in all ten reaches, not in the six
that happen to print a summary.

Two examples re-serialize the parsed JSON before printing it, so their text
differs from the block above in formatting, not in value: JavaScript renders
`100.0` as `100`, so the Node and WebAssembly examples print `"open":100`. The
guarantee is on the bytes the core returns, which is what every binding's golden
test compares — not on how each language's printer formats a float.
