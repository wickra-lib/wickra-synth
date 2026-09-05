#!/usr/bin/env bash
#
# Regenerate every committed lockfile in the workspace:
#   - Rust:   Cargo.lock, fuzz/Cargo.lock        (cargo update)
#   - Node:   bindings/node/package-lock.json    (npm install --package-lock-only)
#   - Python: .github/requirements/ci-dev.txt    (uv pip compile --generate-hashes)
#
# Run from anywhere; the script cd's to the repository root itself:
#
#     ./scripts/update-lockfiles.sh
#
# The Python lock is hash-pinned (OpenSSF Scorecard PinnedDependencies) and
# generated with uv rather than pip-tools, because uv resolves a *target* Python
# version's full transitive closure -- with hashes -- without that interpreter
# being installed locally. One lock covers 3.9 through 3.13 here: the Python
# tests build the extension and parse JSON, so there is no numpy in the closure
# and therefore no cp39/cp313 wheel split to lock around.
#
# This script is deliberately not run in CI. A workflow that regenerates a
# lockfile and proceeds has stopped testing the pinned closure and started
# testing whatever resolved this morning, which is the failure the pinning
# exists to prevent.
#
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "==> Rust (Cargo.lock, fuzz/Cargo.lock)"
cargo update
(cd fuzz && cargo update)

echo "==> Node (bindings/node/package-lock.json)"
(cd bindings/node && npm install --package-lock-only --no-audit --no-fund)

echo "==> Python (.github/requirements/ci-dev.txt via uv)"
# uv is not installed for you unless you ask. Piping an installer URL into a
# shell runs whatever is behind that URL at that moment, with your privileges, on
# the machine of everyone who regenerates a lockfile. Set WKSYNTH_BOOTSTRAP_UV=1
# to opt in; the bootstrap then fetches one pinned release archive and refuses to
# use it unless its checksum matches the one recorded here.
UV_VERSION="0.12.10"
uv_sha256() {
  case "$1" in
    x86_64-unknown-linux-gnu)  echo "173d95a0c32d18c896c46ba6fafbf3cf9c14ab74b033f81b76c883ef492a976b" ;;
    aarch64-unknown-linux-gnu) echo "9ff6b9d4665edcdd3a88dcc73cd1eb641754deb927f14e8c62ebfde6bf4f5f5e" ;;
    aarch64-apple-darwin)      echo "51c6170e8e3a01cef9f33b94f582b7b81ac65046f55d40afb35f9cff5a68c179" ;;
    x86_64-apple-darwin)       echo "5296d5aa2b9143360405eea866f8ef4d5dc8986b164eb0dc35e8f876a9304d30" ;;
    *)                         echo "" ;;
  esac
}

if ! command -v uv >/dev/null 2>&1; then
  if [ "${WKSYNTH_BOOTSTRAP_UV:-0}" != "1" ]; then
    echo "    uv is not on PATH." >&2
    echo "    Install it (https://docs.astral.sh/uv/getting-started/installation/)," >&2
    echo "    or re-run with WKSYNTH_BOOTSTRAP_UV=1 to fetch uv ${UV_VERSION} here." >&2
    exit 1
  fi

  case "$(uname -s)-$(uname -m)" in
    Linux-x86_64)   uv_target="x86_64-unknown-linux-gnu" ;;
    Linux-aarch64)  uv_target="aarch64-unknown-linux-gnu" ;;
    Darwin-arm64)   uv_target="aarch64-apple-darwin" ;;
    Darwin-x86_64)  uv_target="x86_64-apple-darwin" ;;
    *)
      echo "    No pinned uv build for $(uname -s)-$(uname -m); install uv yourself." >&2
      exit 1
      ;;
  esac
  uv_expected="$(uv_sha256 "$uv_target")"

  echo "    bootstrapping uv ${UV_VERSION} (${uv_target})..."
  uv_dir="$(mktemp -d)"
  trap 'rm -rf "$uv_dir"' EXIT
  uv_archive="uv-${uv_target}.tar.gz"
  curl -fsSL --retry 5 --retry-all-errors -o "${uv_dir}/${uv_archive}" \
    "https://github.com/astral-sh/uv/releases/download/${UV_VERSION}/${uv_archive}"
  echo "${uv_expected}  ${uv_dir}/${uv_archive}" | sha256sum -c -
  tar -xzf "${uv_dir}/${uv_archive}" -C "$uv_dir" --strip-components=1
  export PATH="${uv_dir}:$PATH"
fi

req=".github/requirements"
cc="./scripts/update-lockfiles.sh"
uv pip compile --quiet --python-version 3.9 --generate-hashes \
  --custom-compile-command "$cc" "$req/ci-dev.in" -o "$req/ci-dev.txt"

echo "==> Done. Review 'git diff' before committing."
