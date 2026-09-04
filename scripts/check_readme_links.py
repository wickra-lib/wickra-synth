#!/usr/bin/env python3
"""Binding READMEs must not use repository-relative links.

Each `bindings/*/README.md` is, or is one workflow line away from being, the long
description of a published package: PyPI renders the Python one, npm the Node
and WebAssembly ones, NuGet the C# one, pkg.go.dev the Go one, r-universe the R
one. A link like `../../docs/DETERMINISM.md` resolves on GitHub and nowhere else
-- on a registry page it is simply broken, and nothing in the build says so,
because the file it points at does exist in the repository.

So the rule is: anything that ships as package metadata links absolutely. The
repository's own README and the files under `docs/` are exempt and deliberately
keep relative links -- they are read on GitHub far more than anywhere else.

Which README ships is derived from the manifests, not from the path. A README
beside a manifest is a candidate for that package's long description; one with no
manifest beside it -- `bindings/csharp/README.md`, which sits above the two
csproj projects and points at them -- is a developer's map of the directory and
is read only on GitHub. Judging by the path instead would flag it for links that
are correct where it is actually read.

Run from the repository root:  python scripts/check_readme_links.py
"""

from __future__ import annotations

import glob
import os
import re
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))

# A markdown link or image target. Captured so the offender can be named.
LINK = re.compile(r"!?\[[^\]]*\]\(([^)\s]+)")

# Targets that are fine: absolute URLs, same-page anchors, mail links, and the
# protocol-relative form some badge providers hand out.
ABSOLUTE = re.compile(r"^(?:[a-z][a-z0-9+.-]*:|//|#)", re.IGNORECASE)

# A package manifest, in any of the six ecosystems this repository publishes to.
MANIFESTS = (
    "package.json",
    "pyproject.toml",
    "Cargo.toml",
    "go.mod",
    "pom.xml",
    "DESCRIPTION",
)


def ships(directory: str) -> bool:
    """True if this directory's README can become a package long description."""
    return any(os.path.exists(os.path.join(directory, name)) for name in MANIFESTS)


def main() -> int:
    problems: list[str] = []
    checked = 0

    for path in sorted(glob.glob(os.path.join(ROOT, "bindings", "*", "README.md"))):
        if not ships(os.path.dirname(path)):
            continue
        rel = os.path.relpath(path, ROOT).replace(os.sep, "/")
        checked += 1
        with open(path, encoding="utf-8") as handle:
            for number, line in enumerate(handle, start=1):
                for target in LINK.findall(line):
                    if not ABSOLUTE.match(target):
                        problems.append(f"{rel}:{number}: relative link `{target}`")

    if problems:
        print(f"readme links: {len(problems)} relative link(s) in package metadata\n")
        for problem in problems:
            print(f"  {problem}")
        print(
            "\nA registry page has no repository around it. Link to "
            "https://github.com/wickra-lib/wickra-synth/blob/main/<path> instead."
        )
        return 1

    print(f"readme links: {checked} binding README(s) link absolutely")
    return 0


if __name__ == "__main__":
    sys.exit(main())
