# Wickra Synth — .NET binding

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

## Build and test locally

The binding calls the C ABI hub, so build that first. The test project copies
the library out of `target/release`, so it must be the release profile:

```bash
cargo build -p wickra-synth-c --release
dotnet test bindings/csharp/WickraSynth.Tests/WickraSynth.Tests.csproj -c Release
```

The tests locate the golden corpus by walking up from the test assembly, so they
run the same from the repository root, from this directory, or from an IDE.

## How the native library is found

`DllResolver` probes, in order: the `runtimes/<rid>/native/` layout a published
NuGet package ships, then the Cargo `target/` tree above the assembly, then the
default OS search path. The first hit wins, so a repository build and an
installed package both work without an environment variable.

## What is generated and what is not

Nothing here is generated. The whole surface is four C functions, so the binding
is written by hand rather than emitted from the header — which is also why
`scripts/check_binding_surface.py` exists: it holds this file's public members
against the same header every other binding is held against.
