# The `command_json` protocol

One function is the whole cross-language surface. Every binding — Python,
Node.js, WebAssembly, C, C++, C#, Go, Java, R — forwards a JSON string to
`Synth::command_json` and returns the JSON string that comes back. Nothing else
crosses the boundary, which is why ten languages can share one implementation
and why `golden/expected/*.json` can hold all of them to the same bytes.

That makes this file a contract, not a description. If it and
`crates/synth-core/src/synth.rs` disagree, the code is right and this file is a
bug.

## Shape

A request is an object with a `cmd` string. A response is always an object, and
always valid JSON.

```
{"cmd": "<name>", ...}   ->   { ... }
```

`command_json` never fails. A malformed request, an unknown command, an invalid
spec — all come back as an in-band error object, because a boundary that can
both return a value and fail gives every binding two error paths to get right
instead of one.

## The four commands

### `version`

```json
{"cmd": "version"}
```

```json
{"version": "0.1.1"}
```

The crate version. Takes no spec and never fails.

### `set_spec`

```json
{"cmd": "set_spec", "spec": { ...GenSpec... }}
```

```json
{"ok": true}
```

Stores a spec on the handle. The **only stateful command**: it mutates the
handle, and every later `generate` or `generate_stream` without an inline
`"spec"` uses it. See [GENSPEC.md](GENSPEC.md) for what a valid spec is; an
invalid one is rejected here rather than at generation time.

### `generate`

```json
{"cmd": "generate"}
{"cmd": "generate", "spec": { ...GenSpec... }}
```

```json
{"candles": [...], "book_snapshots": [...], "trades": [...], "funding": [...]}
```

The whole run in one object. With an inline `"spec"` the handle's stored spec is
ignored and left untouched; without one, the stored spec is used and its absence
is an error.

### `generate_stream`

```json
{"cmd": "generate_stream"}
{"cmd": "generate_stream", "spec": { ...GenSpec... }}
```

```json
{"events": [{"type": "trade", "trade": {...}}, {"type": "book", "snapshot": {...}}, ...]}
```

The same data as an ordered event list, drawn in the same order from the same
seed. Spec resolution is identical to `generate`.

**The order is part of the contract.** Per bar: every trade in `seq` order, then
the book snapshot, then the candle, then a funding sample if one is due. See
[DETERMINISM.md](DETERMINISM.md) for why the draw order is fixed, and
`crates/synth-core/tests/stream_eq_batch.rs` for the test that pins it.

## Detecting failure

The envelope is asymmetric, and a consumer has to know how:

| Response | Meaning |
|----------|---------|
| `{"ok": true}` | `set_spec` succeeded |
| `{"ok": false, "error": "..."}` | any command failed; `error` is a human-readable message |
| anything else | success — the payload is the answer |

So the rule for a binding is: **parse, then look for `"ok": false`.** A
successful `generate` has no `ok` field at all, because the response is the data
rather than a wrapper around it.

The error messages are for people. They are not stable identifiers and nothing
should branch on their text.

## What the bindings add

Nothing, by design. A binding constructs a handle, forwards strings, and frees
the handle; the JSON it returns is the JSON the core produced, unmodified. That
is what `golden/` checks in all ten reaches, and what
`scripts/check_binding_surface.py` checks structurally against the C ABI header.

Two consequences worth stating:

- **Field order is significant.** The corpus is compared byte for byte, so a
  binding that re-serializes — even to identical values — breaks parity. This
  has happened once, in the reference CLI, which rebuilt the `generate_stream`
  envelope with a macro that alphabetizes nested keys. `stream_json` exists so
  there is one place that decides.
- **The two-call protocol is per command.** The C ABI measures with
  `out = NULL, cap = 0` and then writes; those two calls belong to one command,
  and issuing a different one in between is refused with
  `WICKRA_SYNTH_ERR_PENDING` rather than silently re-running the first.

## See also

- [GENSPEC.md](GENSPEC.md) — the input
- [DETERMINISM.md](DETERMINISM.md) — the draw order and the PRNG
- [ARCHITECTURE.md](ARCHITECTURE.md) — where the boundary sits
- [`../golden/README.md`](../golden/README.md) — the fixtures that hold it
