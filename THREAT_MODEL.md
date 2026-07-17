# Threat model

Wickra Synth is a library that **generates** synthetic market data from a
data-driven spec. It has no network boundary, handles no credentials, and stores
no user data, so its threat surface is narrow. This document records what it is
and — importantly — what it must not be used for.

## Assets

- The reproducibility guarantee: a given `GenSpec` + seed must always produce the
  identical stream (byte-for-byte, across platforms and languages).
- The integrity of the generated JSON shapes (so downstream tools can rely on
  them).

## Actors

- **Library users** — call `command_json` / the CLI to generate data. Trusted to
  supply their own specs.
- **Contributors** — change the engine; gated by CI (fmt, clippy, tests, golden
  fixtures, cargo-deny, CodeQL, Scorecard).

## Threats and mitigations

- **Non-deterministic output.** The single biggest risk to the product's value.
  Mitigated by keeping all randomness in one portable PRNG inside the Rust core
  (no binding draws its own randomness) and pinning cross-platform,
  cross-language output with golden fixtures.
- **Supply-chain.** Dependencies are vetted by `cargo-deny` (licenses, bans,
  crates.io-only sources — no git dependencies), OSV scanning, and hash-pinned
  CI tool installs. There are no runtime network calls.
- **Malformed spec input.** The `command_json` boundary returns a structured
  error rather than panicking; fuzz targets exercise the decode path.

## Explicit non-goal: the PRNG is not cryptographic

The generator uses SplitMix64 and xoshiro256++ — fast, well-distributed,
**non-cryptographic** PRNGs chosen for reproducibility. They are **not** suitable
for keys, tokens, nonces, or any security or cryptographic purpose. Do not use
`wickra-synth`'s randomness where unpredictability matters for security.
