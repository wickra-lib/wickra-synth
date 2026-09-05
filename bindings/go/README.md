<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Synth — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed, byte-identical across ten languages" width="100%"></a>
</p>

# Wickra Synth — Go

Go bindings for the Wickra synthetic-microstructure generator over its C ABI hub
via cgo. A `Synth` is built from a spec JSON and driven over a JSON boundary, so
the result is byte-identical to every other Wickra Synth binding.

## Install

```bash
go get github.com/wickra-lib/wickra-synth/bindings/go
```

The prebuilt C ABI library is staged per platform under `lib/<goos>_<goarch>/`
and the header is vendored under `include/`. For a local build, copy the library
built by `cargo build -p wickra-synth-c --release` into the matching
`lib/<goos>_<goarch>/` directory (on Windows, ensure that directory is on `PATH`
when running tests).

## Usage

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

## Surface

- **`New(specJSON string) (*Synth, error)`** — build a synth from a spec JSON.
  Returns an error if the spec is invalid. Call `Close` when done.
- **`(*Synth) Command(cmdJSON string) (string, error)`** — apply a command
  envelope (`{"cmd":"...", ...}`) and return the response JSON. Commands:
  `set_spec`, `generate`, `generate_stream`, `version`.
- **`Version() string`** — the crate version.

Domain errors (a bad command, an unknown command name) come back as an
`{"ok": false, "error": ...}` response, not as a returned `error`. The `error` is
reserved for hard failures at the C ABI boundary (a null handle, invalid UTF-8).

## Determinism

The response bytes are identical across languages for a given seed, because the
whole generator lives once in the Rust core and this binding forwards its JSON
verbatim.

## See also

- The main project: <https://github.com/wickra-lib/wickra-synth>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](https://github.com/wickra-lib/wickra-synth/blob/main/LICENSE-MIT) or
[Apache-2.0](https://github.com/wickra-lib/wickra-synth/blob/main/LICENSE-APACHE), at your option.
