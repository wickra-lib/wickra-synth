#!/usr/bin/env python3
"""Assert that every place carrying the release version agrees.

The version lives in a dozen files across six package managers, and a bump that
misses one produces a release where, say, the npm package pins a native binary
that was never published. That failure surfaces at install time, on a user's
machine, after the tag is irreversible -- so it is worth a cheap check before the
tag rather than a patch release after it.

    python scripts/check_version_sync.py                  # all files agree
    python scripts/check_version_sync.py --previous 0.1.0 # and none is stale

The file list is explicit rather than a repository-wide grep on purpose:
`Cargo.lock` records third-party crates that will occasionally sit at the same
version as this project, and a grep that matched those would either be noisy or
be silenced with exceptions that outlive their reason.

Counts are exact, not "at least one". A pattern that should match six platform
dependencies and finds five has found the bug this file exists for.
"""

from __future__ import annotations

import argparse
import os
import re
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))

NPM_PLATFORMS = [
    "darwin-arm64",
    "darwin-x64",
    "linux-arm64-gnu",
    "linux-x64-gnu",
    "win32-arm64-msvc",
    "win32-x64-msvc",
]

# (path, regex with a {v} placeholder for the version, expected match count).
# The regex is built per version so a check reads as "this exact string, here,
# this many times".
TOUCHPOINTS: list[tuple[str, str, int]] = [
    # The workspace version, and the path dependency that pins it.
    ("Cargo.toml", r'(?m)^version = "{v}"$', 1),
    ("Cargo.toml", r'wickra-synth-core = \{{ version = "{v}"', 1),
    # Python: maturin reads the version from pyproject, not from Cargo.toml.
    ("bindings/python/pyproject.toml", r'(?m)^version = "{v}"$', 1),
    # Node: the package itself plus one optional dependency per platform.
    ("bindings/node/package.json", r'"version": "{v}"', 1),
    ("bindings/node/package.json", r'"wickra-synth-[a-z0-9-]+": "{v}"', 6),
    # The lockfile records the same numbers; npm ci fails loudly when it drifts.
    ("bindings/node/package-lock.json", r'"version": "{v}"', 2),
    ("bindings/node/package-lock.json", r'"wickra-synth-[a-z0-9-]+": "{v}"', 6),
    # JVM and .NET.
    ("bindings/java/pom.xml", r"<version>{v}</version>", 1),
    ("bindings/csharp/WickraSynth/WickraSynth.csproj", r"<Version>{v}</Version>", 1),
    # R.
    ("bindings/r/DESCRIPTION", r"(?m)^Version: {v}$", 1),
    # The example's own version. It depends on the binding by path
    # (file:../../bindings/node), not by range, so there is no dependency
    # version here to keep in step -- and there must not be: a range would send
    # `npm install` to the registry for a package that does not exist until the
    # first release, which is exactly how that example came to be unrunnable.
    ("examples/node/package.json", r'"version": "{v}"', 1),
    # The supported-versions table.
    ("SECURITY.md", r"{v}", 1),
]

# One per platform stub package.
TOUCHPOINTS += [
    (f"bindings/node/npm/{platform}/package.json", r'"version": "{v}"', 1)
    for platform in NPM_PLATFORMS
]

# Deliberately not touchpoints, so nobody adds them later after a fruitless grep:
#   examples/csharp/Gen/Gen.csproj  -- <ProjectReference>, carries no version
#   examples/go/go.mod              -- replace directive, carries no version
#   CITATION.cff                    -- no version field until the first release
#   Cargo.lock                      -- refreshed by `cargo build`, never by hand


def workspace_version() -> str:
    text = read("Cargo.toml")
    match = re.search(r'(?m)^\[workspace\.package\]\n(?:.*\n)*?version = "([^"]+)"', text)
    if not match:
        sys.exit("Cargo.toml: no [workspace.package] version")
    return match.group(1)


def read(rel: str) -> str:
    path = os.path.join(ROOT, rel)
    if not os.path.exists(path):
        sys.exit(f"missing file: {rel}")
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--previous",
        help="a version that must no longer appear anywhere in the touchpoints",
    )
    args = parser.parse_args()

    version = workspace_version()
    escaped = re.escape(version)
    problems: list[str] = []

    for rel, pattern, expected in TOUCHPOINTS:
        text = read(rel)
        found = len(re.findall(pattern.format(v=escaped), text))
        if found != expected:
            problems.append(
                f"{rel}: expected {expected} occurrence(s) of {version} "
                f"matching /{pattern.format(v=escaped)}/, found {found}"
            )

    if args.previous:
        stale = re.escape(args.previous)
        for rel, pattern, _ in TOUCHPOINTS:
            text = read(rel)
            found = len(re.findall(pattern.format(v=stale), text))
            if found:
                problems.append(f"{rel}: still carries the previous version {args.previous}")

    if problems:
        print(f"version sync: {len(problems)} problem(s) against {version}\n")
        for problem in problems:
            print(f"  {problem}")
        return 1

    print(f"version sync: all {len(TOUCHPOINTS)} touchpoints agree on {version}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
