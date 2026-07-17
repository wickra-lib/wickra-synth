# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `bindings/c`: the no-hidden-allocation C ABI (`cdylib` + `staticlib`) — the hub
  every C-capable language (C, C++, C#, Go, Java, R) links against. Four
  functions (`wickra_synth_{new,command,free,version}`) expose the
  `command_json` surface through a caller-owned length-out buffer protocol; the
  cbindgen header `include/wickra_synth.h` is committed and drift-checked.
- `wickra-synth` (CLI): the reference `synth-core` consumer. Loads a `GenSpec`
  from a `.json`/`.toml` file or the quick-spec flags, generates the batch or
  streamed output, and prints it as a text summary, JSON (byte-identical to
  `generate`), or CSV (`timestamp,open,high,low,close,volume`, read-back
  verified against the ecosystem CSV reader).
- `synth-core`: the data-driven generation engine. A serde `GenSpec` (regimes,
  microstructure, optional funding) plus a portable seeded PRNG (SplitMix64 →
  xoshiro256++, all randomness in the core) produce OHLCV candles, order-book
  snapshots, trades and funding. Exposed through `generate` (batch),
  `generate_stream` (event list — same draws, same order), and the `Synth`
  handle's single `command_json` boundary. An optional `validate` feature runs
  the wickra-core indicators over the output as a sanity check.
- Repository scaffolding: Cargo workspace, the `synth-core` crate stub,
  supply-chain configuration (`deny.toml`, `osv-scanner.toml`, `lychee.toml`),
  lint configuration (`clippy.toml`), `repo-metadata.toml`, governance docs, the
  `.github` tree (issue/PR templates, `setup-rust`, `sync-metadata.py`,
  dependabot), and dual `MIT OR Apache-2.0` licensing.

[Unreleased]: https://github.com/wickra-lib/wickra-synth/commits/main
