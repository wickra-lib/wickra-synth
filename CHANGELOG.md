# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

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
