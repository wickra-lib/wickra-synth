<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Synth — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed, byte-identical across ten languages" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/ci.svg)](https://github.com/wickra-lib/wickra-synth/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-synth)
[![r-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/license.svg)](https://github.com/wickra-lib/wickra-synth#license)

# Wickra Synth — R

---

**Deterministic synthetic market microstructure — for R. `install.packages("wickrasynth", repos = "https://wickra-lib.r-universe.dev")` — over the C ABI via `.Call`, prebuilt library fetched on install.**

R bindings for the Wickra synthetic-microstructure generator over its C ABI hub,
via `.Call`. A synth is built from a spec JSON and driven over a JSON boundary,
so the result is byte-identical to every other Wickra Synth binding.

## Install

From r-universe:

```r
install.packages("wickrasynth", repos = "https://wickra-lib.r-universe.dev")
```

The package's `configure` downloads the prebuilt C ABI library for this exact
version from the GitHub release and bundles it, so an ordinary install needs
nothing but a C toolchain (Rtools on Windows) for the thin `.Call` glue layer. To
build against a local checkout instead, point it at the header and library with
the environment variables below.

### Requirements

`DESCRIPTION` declares `R (>= 4.1)`. Nothing in the binding needs a feature
newer than base R has had for years — the floor is the oldest release this
project is willing to support rather than a technical minimum, because nothing
older is tested. CI builds against R `release` only.

The native library is not a system dependency: `configure` downloads the C ABI
matching this package's version and bundles it beside the compiled object.

### Building from this repository (contributors)

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

## Quick start

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

### Surface

- **`wksynth_new(spec_json)`** — build a synth from a spec JSON (an external
  pointer; freed by a finalizer). Raises an R error if the spec is invalid.
- **`wksynth_command(synth, cmd_json)`** — run a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `set_spec`,
  `generate`, `generate_stream`, `version`.
- **`wksynth_version()`** — the crate version.

Domain errors (a bad command, an unknown command name) come back as an
`{"ok": false, "error": ...}` response, not as an R error.

### Determinism

The response bytes are identical across languages for a given seed, because the
whole generator lives once in the Rust core and this binding forwards its JSON
verbatim.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of R's native `.Call` interface over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-synth/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-synth>
- **Docs** (guides, spec reference, cookbook): <https://synth.wickra.org>
- **Runnable example:** [`examples/r/`](https://github.com/wickra-lib/wickra-synth/tree/main/examples/r)

- The main project: <https://github.com/wickra-lib/wickra-synth>
- Documentation: <https://wickra.org>

Wickra Synth ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-synth/blob/main/SECURITY.md>.

## Disclaimer

`wickra-synth` generates **synthetic** market data for testing, training and
demonstration. It is not real market data and is not financial advice; it comes
with no warranty.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-synth/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-synth/blob/main/LICENSE-MIT) at your option.
