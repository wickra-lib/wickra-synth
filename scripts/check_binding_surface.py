#!/usr/bin/env python3
"""Assert that every binding exposes the whole C ABI surface, spelled its own way.

The C ABI header is the contract: four functions, `wickra_synth_{new, command,
free, version}`. Everything except the Rust core is a consumer of it, and each
consumer is written by hand with its own test suite -- so a capability that goes
missing in one of them fails nowhere. Nothing compares the bindings *to each
other*.

That is not hypothetical for this shape of repository. A binding that forgets to
expose the free/close half of the pair leaks a handle per construction and every
test still passes, because a leak is invisible to an assertion on the returned
JSON.

Two rules keep this honest:

  * The expected set is derived from the header, not hard-coded. Add a fifth C
    function and every binding starts owing a spelling of it.
  * A binding may expose more than the contract. Go has `Close` because Go has
    no destructors; Python and Node free through the language's own object
    lifetime and correctly have no `free`. That is an idiom difference, not
    drift, so the mapping below says per language which capabilities are
    *owed* -- and a binding that ships none of them is the finding.

Run from the repository root:  python scripts/check_binding_surface.py
"""

from __future__ import annotations

import os
import re
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
HEADER = "bindings/c/include/wickra_synth.h"

# Each capability maps to the pattern that proves a binding exposes it, per
# language. `None` means the language does not owe this capability: construction
# and destruction are the object's own lifetime there, not a named method.
#
# The patterns match the binding's *public* surface file, so an internal helper
# with the right name does not satisfy the contract.
SURFACE: dict[str, dict[str, str | None]] = {
    "python": {
        "file": "bindings/python/python/wickra_synth/__init__.pyi",
        "new": r"class Synth",
        "command": r"def command",
        "free": None,  # refcounted; PyO3 drops the Rust value with the object
        "version": r"def version",
    },
    "node": {
        "file": "bindings/node/index.d.ts",
        "new": r"constructor\(specJson",
        "command": r"command\(cmdJson",
        "free": None,  # the napi object owns the handle
        "version": r"version\(\)",
    },
    "wasm": {
        "file": "bindings/wasm/src/lib.rs",
        "new": r"pub fn new",
        "command": r"pub fn command",
        "free": None,  # wasm-bindgen generates free() into the JS glue
        "version": r"pub fn version",
    },
    "csharp": {
        "file": "bindings/csharp/WickraSynth/Synth.cs",
        "new": r"public Synth\(",
        "command": r"public string Command\(",
        "free": r"SafeHandle|Dispose",
        "version": r"Version",
    },
    "go": {
        "file": "bindings/go/wickra.go",
        "new": r"func New\(",
        "command": r"func \(\w+ \*Synth\) Command\(",
        "free": r"func \(\w+ \*Synth\) Close\(",
        "version": r"func Version\(",
    },
    "java": {
        "file": "bindings/java/src/main/java/org/wickra/synth/Synth.java",
        "new": r"public Synth\(",
        "command": r"public String command\(",
        "free": r"public void close\(",
        "version": r"version\(",
    },
    # C++ is the tenth reach and travels inside the C binding as a header-only
    # hull rather than its own directory, which is why it was missed. It is
    # hand-written, so a dropped method is exactly the failure this file exists
    # for: examples/c/golden_test.cpp would catch a behavioural regression, and
    # nothing would catch a removed `version()`.
    "cpp": {
        "file": "bindings/c/include/wickra_synth.hpp",
        "new": r"explicit Synth\(",
        "command": r"std::string command\(",
        "free": r"~Synth\(\)",
        "version": r"static std::string version\(",
    },
    "r": {
        "file": "bindings/r/NAMESPACE",
        "new": r"export\(wksynth_new\)",
        "command": r"export\(wksynth_command\)",
        "free": None,  # an external pointer with a registered finalizer
        "version": r"export\(wksynth_version\)",
    },
}

# How a header symbol maps onto a capability name above.
CAPABILITIES = {
    "wickra_synth_new": "new",
    "wickra_synth_command": "command",
    "wickra_synth_free": "free",
    "wickra_synth_version": "version",
}


def read(rel: str) -> str:
    path = os.path.join(ROOT, rel)
    if not os.path.exists(path):
        sys.exit(f"missing file: {rel}")
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def header_symbols() -> list[str]:
    """The exported functions, read out of the committed cbindgen header."""
    text = read(HEADER)
    return sorted(set(re.findall(r"\b(wickra_synth_[a-z_]+)\s*\(", text)))


def main() -> int:
    symbols = header_symbols()
    unknown = [symbol for symbol in symbols if symbol not in CAPABILITIES]
    if unknown:
        print("binding surface: the C ABI grew a function this script does not know\n")
        for symbol in unknown:
            print(f"  {symbol}")
        print("\nAdd it to CAPABILITIES and say how each language spells it.")
        return 1

    missing_from_header = [symbol for symbol in CAPABILITIES if symbol not in symbols]
    if missing_from_header:
        print("binding surface: the header no longer declares\n")
        for symbol in missing_from_header:
            print(f"  {symbol}")
        return 1

    problems: list[str] = []
    for binding, spec in sorted(SURFACE.items()):
        text = read(str(spec["file"]))
        for symbol in symbols:
            capability = CAPABILITIES[symbol]
            pattern = spec[capability]
            if pattern is None:
                continue
            if not re.search(pattern, text):
                problems.append(
                    f"{binding}: {spec['file']} exposes no `{capability}` "
                    f"(the C ABI declares {symbol})"
                )

    if problems:
        print(f"binding surface: {len(problems)} gap(s)\n")
        for problem in problems:
            print(f"  {problem}")
        return 1

    print(
        f"binding surface: {len(SURFACE)} bindings expose all "
        f"{len(symbols)} C ABI capabilities"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
