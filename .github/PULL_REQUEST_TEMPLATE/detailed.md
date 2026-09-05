<!--
The long form. Use it by appending ?template=detailed.md to the pull-request
URL. Reach for it when the change touches generated data, the command protocol,
or the binding surface -- the three things this repository promises are the same
in ten languages. The default template is right for everything else.
-->

## What

<!-- The change in a sentence, then the reasoning underneath it. -->

## Why

<!-- What was wrong, or what could not be expressed before. -->

## Does the generated output move?

<!--
Answer one of:

  * No. `cargo test -p wickra-synth-core --test golden` passes untouched.
  * Yes, and `golden/expected/*.json` is re-blessed in this PR.

If yes, say which fixtures moved and why the new bytes are the right ones. A
re-bless is a change to the contract every binding is held to, so it belongs in
its own commit with the reasoning in the message, not folded into a refactor.
-->

## Which reaches are touched?

<!--
Tick what this PR changes. Anything crossing `command_json` reaches all ten.
-->

- [ ] `wickra-synth-core`
- [ ] CLI (`wickra-synth`)
- [ ] C ABI (`bindings/c`, header + hull)
- [ ] Python
- [ ] Node.js
- [ ] WebAssembly
- [ ] C#
- [ ] Go
- [ ] Java
- [ ] R
- [ ] Examples
- [ ] Documentation only

## Determinism

- [ ] No new source of randomness outside `wickra-synth-core::rng`
- [ ] The per-bar draw order is unchanged, or `docs/DETERMINISM.md` is updated with it
- [ ] The spec stays data (a serde `GenSpec`), never Rust closures

## Verification

<!--
Paste what you ran, not what you intend to run. Per-binding commands are in
CONTRIBUTING.md.
-->

```text
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo test --workspace --all-features
cargo deny check
```

- Bindings exercised locally:
- Bindings left to CI:

## Performance

<!--
Only if this touches the generation path. Medians from
`cargo bench -p synth-bench`, before and after, on the same idle machine. Skip
the section otherwise -- an empty table is worse than no table.
-->

## Checklist

- [ ] One logical change per commit
- [ ] `CHANGELOG.md` updated under `[Unreleased]`
- [ ] Public items documented (rustdoc, stubs, `index.d.ts`, XML docs as applicable)
- [ ] Generated files regenerated and committed (`bindings/c/include/wickra_synth.h`, `bindings/node/index.js`, `index.d.ts`)
- [ ] No absolute local paths, no machine names, no attribution trailers
