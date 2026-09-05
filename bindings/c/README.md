# Wickra Synth — C ABI

The C ABI is the hub every C-capable language (C, C++, C#, Go, Java, R) links
against. It exposes `wickra-synth-core` as a tiny, JSON-shaped surface built as
both a `cdylib` (dynamic library) and a `staticlib`.

## Surface

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

## Command / response protocol

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

## Header generation

`include/wickra_synth.h` is generated with [cbindgen] and committed; CI
fails if it drifts from the source. Regenerate after changing the ABI:

```sh
cbindgen --config cbindgen.toml --crate wickra-synth-c --output include/wickra_synth.h
```

[cbindgen]: https://github.com/mozilla/cbindgen
