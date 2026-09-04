#!/usr/bin/env python3
"""Every published package must carry the licence texts it claims.

The repository is dual-licensed and every manifest says `MIT OR Apache-2.0`, but
an SPDX expression is a reference to two documents, not the documents. A package
that ships the expression alone leaves whoever received it with terms they have
to go and find.

Cargo decides what to package from git, so an untracked copy makes
`cargo publish` refuse the dirty tree and a gitignored copy is dropped from the
.crate entirely. Committed copies are the only thing that works, and the cost of
a committed copy is drift -- which is what this checks.

The npm platform stubs are handled at publish time (release.yml stages the texts
into each package directory) because npm packs whatever is in the tree moments
beforehand; this script checks the manifests list them, so a staged file is not
silently dropped from the tarball.

Locations are derived, not listed: every workspace member without
`publish = false`, plus the two bindings whose artefacts are built from their own
directory. Add a publishable crate and this starts requiring its licences
without anyone remembering to edit the list.

Run from the repository root:  python scripts/check_license_copies.py
"""

from __future__ import annotations

import json
import os
import re
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
NAMES = ("LICENSE-MIT", "LICENSE-APACHE")


def read(path: str) -> str:
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def workspace_members() -> list[str]:
    text = read(os.path.join(ROOT, "Cargo.toml"))
    block = re.search(r"(?ms)^members = \[(.*?)\]", text)
    if not block:
        sys.exit("Cargo.toml: no workspace members list")
    return re.findall(r'"([^"]+)"', block.group(1))


def publishes(manifest_dir: str) -> bool:
    """A crate publishes unless it opts out with `publish = false`."""
    manifest = os.path.join(ROOT, manifest_dir, "Cargo.toml")
    if not os.path.exists(manifest):
        return False
    return not re.search(r"(?m)^publish\s*=\s*false", read(manifest))


def expected_dirs() -> list[str]:
    dirs = [member for member in workspace_members() if publishes(member)]
    # maturin builds the wheel and the sdist from this directory, and wasm-pack
    # the npm package from its own; neither goes to crates.io, so neither is
    # caught by the publish check above.
    for extra in ("bindings/python", "bindings/wasm"):
        if extra not in dirs:
            dirs.append(extra)
    return sorted(dirs)


def npm_manifests() -> list[str]:
    manifests = ["bindings/node/package.json"]
    npm_dir = os.path.join(ROOT, "bindings", "node", "npm")
    if os.path.isdir(npm_dir):
        for entry in sorted(os.listdir(npm_dir)):
            manifest = os.path.join("bindings", "node", "npm", entry, "package.json")
            if os.path.exists(os.path.join(ROOT, manifest)):
                manifests.append(manifest.replace(os.sep, "/"))
    return manifests


def main() -> int:
    problems: list[str] = []

    canonical = {name: read(os.path.join(ROOT, name)) for name in NAMES}

    for directory in expected_dirs():
        for name in NAMES:
            path = os.path.join(ROOT, directory, name)
            if not os.path.exists(path):
                problems.append(f"{directory}/{name}: missing")
            elif read(path) != canonical[name]:
                problems.append(f"{directory}/{name}: differs from the root copy")

    for manifest in npm_manifests():
        listed = json.loads(read(os.path.join(ROOT, manifest))).get("files")
        if listed is None:
            # No `files` key means npm packs everything, licences included.
            continue
        missing = [name for name in NAMES if name not in listed]
        if missing:
            problems.append(f"{manifest}: `files` does not list {', '.join(missing)}")

    if problems:
        print(f"licence copies: {len(problems)} problem(s)\n")
        for problem in problems:
            print(f"  {problem}")
        return 1

    checked = len(expected_dirs()) * len(NAMES) + len(npm_manifests())
    print(f"licence copies: {checked} location(s) carry both texts")
    return 0


if __name__ == "__main__":
    sys.exit(main())
