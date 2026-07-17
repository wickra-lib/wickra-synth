# Architecture

Wickra Synth is one deterministic core with a thin, uniform binding surface. The
whole project is organized around a single rule: **all randomness lives in the
Rust core and nowhere else.** Every language binding forwards a JSON command to
that core and returns its response verbatim, so a given seed produces the
byte-for-byte identical market on every platform and in every language.

## Workspace layout

| Crate / dir | Role |
|-------------|------|
| `crates/synth-core` | The engine: `GenSpec`, the portable PRNG, the price walk, the microstructure model, and the `command_json` data API. No I/O, no networking. |
| `crates/synth-cli` | The reference CLI (`wickra-synth`): quick-spec flags or a `--spec` file in, text or JSON out. |
| `crates/synth-bench` | Criterion benchmarks over the core. |
| `bindings/c` | The C ABI hub (`wickra_synth_{new,command,free,version}`) plus the committed header. |
| `bindings/{python,node,wasm}` | Native bindings (PyO3, napi, wasm-bindgen), each a workspace member. |
| `bindings/{csharp,go,java,r}` | C-ABI languages that link the built `wickra_synth` library. |
| `golden/` | Blessed cross-language corpus: `specs/*.json` + byte-exact `expected/*.json`. |
| `fuzz/` | A detached cargo-fuzz workspace targeting the spec parser, generator, PRNG and command API. |

## Data flow

```
GenSpec (JSON) ──▶ Synth::new ──▶ command("generate") ──▶ GenOutput (JSON)
                     │                                        candles
                     └── DetRng (SplitMix64 → xoshiro256++)   book snapshots
                         portable, fixed 64-bit arithmetic    trades
                                                              funding samples
```

The core exposes exactly one behavioural surface, `Synth::command_json`, which
never returns `Err`: unknown commands, malformed JSON and invalid specs all come
back as an in-band `{"ok":false,"error":…}` object. Only the constructor
`Synth::new(spec_json)` rejects a non-empty invalid spec. Every binding wraps
these two entry points and nothing else, which is why the bindings stay trivial
and cannot drift from the core's behaviour.

## Determinism boundary

The PRNG (`rng.rs`) is the moat. It is a `SplitMix64` seed expander feeding an
`xoshiro256++` stream, written with explicit wrapping 64-bit arithmetic so the
bit sequence is identical on every target. No binding, and no other module,
ever draws its own randomness. The exact order in which the core consumes draws
is a hard contract — see [DETERMINISM.md](DETERMINISM.md).

## See also

- [GENSPEC.md](GENSPEC.md) — the input specification.
- [REGIMES.md](REGIMES.md) — the price-path formulas.
- [MICROSTRUCTURE.md](MICROSTRUCTURE.md) — book, trades and funding.
- [DETERMINISM.md](DETERMINISM.md) — the PRNG and draw-order contract.
