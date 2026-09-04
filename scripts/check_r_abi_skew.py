#!/usr/bin/env python3
"""Assert that the R binding can link against the C ABI its version names.

Every other binding ships its native code in the same artefact as its wrapper, so
the two can never disagree. R is the exception: `bindings/r/configure` downloads
the prebuilt `wickra-synth-c-<triple>.tar.gz` release asset named by
`DESCRIPTION: Version` and compiles `src/wickra_synth.c` against *that* header.
The wrapper comes from the working tree; the ABI comes from a published release.

Our own CI never sees that pairing, because the R job sets WKSYNTH_INC and
WKSYNTH_LIB and builds against the header in the tree, which match by
construction. r-universe does see it, and reports the mismatch days later.

Two claims, only one of them blocking:

  * Every `wickra_synth_*` symbol the wrapper calls must exist in the header in
    this tree. A violation means the wrapper is stale, which is a defect, and
    fails.
  * The same, against the header at the tag `DESCRIPTION: Version` names. A
    violation means main is ahead of the last release and r-universe will stay
    red until the next one -- expected between a feature landing and its release,
    so it is reported and does not fail. Before a release it should be empty.

    python scripts/check_r_abi_skew.py              # both claims, offline-safe
    python scripts/check_r_abi_skew.py --offline    # skip the released header
"""

from __future__ import annotations

import argparse
import os
import re
import sys
import urllib.error
import urllib.request

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
WRAPPER = "bindings/r/src/wickra_synth.c"
HEADER = "bindings/c/include/wickra_synth.h"
DESCRIPTION = "bindings/r/DESCRIPTION"
RAW = "https://raw.githubusercontent.com/wickra-lib/wickra-synth/v{version}/" + HEADER

# `name(` where name starts with the ABI prefix. Declarations in the header and
# calls in the wrapper have the same shape, so one pattern reads both.
SYMBOL = re.compile(r"\b(wickra_synth_[a-z_]+)\s*\(")


def read(rel: str) -> str:
    path = os.path.join(ROOT, rel)
    if not os.path.exists(path):
        sys.exit(f"missing file: {rel}")
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def described_version() -> str:
    match = re.search(r"(?m)^Version:\s*(\S+)", read(DESCRIPTION))
    if not match:
        sys.exit(f"{DESCRIPTION}: no Version field")
    return match.group(1)


def fetch(url: str) -> str | None:
    try:
        with urllib.request.urlopen(url, timeout=30) as response:  # noqa: S310
            return response.read().decode("utf-8")
    except (urllib.error.URLError, urllib.error.HTTPError, TimeoutError):
        return None


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--offline", action="store_true", help="skip the released header")
    args = parser.parse_args()

    called = set(SYMBOL.findall(read(WRAPPER)))
    if not called:
        sys.exit(f"{WRAPPER}: calls no wickra_synth_* symbol at all")

    in_tree = set(SYMBOL.findall(read(HEADER)))
    stale = sorted(called - in_tree)
    if stale:
        print(f"R ABI skew: the wrapper calls {len(stale)} symbol(s) the header does not declare\n")
        for symbol in stale:
            print(f"  {symbol}")
        print(f"\n{WRAPPER} is stale against {HEADER}.")
        return 1

    version = described_version()
    if args.offline:
        print(f"R ABI skew: {len(called)} symbol(s) resolve in the tree (released header skipped)")
        return 0

    released = fetch(RAW.format(version=version))
    if released is None:
        # No release yet, or no network. Both are "not decidable", not "green":
        # say so rather than printing a pass the reader would trust.
        print(
            f"R ABI skew: {len(called)} symbol(s) resolve in the tree. "
            f"The header for v{version} could not be fetched (no such tag yet, or no "
            f"network), so the released-ABI half is unchecked."
        )
        return 0

    ahead = sorted(called - set(SYMBOL.findall(released)))
    if ahead:
        print(
            f"R ABI skew: {len(ahead)} symbol(s) exist in the tree but not in v{version}, "
            f"which is the release bindings/r/configure downloads:\n"
        )
        for symbol in ahead:
            print(f"  {symbol}")
        print(
            "\nExpected between a change landing and its release; r-universe will be red "
            "until v" + version + " ships. Not a failure."
        )
        return 0

    print(f"R ABI skew: {len(called)} symbol(s) resolve in the tree and in v{version}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
