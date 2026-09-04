# Wickra Synth — Java

JVM bindings for the Wickra synthetic-microstructure generator over its C ABI
hub, using the Foreign Function & Memory API (FFM / Panama). A `Synth` is built
from a spec JSON and driven over a JSON boundary, so the result is byte-identical
to every other Wickra Synth binding.

## Requirements

- JDK 22+ (the FFM API is stable since Java 22). Run with
  `--enable-native-access=ALL-UNNAMED`.
- The native C ABI library, built by `cargo build -p wickra-synth-c`.
  The binding loads it from the directory named by the `native.lib.dir` system
  property (the Maven build points it at the workspace `target/debug`).

## Usage

```java
import org.wickra.synth.Synth;

String spec = "{\"seed\":42,\"bars\":20,\"start_price\":100.0,"
    + "\"regimes\":[{\"kind\":\"trend\",\"len\":20,\"drift\":0.002,\"vol\":0.01}],"
    + "\"microstructure\":{\"book_depth\":5,\"spread_bps\":4.0,\"trade_rate\":8.0}}";

try (Synth synth = new Synth(spec)) {
    String response = synth.command("{\"cmd\":\"generate\"}");
    System.out.println(response);
}
```

## Surface

- **`new Synth(String specJson)`** — build a synth from a spec JSON. Throws
  `IllegalArgumentException` if the spec is invalid. Implements `AutoCloseable`;
  use try-with-resources.
- **`String command(String cmdJson)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `set_spec`,
  `generate`, `generate_stream`, `version`.
- **`static String version()`** — the crate version.

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

Dual-licensed under either [MIT](https://github.com/wickra-lib/wickra-synth/blob/main/LICENSE-MIT) or
[Apache-2.0](https://github.com/wickra-lib/wickra-synth/blob/main/LICENSE-APACHE), at your option.
