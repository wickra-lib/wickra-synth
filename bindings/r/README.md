# Wickra Synth — R

R bindings for the Wickra synthetic-microstructure generator over its C ABI hub,
via `.Call`. A synth is built from a spec JSON and driven over a JSON boundary,
so the result is byte-identical to every other Wickra Synth binding.

## Requirements

`DESCRIPTION` declares `R (>= 4.1)`. Nothing in the binding needs a feature
newer than base R has had for years — the floor is the oldest release this
project is willing to support rather than a technical minimum, because nothing
older is tested. CI builds against R `release` only.

The native library is not a system dependency: `configure` downloads the C ABI
matching this package's version and bundles it beside the compiled object.

## Build & test

The C ABI header and shared library are provided out-of-tree through two
environment variables (set by CI / the installer):

```bash
export WKSYNTH_INC=/path/to/bindings/c/include   # the header dir
export WKSYNTH_LIB=/path/to/target/release       # the library dir
R CMD INSTALL bindings/r
Rscript bindings/r/tests/run_tests.R
```

At run time the loader must find the shared library on `LD_LIBRARY_PATH`
(Linux), `DYLD_LIBRARY_PATH` (macOS) or `PATH` (Windows).

## Usage

```r
library(wickrasynth)

spec <- paste0(
  '{"seed":42,"bars":20,"start_price":100.0,',
  '"regimes":[{"kind":"trend","len":20,"drift":0.002,"vol":0.01}],',
  '"microstructure":{"book_depth":5,"spread_bps":4.0,"trade_rate":8.0}}'
)

synth <- wksynth_new(spec)
response <- wksynth_command(synth, '{"cmd":"generate"}')
cat(response)
```

## Surface

- **`wksynth_new(spec_json)`** — build a synth from a spec JSON (an external
  pointer; freed by a finalizer). Raises an R error if the spec is invalid.
- **`wksynth_command(synth, cmd_json)`** — run a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `set_spec`,
  `generate`, `generate_stream`, `version`.
- **`wksynth_version()`** — the crate version.

Domain errors (a bad command, an unknown command name) come back as an
`{"ok": false, "error": ...}` response, not as an R error.

## Determinism

The response bytes are identical across languages for a given seed, because the
whole generator lives once in the Rust core and this binding forwards its JSON
verbatim.

## See also

- The main project: <https://github.com/wickra-lib/wickra-synth>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either MIT or Apache-2.0, at your option.
