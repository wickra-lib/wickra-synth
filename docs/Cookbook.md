# Cookbook

Short, practical recipes. Every one is deterministic: the same seed gives the
same bytes in every language.

## Generate a trending market (CLI)

```bash
wickra-synth --seed 42 --bars 20 --regime trend --drift 0.002 --vol 0.01 --format json
```

`--format text` prints a human summary (bar count, book snapshots, trades)
instead of the JSON payload.

## Generate from a spec file

```bash
wickra-synth --spec my-spec.json --format json > market.json
```

A `--spec` file (JSON or TOML) takes precedence over the quick-spec flags and is
the only way to express multiple regimes or a funding schedule.

## Multi-regime market (spec file)

```json
{
  "seed": 2024, "bars": 32, "start_price": 100.0,
  "regimes": [
    { "kind": "trend", "len": 8, "drift": 0.003, "vol": 0.01 },
    { "kind": "range", "len": 8, "drift": 0.5,   "vol": 0.012 },
    { "kind": "crash", "len": 8, "drift": 0.01,  "vol": 0.03 },
    { "kind": "vol",   "len": 8, "drift": 0.0,   "vol": 0.04 }
  ],
  "microstructure": {
    "book_depth": 5, "spread_bps": 5.0, "trade_rate": 12.0,
    "funding": { "interval_bars": 8, "base_rate": 0.0001, "sensitivity": 0.5 }
  }
}
```

The regime lengths sum to `bars` (8 × 4 = 32) — a mismatch is rejected.

## From Rust

```rust
use synth_core::{generate, GenSpec};

let spec: GenSpec = GenSpec::from_json(spec_json).expect("valid spec");
let out = generate(&spec).expect("generate");
println!("{} candles", out.candles.len());
```

## From any binding (JSON-over-C-ABI)

```python
from wickra_synth import Synth
import json

synth = Synth(spec_json)
out = json.loads(synth.command('{"cmd":"generate"}'))
print(len(out["candles"]))
```

`command` never raises for a bad command or spec — it returns
`{"ok": false, "error": ...}` in-band. Only the `Synth(...)` constructor rejects
a non-empty invalid spec.

## Stream events instead of a batch

The `generate_stream` command interleaves candles, book snapshots, trades and
funding as a single ordered `events` array — useful for feeding an event loop:

```python
events = json.loads(synth.command('{"cmd":"generate_stream"}'))["events"]
```

## See also

- [GENSPEC.md](GENSPEC.md) · [REGIMES.md](REGIMES.md) · [MICROSTRUCTURE.md](MICROSTRUCTURE.md) · [DETERMINISM.md](DETERMINISM.md)
