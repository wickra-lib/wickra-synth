"""Cross-language golden: every binding must produce byte-identical output JSON.

The fixtures live in the repository-root ``golden/`` directory (specs + expected
responses), blessed from ``wickra-synth-core::generate``. This binding must reproduce
them byte-for-byte.
"""

import json
import pathlib

from wickra_synth import Synth

ROOT = pathlib.Path(__file__).resolve().parents[3]
GOLDEN = ROOT / "golden"


def _spec_files() -> list[pathlib.Path]:
    specs = GOLDEN / "specs"
    if not specs.exists():
        return []
    return sorted(specs.glob("*.json"))


def test_golden_corpus_is_present() -> None:
    """A loop over an empty list checks nothing and passes.

    Without this the whole cross-language guarantee could evaporate from the
    Python side by moving a directory, and the suite would stay green.
    """
    assert _spec_files(), f"no golden specs under {GOLDEN / 'specs'}"


def test_golden_generate_is_byte_identical() -> None:
    # A plain loop rather than pytest.mark.parametrize, so the Python 3.9 CI
    # row can run this module without pytest (see run_without_pytest.py); the
    # spec name is in the message so a failure still says which case.
    for spec_path in _spec_files():
        expected = (GOLDEN / "expected" / f"{spec_path.stem}.json").read_text(
            encoding="utf-8"
        )
        synth = Synth(spec_path.read_text(encoding="utf-8"))
        response = synth.command(json.dumps({"cmd": "generate"}))
        assert response == expected.strip(), spec_path.stem
