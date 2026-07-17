# Wickra Synth — C#

.NET bindings for the Wickra synthetic-microstructure generator over its C ABI
hub. A `Synth` is built from a spec JSON and driven over a JSON boundary, so the
result is byte-identical to every other Wickra Synth binding.

## Install

```bash
dotnet add package Wickra.Synth
```

The package ships the native C ABI library per runtime identifier under
`runtimes/<rid>/native/`. For a local build, `cargo build -p wickra-synth-c --release`
places the library in `target/release/`; the bundled `DllImportResolver` probes
the Cargo `target/` tree, so tests and apps in the repo find it without extra
steps.

## Usage

```csharp
using Wickra.Synth;

const string spec = """
{"seed":42,"bars":20,"start_price":100.0,
 "regimes":[{"kind":"trend","len":20,"drift":0.002,"vol":0.01}],
 "microstructure":{"book_depth":5,"spread_bps":4.0,"trade_rate":8.0}}
""";

using var synth = new Synth(spec);

string response = synth.Command("""{"cmd":"generate"}""");
Console.WriteLine(response);
```

## Surface

- **`new Synth(string specJson)`** — build a synth from a spec JSON. Throws
  `ArgumentException` if the spec is invalid. Implements `IDisposable`; dispose
  it when done.
- **`string Command(string cmdJson)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `set_spec`,
  `generate`, `generate_stream`, `version`.
- **`static string Version()`** — the crate version.

Domain errors (a bad command, an unknown command name) come back as an
`{"ok": false, "error": ...}` response, not as an exception. Exceptions are
reserved for an invalid spec at construction and hard failures at the C ABI
boundary.

## Determinism

The response bytes are identical across languages for a given seed, because the
whole generator lives once in the Rust core and this binding forwards its JSON
verbatim.

## See also

- The main project: <https://github.com/wickra-lib/wickra-synth>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../../LICENSE-MIT) or
[Apache-2.0](../../../LICENSE-APACHE), at your option.
