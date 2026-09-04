# Contributing to wickra-synth

Thanks for your interest. Issues, bug reports, ideas and pull requests are all
welcome at <https://github.com/wickra-lib/wickra-synth>. For larger changes,
open an issue first so we can agree on the approach.

## Orientation

- The core — the `GenSpec` model, the portable PRNG (SplitMix64 → xoshiro256++),
  the price walk and the order-book / trade / funding synthesis — lives in
  `crates/synth-core`. The spec is **data, not code**: a serde structure, so the
  same generation crosses the C ABI and WASM unchanged, and all randomness lives
  in the core so every language sees the same seed produce the same stream.
- The reference consumer is `crates/synth-cli` (the `wickra-synth` binary).
- Every language binding lives under `bindings/<lang>/` and exposes the same
  data-driven surface: a `Synth` handle plus `command(json) -> json` and
  `version`. Bindings must preserve the **golden-parity invariant**: given the
  spec in `golden/specs/`, the same command produces the byte-identical output
  in `golden/expected/`.

## The dev loop

Every change runs green locally before a commit:

```bash
cargo fmt --all
cargo test --workspace --all-features
cargo test --workspace --no-default-features   # the core without the validate oracle
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
```

`cargo fmt --all` and the `clippy -D warnings` gate are enforced in CI on three
operating systems, across both the default and
`--no-default-features` (sequential / WASM) feature sets — a generation must
produce byte-identical output either way.

## Conventions

- **Commits are signed** and follow Conventional Commits (`feat:`, `fix:`,
  `chore:`, `docs:`…). One logical change per commit. Open a PR against `main`;
  do not push to `main` directly.
- **All public artifacts are in English** — code, comments, commit messages, PR
  titles and bodies, issues and docs.
- **No secrets, ever** — not in code, tests, fixtures, logs, issues or PRs. The
  library only generates data; it makes no network calls and needs no
  credentials.
- **Production code only** — no mocks outside `#[cfg(test)]`, no TODO stubs, and
  no defensive branches that can never run (they fail coverage).

## Adding a regime or a microstructure feature

The `GenSpec` is a serde structure, so extending the generator means adding a
field or variant, not a closure. A new regime kind, price-walk parameter or
microstructure knob is added to `crates/synth-core/src/spec.rs` and handled in
the generation path, with a serde round-trip test and a golden fixture. All
randomness must go through the core's portable PRNG so the byte-parity guarantee
holds. See `docs/GENERATION.md`.

## Developer Certificate of Origin

Contributions are accepted under the [DCO](DCO); sign off your commits with
`git commit -s`. By contributing you agree your work is dual-licensed under
`MIT OR Apache-2.0`.
