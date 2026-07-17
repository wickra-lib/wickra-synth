"""Smoke test: construct a synth, generate, and parse the response."""

import json

from wickra_synth import Synth, __version__

SPEC = json.dumps(
    {
        "seed": 42,
        "bars": 8,
        "start_price": 100.0,
        "regimes": [{"kind": "trend", "len": 8, "drift": 0.002, "vol": 0.01}],
        "microstructure": {"book_depth": 3, "spread_bps": 4.0, "trade_rate": 3.0},
    }
)


def test_generate_roundtrip() -> None:
    synth = Synth(SPEC)
    out = json.loads(synth.command(json.dumps({"cmd": "generate"})))
    assert len(out["candles"]) == 8
    assert len(out["book_snapshots"]) == 8
    # Trades carry a globally-monotonic seq.
    for i, trade in enumerate(out["trades"]):
        assert trade["seq"] == i


def test_stream_matches_batch() -> None:
    synth = Synth(SPEC)
    batch = json.loads(synth.command(json.dumps({"cmd": "generate"})))
    events = json.loads(synth.command(json.dumps({"cmd": "generate_stream"})))["events"]
    stream_candles = [e["candle"] for e in events if e["type"] == "candle"]
    assert stream_candles == batch["candles"]


def test_deterministic_for_same_seed() -> None:
    a = Synth(SPEC).command(json.dumps({"cmd": "generate"}))
    b = Synth(SPEC).command(json.dumps({"cmd": "generate"}))
    assert a == b


def test_version_matches_module() -> None:
    assert Synth.version() == __version__


def test_bad_spec_raises() -> None:
    try:
        Synth("not json")
    except ValueError:
        return
    raise AssertionError("expected ValueError for a malformed spec")
