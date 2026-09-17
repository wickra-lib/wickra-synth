# Wickra Synth examples — R

Runnable R examples for the [Wickra Synth R binding](../../bindings/r). The package compiles a thin
`.Call` glue layer against the C ABI library, so build the library and install
the package first (the CI examples job does exactly this):

```bash
cargo build -p wickra-synth-c --release
R CMD INSTALL bindings/r
```

## Run

```bash
Rscript examples/r/gen.R
Rscript bindings/r/golden_test.R
```

## The examples

| Example | What it does |
|---------|--------------|
| `gen.R` | A runnable R example: generate synthetic microstructure through the binding. |
