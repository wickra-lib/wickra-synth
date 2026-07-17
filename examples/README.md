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

## Expected output

Every example prints the version, the bar count, and the first three candles.
The candles are identical across all ten languages:

```text
wickra-synth 0.1.0
bars: 20
first 3 candles:
  {"ts":1700000000,"open":100.0,"high":100.74470633,"low":99.19272168,"close":99.93141616,"volume":1362.45253483}
  {"ts":1700003600,"open":99.93141616,"high":102.8484968,"low":98.85044064,"close":101.73596671,"volume":2313.08136017}
  {"ts":1700007200,"open":101.73596671,"high":102.46794824,"low":101.14606306,"close":101.87380028,"volume":918.90818804}
```

The C, C++, Go, C#, Java, and R examples print the full `generate` response
(candles plus the book, trade, and funding streams) rather than slicing the
first three candles, but the candle values are the same — that is what the
golden corpus pins.
