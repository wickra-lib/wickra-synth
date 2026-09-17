# Wickra Synth examples — C#

Runnable C# examples for the [Wickra Synth C# binding](../../bindings/csharp). The binding consumes the C ABI
library through P/Invoke, so build it once before running anything:

```bash
cargo build -p wickra-synth-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Gen -c Release
```

## The examples

| Example | What it does |
|---------|--------------|
| `Gen/Program.cs` | A runnable C# example: generate synthetic microstructure and print the first three candles. |
