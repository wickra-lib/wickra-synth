<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Synth — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed, byte-identical across ten languages" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/ci.svg)](https://github.com/wickra-lib/wickra-synth/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-synth)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-synth-go)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/license.svg)](https://github.com/wickra-lib/wickra-synth#license)

# Wickra Synth — Go

---

**Deterministic synthetic market microstructure — for Go. `go get github.com/wickra-lib/wickra-synth-go` — over the C ABI via cgo, prebuilt library bundled in the module.**

Go bindings for the Wickra synthetic-microstructure generator over its C ABI hub
via cgo. A `Synth` is built from a spec JSON and driven over a JSON boundary, so
the result is byte-identical to every other Wickra Synth binding.

## Install

Use the published **`wickra-synth-go`** module, which bundles the prebuilt C ABI
library for every platform, so `go get` + `go build` works with no extra steps
(a C compiler is still required, as the binding uses cgo):

```bash
go get github.com/wickra-lib/wickra-synth-go
```

`wickra-synth-go` is generated from this directory by the release pipeline: it mirrors
the Go sources, the vendored C ABI header (`include/wickra_synth.h`) and the prebuilt
libraries under `lib/<goos>_<goarch>/`. On Linux/macOS the library path is baked
in via rpath; on Windows the DLL must be discoverable at run time (next to the
executable or on `PATH`).

The prebuilt C ABI library is staged per platform under `lib/<goos>_<goarch>/`
and the header is vendored under `include/`. For a local build, copy the library
built by `cargo build -p wickra-synth-c --release` into the matching
`lib/<goos>_<goarch>/` directory (on Windows, ensure that directory is on `PATH`
when running tests).

### Building from this repository (contributors)

This `bindings/go` directory is the development source. To build it directly,
compile the C ABI and stage the library into the per-platform directory cgo
links against:

```bash
cargo build -p wickra-synth-c --release
mkdir -p bindings/go/lib/linux_amd64
cp target/release/libwickra_synth.so bindings/go/lib/linux_amd64/
```

Then, with the library on the loader path, run `go test ./...` from this directory.

## Quick start

```go
package main

import (
	"fmt"

	wickra "github.com/wickra-lib/wickra-synth/bindings/go"
)

func main() {
	spec := `{"seed":42,"bars":20,"start_price":100.0,` +
		`"regimes":[{"kind":"trend","len":20,"drift":0.002,"vol":0.01}],` +
		`"microstructure":{"book_depth":5,"spread_bps":4.0,"trade_rate":8.0}}`

	synth, err := wickra.New(spec)
	if err != nil {
		panic(err)
	}
	defer synth.Close()

	resp, err := synth.Command(`{"cmd":"generate"}`)
	if err != nil {
		panic(err)
	}
	fmt.Println(resp)
}
```

### Surface

- **`New(specJSON string) (*Synth, error)`** — build a synth from a spec JSON.
  Returns an error if the spec is invalid. Call `Close` when done.
- **`(*Synth) Command(cmdJSON string) (string, error)`** — apply a command
  envelope (`{"cmd":"...", ...}`) and return the response JSON. Commands:
  `set_spec`, `generate`, `generate_stream`, `version`.
- **`Version() string`** — the crate version.

Domain errors (a bad command, an unknown command name) come back as an
`{"ok": false, "error": ...}` response, not as a returned `error`. The `error` is
reserved for hard failures at the C ABI boundary (a null handle, invalid UTF-8).

### Determinism

The response bytes are identical across languages for a given seed, because the
whole generator lives once in the Rust core and this binding forwards its JSON
verbatim.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of cgo over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-synth/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-synth>
- **Docs** (guides, spec reference, cookbook): <https://synth.wickra.org>
- **Runnable example:** [`examples/go/`](https://github.com/wickra-lib/wickra-synth/tree/main/examples/go)

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
