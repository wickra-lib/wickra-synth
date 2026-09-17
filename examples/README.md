# Wickra Synth examples

A runnable "generate a synthetic market" example in every language. Each one
builds a generator from the **same seeded spec** — seed `42`, 20 bars starting
at price `100.0`, a single trend regime (`drift 0.002`, `vol 0.01`), a 5-level
book at `4 bps` spread and `8` trades/bar, plus an 8-bar funding cycle — and
prints the version and the first three candles.

## Rust — `examples/rust/`

As the CI examples job runs it, from the repository root:

```bash
cargo run -p wickra-synth-example
```

| Example | What it does |
| --- | --- |
| `src/main.rs` | A runnable Rust example: generate synthetic microstructure from a seeded spec and print the first three candles. |

## C / C++ — `examples/c/`

Build the library first (`cargo build -p wickra-synth-c --release`), then build and run
the examples via CMake, as the CI C ABI job does:

```bash
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

| Example | What it does |
| --- | --- |
| `gen.c` | A runnable C example: generate synthetic microstructure through the |
| `gen.cpp` | A runnable C++ example: generate synthetic microstructure through the wickra-synth C ABI. |

## C# — `examples/csharp/`

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Gen -c Release
```

| Example | What it does |
| --- | --- |
| `Gen/Program.cs` | A runnable C# example: generate synthetic microstructure and print the first three candles. |

## Go — `examples/go/`

As the CI examples job runs it, from the repository root:

```bash
mkdir -p bindings/go/lib/linux_amd64
cp target/release/libwickra_synth.so bindings/go/lib/linux_amd64/
export LD_LIBRARY_PATH="$PWD/bindings/go/lib/linux_amd64:${LD_LIBRARY_PATH:-}"
cd examples/go && go run .
```

| Example | What it does |
| --- | --- |
| `gen.go` | A runnable Go example: generate synthetic microstructure and print the first three candles. |

## R — `examples/r/`

As the CI examples job runs it, from the repository root:

```bash
export LD_LIBRARY_PATH="$WKSYNTH_LIB:${LD_LIBRARY_PATH:-}"
R CMD INSTALL bindings/r
Rscript examples/r/gen.R
Rscript bindings/r/golden_test.R
```

| Example | What it does |
| --- | --- |
| `gen.R` | A runnable R example: generate synthetic microstructure through the binding. |

## Java — `examples/java/`

As the CI examples job runs it, from the repository root:

```bash
mvn -B -q -f bindings/java package -DskipTests
javac -cp bindings/java/target/classes examples/java/Gen.java -d examples/java/out
java --enable-native-access=ALL-UNNAMED  -Dnative.lib.dir=target/debug  -cp "bindings/java/target/classes:examples/java/out" Gen
```

| Example | What it does |
| --- | --- |
| `Gen.java` | A runnable Java example: generate synthetic microstructure through the binding. |

## Python — `examples/python/`

As the CI examples job runs it, from the repository root:

```bash
pip install --require-hashes -r ../../.github/requirements/ci-dev-py3.txt
maturin build --release --out dist
pip install --no-index --find-links dist wickra-synth
python ../../examples/python/gen.py
working-directory: bindings/python
```

| Example | What it does |
| --- | --- |
| `gen.py` | A runnable Python example: generate synthetic microstructure and print the |

## Node.js — `examples/node/`

As the CI examples job runs it, from the repository root:

```bash
cd bindings/node
npm install --no-audit --no-fund
npx napi build --platform --release
cd ../../examples/node
npm install --no-audit --no-fund
node gen.js
```

| Example | What it does |
| --- | --- |
| `gen.js` | A runnable Node.js example: generate synthetic microstructure and print the first three candles. |

## WASM — `examples/wasm/`

Build the WASM package, serve the repository root, and open the page in a browser;
the module script inside it is what runs (CI parses it with `node --check`):

```bash
wasm-pack build bindings/wasm --target web
python -m http.server 8000     # then open http://localhost:8000/examples/wasm/
```

| Example | What it does |
| --- | --- |
| `gen.mjs` | A runnable WebAssembly example: generate synthetic microstructure through the wasm build and print the first three candles. |

## Example datasets

The examples read from [`examples/data/`](data/): . The
cross-language golden fixtures, which every binding is checked against byte for
byte, live in [`../golden/`](../golden).

## WebAssembly

The wasm example runs the browser build under Node, so build the package for the
`nodejs` target first:

```bash
cd bindings/wasm && wasm-pack build --target nodejs
node examples/wasm/gen.mjs
```
