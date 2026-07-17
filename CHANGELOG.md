# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `docs/`: deep-dive documentation — `ARCHITECTURE.md` (workspace layout and the
  core-only-randomness design), `GENSPEC.md` (the input specification),
  `REGIMES.md` (the trend/range/crash/vol price-path formulas),
  `MICROSTRUCTURE.md` (order book, trades and funding), `DETERMINISM.md` (the
  PRNG and the fixed per-bar draw-order contract), and `Cookbook.md` (recipes).
  `BENCHMARKS.md` now carries measured generation-throughput numbers.
- `.github/workflows/`: the full CI suite — `ci.yml` (format/clippy on both
  feature sets, a 3-OS × 2-feature test matrix, MSRV 1.86 + MSRV-node 1.88,
  cargo-deny, a CLI smoke test, and per-binding jobs for the C ABI, Python,
  Node.js, WASM, Go, C#, Java and R with header/index drift checks), plus
  `codeql.yml`, `scorecard.yml`, `zizmor.yml`, `links.yml`, `bench.yml`,
  `sync-metadata.yml` and a USER-GO-gated `release.yml`.
- `examples/`: a runnable "generate a synthetic market" example in every
  language (Rust, Python, Node.js, C, C++, Go, C#, Java, R), each printing the
  same first three candles from seed 42 — a visible cross-language-equality
  proof.
- `synth-core` tests: `conformance` (serde round-trip of every spec/output type;
  unknown/missing fields and unknown regime kinds are rejected), `golden`
  (byte-exact against `golden/expected`), `stream_eq_batch` (the reassembled
  event stream equals the batch output), `rng_vectors` (fixed SplitMix64 /
  xoshiro256++ reference vectors — the reproducibility anchor), and
  `proptest_invariants` (random specs stay finite and well-formed; same seed →
  identical output).
- `fuzz/`: cargo-fuzz targets `spec_parse`, `generate` (bounded to avoid OOM),
  `rng_stream`, and `command_json` — the parse/generate/PRNG/FFI surfaces must
  never panic on arbitrary input.
- `synth-bench`: Criterion benchmarks for `generate` scaling by bar count, book
  depth and trade rate, on the parallel and single-threaded engines.
- `GenSpec`/`Regime`/`Microstructure`/`FundingSpec` now reject unknown fields
  (`deny_unknown_fields`) so a typo'd spec is an error, not silently ignored.
- `golden/`: the cross-language golden corpus — five `specs/*.json` (trend,
  range, crash, vol, mixed) and their byte-exact `expected/*.json` `GenOutput`
  fixtures, blessed from `synth-core::generate`. Every language binding replays
  the specs and must reproduce the expected output byte-for-byte. No `data/`
  directory: the seed is the complete input.
- `bindings/r`: R bindings (`wickrasynth`) over the C ABI hub via `.Call`, with
  an external-pointer handle freed by a finalizer, the header/library provided
  out-of-tree through `WKSYNTH_INC`/`WKSYNTH_LIB`, and a plain-R test script
  covering generate, determinism, stream-vs-batch candle equality and the
  in-band error path.
- `bindings/java`: JVM bindings (`org.wickra.synth.Synth`) over the C ABI hub via
  the Foreign Function & Memory API (FFM/Panama), with an `AutoCloseable` handle
  and JUnit 5 tests covering generate, determinism, stream-vs-batch candle
  equality and the in-band error path.
- `bindings/csharp`: .NET bindings (`Wickra.Synth`) over the C ABI hub via
  `[LibraryImport]` P/Invoke, with a `SafeHandle`, a `DllImportResolver` that
  probes the packaged/dev/CI layouts, and xUnit tests covering generate,
  determinism, stream-vs-batch and the in-band error path.
- `bindings/go`: cgo bindings over the C ABI hub exposing a `Synth` type
  (`New` / `Command` / `Close` / `Version`), with the header vendored under
  `include/` (drift-checked) and the prebuilt library staged per platform under
  `lib/<goos>_<goarch>/`; tests cover generate, determinism, stream-vs-batch and
  the in-band error path.
- `bindings/wasm`: wasm-bindgen bindings exposing a `Synth` class (`command` /
  `version`) plus a module-level `version()` over the same `command_json`
  surface; a direct `default-features = false` core dep keeps the browser build
  byte-identical to native, verified by a wasm-pack golden test.
- `bindings/node`: napi-rs bindings exposing a `Synth` class (`command` /
  `version`) over the same `command_json` surface, with generated `index.js` /
  `index.d.ts`, per-platform npm sub-packages, and node:test
  smoke/completeness/golden tests.
- `bindings/python`: PyO3/maturin bindings (`abi3-py39`) exposing a `Synth`
  class with `command` / `version` over the same `command_json` surface, with
  type stubs, `py.typed`, and smoke/completeness/golden tests.
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
