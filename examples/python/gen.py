"""A runnable Python example: generate synthetic microstructure and print the
first three candles.

    pip install wickra-synth
    python examples/python/gen.py

Every language example uses the same seed and prints the same candles — that is
the cross-language guarantee.
"""

import json

from wickra_synth import Synth

SPEC = json.dumps(
    {
        "seed": 42,
        "bars": 20,
        "start_price": 100.0,
        "regimes": [{"kind": "trend", "len": 20, "drift": 0.002, "vol": 0.01}],
        "microstructure": {
            "book_depth": 5,
            "spread_bps": 4.0,
            "trade_rate": 8.0,
            "funding": {"interval_bars": 8, "base_rate": 0.0001, "sensitivity": 0.5},
        },
    }
)


def main() -> None:
    synth = Synth(SPEC)
    out = json.loads(synth.command(json.dumps({"cmd": "generate"})))

    print(f"wickra-synth {Synth.version()}")
    print(f"bars: {len(out['candles'])}")
    print("first 3 candles:")
    for candle in out["candles"][:3]:
        print(f"  {json.dumps(candle, separators=(',', ':'))}")


if __name__ == "__main__":
    main()
