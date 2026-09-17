# Fuzzing Wickra Synth

One target per **input form**, not per function. The rule is that anything which
takes data from outside gets one, and the question a new target has to answer is
"what shape of input reaches this, and does anything else already cover it?"

## Setup

```bash
cargo install cargo-fuzz
rustup toolchain install nightly-2026-07-01
```

The date is the family's fuzz nightly, pinned in `ci.yml`: a rolling `nightly`
regressed with a codegen ICE unrelated to this code, so every repository moves
the date together, on purpose.

## Targets

| Target | What it exercises |
| --- | --- |
| `spec_toml` | The TOML spec parser. |
| `c_abi_buffer` | The C ABI's length-out buffer protocol. |
| `spec_parse` | The spec-parsing surface: arbitrary bytes are parsed as a `GenSpec` (JSON). |
| `generate` | The generator with a bounded spec derived from arbitrary bytes. |
| `rng_stream` | The PRNG driver: any seed must yield a total, panic-free draw stream — `next_f64` stays in `[0, 1)`, `next_normal` is finite, `next_poisson` never loops forever, and the same seed always produces the same sequence. |
| `command_json` | The `command_json` FFI boundary that every binding calls. |

## Run

```bash
# From the repository root:
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu spec_toml
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu c_abi_buffer
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu spec_parse
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu generate
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu rng_stream
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu command_json
```

Each run continues until a crash is found or it is interrupted. A short
time-boxed smoke run is what CI does:

```bash
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu spec_toml -- -max_total_time=30
```

The expectation for every target is that it never panics: malformed or
adversarial input must surface as an `Err` or an in-band error, never a crash.
