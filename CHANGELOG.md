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
  depth and trade rate, plus JSON serialization.
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
- `bindings/c/include/wickra_synth.hpp`: an optional header-only C++ hull over
  the C ABI — a move-only RAII `wickra::synth::Synth` that owns the handle and
  runs the two-call length protocol, returning the response unmodified.
- `examples/wasm/gen.mjs`: the WebAssembly example, so all ten reaches have a
  runnable one. The `examples/` table listed nine.
- Golden-corpus parity in every binding. Go, C#, Java, R, WebAssembly and the
  C / C++ examples now replay all of `golden/specs` and assert the response is
  byte-for-byte `golden/expected`. Until now only Python and Node compared
  against the committed corpus; the other six compared one call against a second
  call in the same process, which is true of any pure function and proves
  nothing about parity with the other nine languages. `examples/c` gains
  `golden_test.c` and `golden_test.cpp`, registered as ctest targets, with the
  fixture list derived from the corpus at configure time so a new spec is picked
  up without editing C.

- `scripts/`: five invariant checks nothing in the repository performed.
  `check_version_sync.py` (nineteen version touchpoints across six package
  managers, with exact counts), `check_license_copies.py` (every published
  package carries the texts its SPDX expression names),
  `check_readme_links.py` (a binding README is a registry long description and
  cannot use repository-relative links), `check_binding_surface.py` (every
  binding exposes the whole C ABI, spelled its own way) and
  `check_r_abi_skew.py` (the R wrapper against the ABI its `DESCRIPTION`
  names). Plus `scripts/update-lockfiles.sh`, which regenerates the Cargo, npm
  and hash-pinned Python locks and fetches `uv` only on request, against a
  recorded checksum.
- Five issue templates — detailed bug report, detailed feature request,
  performance regression, documentation, question — and the long-form
  pull-request template, reachable with `?template=detailed.md`.
- `LICENSES/MIT.txt` and `LICENSES/Apache-2.0.txt`, the REUSE-conformant second
  copies, and `docs/README.md`, the signpost that says which pages under
  `docs/` are contracts the tests enforce.
- `bindings/csharp/README.md`, the developer view of that directory (the file
  NuGet renders stays one level down).
- Licence texts beside every published package: `synth-core`, `wickra-synth`,
  and the Python, Node and WebAssembly bindings. The npm manifests list them in
  `files`; wasm-pack had been reporting their absence on every build.
- `README.md`: `Project layout`, `Building everything from source`, `Testing`
  and `Ecosystem`, a runnable command above the first heading, and a
  Requirements table naming all eight declared floors and the manifest each one
  lives in.
- `docs.rs` metadata (`all-features`, `--cfg docsrs`) on both published crates.
  docs.rs builds with nightly and sets `docsrs`; GitHub CI is stable and never
  does, so without this the published reference omits feature-gated items and
  the nightly-only path is exercised nowhere before the release.

- Seven CI jobs the pipeline was missing: `fuzz-smoke` (all four targets, 30 s
  each, list derived from `cargo fuzz list`), `binding-surface` (the five
  invariant scripts), `semver` (`cargo-semver-checks` on `synth-core`),
  `examples-smoke` (every example actually run, in all ten reaches),
  `python-wheel-container-smoke` (manylinux and musllinux), a non-blocking
  `links` copy of the weekly lychee run, and `osv-scan`.
- `.github/workflows/actionlint.yml` — workflow correctness and, through the
  bundled shellcheck, the shell inside every `run:` block. zizmor reads the
  workflows for security; nothing read them for whether they work.
- `.github/workflows/codspeed.yml` — instruction counts per pull request. The
  bench crate's `criterion` is now `codspeed-criterion-compat` under that name;
  without the alias the benches run under the CodSpeed runner and report
  nothing.
- `.github/codeql/codeql-config.yml`, and six more languages in the CodeQL
  matrix. Rust was the only one analysed, so three of the four places a memory
  mistake is possible here — the Go binding's `unsafe.Pointer`, the R glue's
  external-pointer finalizer, the hand-written C++ hull — were unscanned.
- Dependabot entries for the fuzz crate (a detached workspace the root `cargo`
  entry never reached) and the Go example module.

### Changed

- Every workflow job has a `timeout-minutes`. Nineteen had none and inherited
  GitHub's six-hour default, so a job wedged on a hung download held a runner
  for six hours while the pull request read as merely slow.
- The Python CI job installs the hash-pinned
  `.github/requirements/ci-dev.txt` instead of `pip install maturin pytest`. The
  lock existed for the OpenSSF pinned-dependencies check and nothing installed
  it, so the pinning proved nothing.

- The R binding builds without our CI environment. It linked against a header
  and library handed to it through `WKSYNTH_INC` / `WKSYNTH_LIB`, which only our
  own R job sets — so r-universe, which runs `R CMD INSTALL` with no such
  variables, could never have produced a binary. `configure` and `configure.win`
  now download the `wickra-synth-c-<triple>.tar.gz` release asset named by
  `DESCRIPTION: Version` and bundle it beside the package object, with an rpath
  on Unix. `WKSYNTH_INC` / `WKSYNTH_LIB` remain the developer override.
  `DESCRIPTION` gains `Depends: R (>= 4.1)`.
- `bindings/node/package.json` stops listing `npm` in `files`. That directory
  holds the six per-platform manifests npm installs from the registry; shipping
  them inside the main tarball as well served nothing.
- `wickra-core` / `wickra-data` move from `0.9` to `1.0`, the published line.

### Removed

- The `parallel` feature and the rayon dependency. `synth-core` declared
  `default = ["parallel"]` and pulled in rayon, but no code path ever used it:
  generation is sequential by construction — one PRNG stream, one fixed draw
  order — so bar N cannot be produced before bar N-1. The feature compiled rayon
  into every default build, and the bench crate documented a "parallel engine"
  and a "single-threaded path" that were the same code. `synth-core` now
  declares no default features; `validate` is unchanged.
- The unused `rust_decimal` workspace dependency. Prices and quantities are
  `f64` rounded through `round_to`; nothing in the workspace used the crate.

### Fixed

- `examples/go/go.mod` required `.../bindings/go v0.0.0` with no `replace`
  directive — a version no module proxy can resolve, for a module that lives
  in this repository. `go build .` failed with "missing go.sum entry" and
  always had; `examples/README.md` listed it as `go run .` and nothing ran it.
- `.github/requirements/ci-dev.txt` was compiled against Python 3.12 and pinned
  pytest 9, which cannot be installed on the 3.9 row of the matrix.
  Recompiled against 3.9, the oldest interpreter, so one lock covers every row.
- Three action pins carried a version comment that disagreed with the SHA, and
  Dependabot acts on the comment: `Swatinem/rust-cache` was pinned to an
  untagged commit labelled `# v2`, `actions/setup-node` was labelled `v6.4.0` on
  three lines and `v7.0.0` on the same SHA elsewhere, and
  `lycheeverse/lychee-action` was labelled `# v2` for a v2.9.0 commit.
- Three workflow headers described bindings that had long since landed as still
  to come, each citing `P-FS-3` — a phase id belonging to a different
  repository — and `release.yml` described this project's Python binding as
  "pure canonicalization + hashing", which is a different project entirely.
- The first recipe in `docs/Cookbook.md` passed `--regime`, which the CLI does
  not accept — the flag is `--kind`. The documented command exited with
  `unexpected argument`.
- The ecosystem paragraph in `README.md` was pasted twice, the second copy
  truncated to a sentence fragment.
- `BENCHMARKS.md` reported generation times three to five times slower than the
  bench measures. The table is re-measured, and now names the machine.
- Four binding READMEs linked their licences relatively (`../../LICENSE-MIT`).
  Those files are rendered by PyPI, npm, NuGet, pkg.go.dev and r-universe with
  no repository around them, so the links were broken on every page a consumer
  reads.
- `SECURITY.md`'s supported-versions table said `0.1.x` where every other
  version touchpoint says `0.1.0`, so a bump would have left it behind
  silently.

[Unreleased]: https://github.com/wickra-lib/wickra-synth/commits/main
