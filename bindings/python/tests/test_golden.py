"""Cross-language golden: every binding must produce byte-identical output JSON.

The fixtures live in the repository-root ``golden/`` directory (specs + expected
responses). They are added in a later phase (P-SYN-4); until then this test
skips cleanly.
"""

import json
import pathlib

import pytest

from wickra_synth import Synth

ROOT = pathlib.Path(__file__).resolve().parents[3]
GOLDEN = ROOT / "golden"


def _spec_files() -> list[pathlib.Path]:
    specs = GOLDEN / "specs"
    if not specs.exists():
        return []
    return sorted(specs.glob("*.json"))


@pytest.mark.skipif(not GOLDEN.exists(), reason="golden fixtures not present yet")
@pytest.mark.parametrize("spec_path", _spec_files())
def test_golden_generate_is_byte_identical(spec_path: pathlib.Path) -> None:
    expected = (GOLDEN / "expected" / f"{spec_path.stem}.json").read_text(
        encoding="utf-8"
    )
    synth = Synth(spec_path.read_text(encoding="utf-8"))
    response = synth.command(json.dumps({"cmd": "generate"}))
    assert response == expected.strip()
