<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Synth — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed, byte-identical across ten languages" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/ci.svg)](https://github.com/wickra-lib/wickra-synth/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-synth)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/nuget.svg)](https://www.nuget.org/packages/Wickra.Synth)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-synth/license.svg)](https://github.com/wickra-lib/wickra-synth#license)

# Wickra Synth — C#

---

**Deterministic synthetic market microstructure — for C#. `dotnet add package Wickra.Synth` — prebuilt native library, no system dependencies.**

Two projects live here:

| Path | What it is |
|------|------------|
| [`WickraSynth/`](WickraSynth/) | The `Wickra.Synth` package: `Synth`, the P/Invoke surface, and the resolver that finds the native library. Its [README](WickraSynth/README.md) is the one NuGet renders. |
| [`WickraSynth.Tests/`](WickraSynth.Tests/) | xUnit tests, including the golden-corpus check that this binding reproduces `golden/expected` byte for byte. |

This file is the developer view; the package description a consumer reads on
[nuget.org](https://www.nuget.org/packages/Wickra.Synth) is
`WickraSynth/README.md`, packed by the csproj. Keeping the two separate is
deliberate — a registry page has no repository around it, so it cannot link to
sibling directories the way this one does.

## Install

```bash
dotnet add package Wickra.Synth
```

The native library ships prebuilt per platform under `runtimes/<rid>/native/`,
selected automatically. There is nothing to compile. Targets .NET 8 and later.

### Building from this repository (contributors)

The binding calls the C ABI hub, so build that first. The test project copies
the library out of `target/release`, so it must be the release profile:

```bash
cargo build -p wickra-synth-c --release
dotnet test bindings/csharp/WickraSynth.Tests/WickraSynth.Tests.csproj -c Release
```

The tests locate the golden corpus by walking up from the test assembly, so they
run the same from the repository root, from this directory, or from an IDE.

## Quick start

[`examples/csharp/Gen/Program.cs`](https://github.com/wickra-lib/wickra-synth/blob/main/examples/csharp/Gen/Program.cs) is the runnable example the CI smoke job executes; in full:

```csharp
// A runnable C# example: generate synthetic microstructure and print the first
// three candles.
//
//   dotnet run --project examples/csharp/Gen
//
// Every language example uses the same seed and prints the same candles.
using System.Text.Json;
using Wickra.Synth;

const string spec = """
    {"seed":42,"bars":20,"start_price":100.0,
     "regimes":[{"kind":"trend","len":20,"drift":0.002,"vol":0.01}],
     "microstructure":{"book_depth":5,"spread_bps":4.0,"trade_rate":8.0,
        "funding":{"interval_bars":8,"base_rate":0.0001,"sensitivity":0.5}}}
    """;

using var synth = new Synth(spec);
var raw = synth.Command("""{"cmd":"generate"}""");
using var doc = JsonDocument.Parse(raw);
var candles = doc.RootElement.GetProperty("candles");

Console.WriteLine($"wickra-synth {Synth.Version()}");
Console.WriteLine($"bars: {candles.GetArrayLength()}");
Console.WriteLine("first 3 candles:");
for (var i = 0; i < 3 && i < candles.GetArrayLength(); i++)
{
    Console.WriteLine($"  {candles[i].GetRawText()}");
}
```

### How the native library is found

`DllResolver` probes, in order: the `runtimes/<rid>/native/` layout a published
NuGet package ships, then the Cargo `target/` tree above the assembly, then the
default OS search path. The first hit wins, so a repository build and an
installed package both work without an environment variable.

### What is generated and what is not

Nothing here is generated. The whole surface is four C functions, so the binding
is written by hand rather than emitted from the header — which is also why
`scripts/check_binding_surface.py` exists: it holds this file's public members
against the same header every other binding is held against.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of `[LibraryImport]` P/Invoke over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-synth/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-synth>
- **Docs** (guides, spec reference, cookbook): <https://synth.wickra.org>
- **Runnable example:** [`examples/csharp/`](https://github.com/wickra-lib/wickra-synth/tree/main/examples/csharp)

Wickra Synth ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-synth/blob/main/SECURITY.md>.

## Disclaimer

`wickra-synth` generates **synthetic** market data for testing, training and
demonstration. It is not real market data and is not financial advice; it comes
with no warranty.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-synth/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-synth/blob/main/LICENSE-MIT) at your option.
