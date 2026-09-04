<!-- Keep it short. One logical change per PR.

     Changing the generated output, the command protocol, or the bindings'
     shared surface? There is a longer template that asks what such a change
     has to answer: reopen this pull request with ?template=detailed.md
     appended to the URL. GitHub offers no picker for a second template, so
     it is only reachable that way. -->

## What

<!-- What does this change and why? -->

## Checklist

- [ ] `cargo fmt --all` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` are clean
- [ ] `cargo test --workspace --all-features` and `--no-default-features` pass
- [ ] `cargo deny check` is clean
- [ ] Tests added/updated (prefer hand-computed expectations for core changes)
- [ ] The spec stays data (a serde `GenSpec`), never Rust closures; all randomness stays in the core PRNG
- [ ] Binding surface mirrored across languages; golden reports regenerated if the schema changed
- [ ] `CHANGELOG.md` updated under `[Unreleased]`
