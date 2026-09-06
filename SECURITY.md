# Security Policy

`wickra-synth` generates synthetic market data from a data-driven spec. It holds
no secret material, opens no network connections, and places no orders. The
attack surface is therefore narrow — principally the parsing of an untrusted
`GenSpec` as it crosses the C ABI and WASM boundary. Note also that the built-in
PRNG is a fast **non-cryptographic** generator for reproducible simulation and
must never be used for any security purpose. See
[THREAT_MODEL.md](THREAT_MODEL.md) for the asset inventory and trust boundaries.

## Supported versions

This project is pre-release. Security fixes target the `main` branch and the most
recent published version once a release exists.

| Version | Supported |
|---------|-----------|
| `main`  | ✅        |
| `0.1.1` (upcoming) | ✅ |

## Reporting a vulnerability

**Please do not open a public issue, pull request or discussion for security
problems.** Report privately through either channel:

- GitHub → the repository's **Security** tab → **Report a vulnerability**
  (private advisory), or
- email **support@wickra.org**.

Include a description, affected version/commit, reproduction steps and impact.

We aim to acknowledge within a few days, agree a disclosure timeline, and credit
reporters who wish to be named once a fix ships.

## Scope

In scope: memory-safety or panic-across-FFI flaws in the C ABI hub and its
buffer protocol; a `GenSpec` that passes `GenSpec::validate` and then panics,
wraps, or produces a non-deterministic or malformed result; and any input that
makes one binding disagree with another for the same seed.

Out of scope: incorrect generation mathematics (a functional bug, not a
vulnerability); advisories in third-party crates that are already tracked and
triaged in [`osv-scanner.toml`](osv-scanner.toml); and the memory a spec asks
for. That last one is worth stating plainly rather than leaving as an implied
promise: `bars` and `book_depth` are the caller's own numbers, and generating a
billion bars allocates a billion bars. The library rejects a spec that cannot
work — a timeline that overflows `i64`, a zero depth, a non-finite parameter —
and it does not second-guess a spec that merely asks for a lot. A process that
accepts specs from an untrusted source has to bound them itself, the same way it
would bound any other size it was handed.

## Vulnerability disclosure (VEX)

This repository ships a machine-readable VEX record in
[`osv-scanner.toml`](osv-scanner.toml), kept in lock-step with the cargo-deny
advisory ignore list in [`deny.toml`](deny.toml). Any advisory assessed as not
affecting `wickra-synth` is documented there with a reason, so downstream
scanners see an explicit, auditable justification rather than an unexplained
suppression.
