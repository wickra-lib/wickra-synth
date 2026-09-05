# Fuzz targets

One target per **input form**, not per function. The rule is that anything which
takes data from outside gets one, and the question a new target has to answer is
"what shape of input reaches this, and does anything else already cover it?"

| Target | Input form | What it holds |
|--------|-----------|---------------|
| `spec_parse` | arbitrary bytes as JSON | `GenSpec::from_json` parses or rejects; never panics. |
| `spec_toml` | arbitrary bytes as TOML | the same for `from_toml`, a second grammar the CLI reaches by file extension. |
| `generate` | a bounded spec built from bytes | generation never panics and produces finite, well-formed output — with the timeline drawn full-width, because `start_ts + (bars - 1) * bar_secs` is where the walk ran off the end of `i64`. |
| `rng_stream` | arbitrary seeds and draw counts | the PRNG produces the same stream for the same seed, and every draw is in range. |
| `command_json` | arbitrary bytes as a command | the JSON boundary answers every input with valid JSON, including nonsense. |
| `c_abi_buffer` | arbitrary command bytes plus a chosen capacity | the C ABI's length-out protocol never reads or writes outside the caller's buffer, and leaves it untouched when the capacity is too small. |

`c_abi_buffer` is the one that matters most and was the last to exist. The C ABI
is the only `unsafe` in the workspace, `SECURITY.md` names its buffer protocol as
in scope, and the other five targets all point at safe Rust in the core. It links
`wickra-synth-c` as an `rlib` — the crate declares that type solely so this
target can exist, since neither `cdylib` nor `staticlib` can be depended on from
Rust.

## Running them

cargo-fuzz needs nightly (libfuzzer-sys uses sanitizer flags):

```bash
cargo install cargo-fuzz
cargo +nightly fuzz build            # all targets, which is what catches API drift
cargo +nightly fuzz list             # the names, derived rather than remembered
cargo +nightly fuzz run generate     # one target, until you stop it
cargo +nightly fuzz run generate -- -max_total_time=30
```

CI runs every target for 30 seconds on each push (`ci.yml`, job `fuzz-smoke`),
with the list taken from `cargo fuzz list` rather than written out — a new target
is fuzzed the moment it exists rather than the moment someone remembers to add a
line. That is a smoke test, not a campaign: it catches a target that stopped
compiling or started panicking, and it will not find a novel bug. Real campaigns
belong on dedicated infrastructure with a persistent corpus.

## Why this is a detached workspace

`fuzz/Cargo.toml` declares its own `[workspace]`, so cargo-fuzz can build it on
nightly with sanitizer flags without those flags reaching the rest of the tree.
The cost is that the root `cargo` Dependabot entry does not see it, which is why
`.github/dependabot.yml` carries a second entry for `/fuzz` — and why
`Cargo.lock` here is committed rather than ignored. A detached workspace without
a lockfile resolves differently on every machine, and an unwatched lockfile only
moves when somebody notices.
