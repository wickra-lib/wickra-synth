# Wickra Synth — C / C++ examples

The Wickra Synth C ABI is a single shared/static library plus a generated header
([`bindings/c/include/wickra_synth.h`](../../bindings/c/include/wickra_synth.h)). Any C-capable
language links against the same artifact; these examples show the plain-C path
and, through [`wickra_synth.hpp`](../../bindings/c/include/wickra_synth.hpp), the C++ one.

## Build the library

From the workspace root:

```sh
cargo build -p wickra-synth-c --release
```

This produces, in `target/release/`:

| Platform | Shared library | Link target |
|----------|----------------|-------------|
| Linux    | `libwickra_synth.so`     | `-lwickra_synth` |
| macOS    | `libwickra_synth.dylib`  | `-lwickra_synth` |
| Windows (MSVC) | `wickra_synth.dll` | `wickra_synth.dll.lib` (import lib) |

A static library (`libwickra_synth.a` / `wickra_synth.lib`) is emitted alongside.

## Build and run the examples

With CMake, as the CI C ABI job does:

```sh
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

## The examples

| Example | What it does |
|---------|--------------|
| `gen.c` | A runnable C example: generate synthetic microstructure through the |
| `gen.cpp` | A runnable C++ example: generate synthetic microstructure through the wickra-synth C ABI. |

## Usage shape

Every call follows the same handle discipline: construct from a spec JSON, drive
with command JSON, read the response, free the handle exactly once. `wickra_synth.h` is
the whole contract; the C++ header, where one ships, wraps the handle in a
move-only RAII type. See [`bindings/c/README.md`](../../bindings/c/README.md).
