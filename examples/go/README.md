# Wickra Synth examples — Go

Runnable Go examples for the [Wickra Synth Go binding](../../bindings/go). The binding links against the
prebuilt C ABI library, so build and stage it once before running anything:

```bash
cargo build -p wickra-synth-c --release
cp target/release/libwickra_synth.so bindings/go/lib/linux_amd64/   # match your GOOS_GOARCH
```

## Run

As the CI examples job runs it, from the repository root:

```bash
mkdir -p bindings/go/lib/linux_amd64
cp target/release/libwickra_synth.so bindings/go/lib/linux_amd64/
export LD_LIBRARY_PATH="$PWD/bindings/go/lib/linux_amd64:${LD_LIBRARY_PATH:-}"
cd examples/go && go run .
```

## The examples

| Example | What it does |
|---------|--------------|
| `gen.go` | A runnable Go example: generate synthetic microstructure and print the first three candles. |
