# Wickra Synth examples — Java

Runnable Java examples for the [Wickra Synth Java binding](../../bindings/java). The binding reaches the C ABI
through the Foreign Function & Memory API (JDK 22+), so build the library once
and point the JVM at it with `-Dnative.lib.dir`:

```bash
cargo build -p wickra-synth-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
mvn -B -q -f bindings/java package -DskipTests
javac -cp bindings/java/target/classes examples/java/Gen.java -d examples/java/out
java --enable-native-access=ALL-UNNAMED  -Dnative.lib.dir=target/debug  -cp "bindings/java/target/classes:examples/java/out" Gen
```

## The examples

| Example | What it does |
|---------|--------------|
| `Gen.java` | A runnable Java example: generate synthetic microstructure through the binding. |
