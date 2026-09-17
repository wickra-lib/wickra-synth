<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Synth — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed, byte-identical across ten languages" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/ci.svg)](https://github.com/wickra-lib/wickra-synth/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-synth)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/release.svg)](https://github.com/wickra-lib/wickra-synth/releases/latest)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/license.svg)](https://github.com/wickra-lib/wickra-synth#license)

# Wickra Synth — C / C++

---

**Deterministic synthetic market microstructure — for C / C++. `cargo build -p wickra-synth-c --release` — a prebuilt shared/static library plus a generated `wickra_synth.h`, no system dependencies.**

The C ABI is the hub every C-capable language (C, C++, C#, Go, Java, R) links
against. It exposes `wickra-synth-core` as a tiny, JSON-shaped surface built as
both a `cdylib` (dynamic library) and a `staticlib`.

## Install

Grab the prebuilt header + library for your platform from the
[GitHub releases](https://github.com/wickra-lib/wickra-synth/releases) — each archive
has `wickra_synth.h`, the C++ wrapper where the binding ships one, and the shared/static
library — or build from source:

```bash
cargo build -p wickra-synth-c --release
# -> target/release/libwickra_synth.{so,dylib} or wickra_synth.dll (+ import lib) + a staticlib
```

Then compile against the header and link the library.

## Quick start

[`examples/c/gen.c`](https://github.com/wickra-lib/wickra-synth/blob/main/examples/c/gen.c) is the runnable example the CI smoke job executes; in full:

```c
/* A runnable C example: generate synthetic microstructure through the
 * wickra-synth C ABI and print the raw JSON output. Every language example
 * uses the same seed and prints the same candles. */
#include <stdio.h>
#include <stdlib.h>

#include "wickra_synth.h"

static const char *SPEC =
    "{\"seed\":42,\"bars\":20,\"start_price\":100.0,"
    "\"regimes\":[{\"kind\":\"trend\",\"len\":20,\"drift\":0.002,\"vol\":0.01}],"
    "\"microstructure\":{\"book_depth\":5,\"spread_bps\":4.0,\"trade_rate\":8.0,"
    "\"funding\":{\"interval_bars\":8,\"base_rate\":0.0001,\"sensitivity\":0.5}}}";

static const char *CMD = "{\"cmd\":\"generate\"}";

int main(void) {
    WickraSynth *synth = wickra_synth_new(SPEC);
    if (!synth) {
        fprintf(stderr, "failed to build synth\n");
        return 1;
    }

    /* Length-out protocol: learn the length, then read into a caller buffer. */
    int len = wickra_synth_command(synth, CMD, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        wickra_synth_free(synth);
        return 1;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        wickra_synth_free(synth);
        return 1;
    }
    wickra_synth_command(synth, CMD, buf, (size_t)len + 1);

    printf("wickra-synth %s\n", wickra_synth_version());
    printf("output: %s\n", buf);

    free(buf);
    wickra_synth_free(synth);
    return 0;
}
```

### Surface

```c
#include "wickra_synth.h"

WickraSynth *wickra_synth_new(const char *spec_json);
void                wickra_synth_free(WickraSynth *handle);
int32_t             wickra_synth_command(WickraSynth *handle,
                                                 const char *cmd_json,
                                                 char *out, size_t cap);
const char         *wickra_synth_version(void);
```

- **`wickra_synth_new`** builds a synth from a spec JSON. Returns
  `NULL` if the argument is null, not UTF-8, or not a valid spec.
- **`wickra_synth_free`** destroys a handle (null is a no-op).
- **`wickra_synth_command`** applies a command JSON and writes the
  response JSON into the caller's buffer using a length-out protocol (below).
- **`wickra_synth_version`** returns a static, NUL-terminated version
  string (do not free).

### Command / response protocol

Everything after construction goes through `wickra_synth_command`.
Commands are JSON objects with a `"cmd"` field: `set_spec`, `generate`,
`generate_stream`, `version`. Responses are JSON, e.g. a `GenOutput`
`{"candles":[...],"book_snapshots":[...],"trades":[...],"funding":[...]}` for
`generate`, an `{"events":[...]}` list for `generate_stream`, or `{"ok":true}`
for `set_spec`.

The response is returned via a caller-owned buffer with a length-out protocol —
the callee never allocates memory the caller must free:

1. Call with `out = NULL`, `cap = 0` to learn the response length `len`
   (excluding the terminating NUL).
2. Allocate `len + 1` bytes and call again; the response plus a NUL is written.

Whenever `len < cap`, the response is written on that call, so a
sufficiently-large buffer needs only one call.

Return codes:

| Return   | Meaning                                             |
|----------|-----------------------------------------------------|
| `>= 0`   | Response length in bytes (excluding the NUL).       |
| `-1`     | A required pointer (`handle` or `cmd_json`) is null. |
| `-2`     | `cmd_json` is not valid UTF-8.                       |
| `-3`     | A panic was caught at the boundary.                 |

Domain errors (a bad spec, an unknown command) are **not** negative — they come
back in-band as `{"ok":false,"error":...}` JSON in the buffer.

Because a given seed produces byte-identical output on every platform, the JSON
a caller reads here matches, byte for byte, what every other language binding
and the [`wickra-synth` CLI](https://github.com/wickra-lib/wickra-synth) produce.

### Header generation

`include/wickra_synth.h` is generated with [cbindgen] and committed; CI
fails if it drifts from the source. Regenerate after changing the ABI:

```sh
cbindgen --config cbindgen.toml --crate wickra-synth-c --output include/wickra_synth.h
```

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the C ABI itself, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-synth/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-synth>
- **Docs** (guides, spec reference, cookbook): <https://synth.wickra.org>
- **Runnable example:** [`examples/c/`](https://github.com/wickra-lib/wickra-synth/tree/main/examples/c)

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

[cbindgen]: https://github.com/mozilla/cbindgen
