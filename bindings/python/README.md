# Wickra Synth — Python

Python bindings for [wickra-synth](https://github.com/wickra-lib/wickra-synth),
the deterministic synthetic-microstructure generator. Build a `Synth` from a
spec JSON, drive it with command JSONs, and read back the generated OHLCV, order
book, trades and funding — the same command protocol every language binding
speaks, byte-identical for a given seed.

## Install

```sh
pip install wickra-synth
```

## Usage

```python
import json
from wickra_synth import Synth

spec = json.dumps({
    "seed": 42,
    "bars": 20,
    "start_price": 100.0,
    "regimes": [{"kind": "trend", "len": 20, "drift": 0.002, "vol": 0.01}],
    "microstructure": {"book_depth": 5, "spread_bps": 4.0, "trade_rate": 8.0},
})

synth = Synth(spec)

out = json.loads(synth.command(json.dumps({"cmd": "generate"})))
print(len(out["candles"]))   # 20
print(out["candles"][0])     # {'ts': 1700000000, 'open': 100.0, ...}

# The same seed yields byte-identical output in every language binding.
```

## API

| Method | Description |
|--------|-------------|
| `Synth(spec_json)` | Build a synth from a spec JSON (raises `ValueError` if invalid). |
| `synth.command(cmd_json) -> str` | Apply a command JSON, return the response JSON. Commands: `set_spec`, `generate`, `generate_stream`, `version`. |
| `Synth.version() -> str` | The library version. |

## Build from source

```sh
maturin develop --release
pytest -q
```

## License

`MIT OR Apache-2.0`.
