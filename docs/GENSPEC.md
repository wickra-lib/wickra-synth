# GenSpec — the input specification

A `GenSpec` is the complete, serde-serializable description of a synthetic
market. It is **data, not code**: the same JSON produces the same output in
every language, and the seed is the entire input — there is no external data
file. Unknown fields are rejected (`deny_unknown_fields`).

## Fields

| Field | Type | Default | Meaning |
|-------|------|---------|---------|
| `seed` | `u64` | — | Master seed; the whole stream derives from it. |
| `bars` | `usize` | — | Number of candles to emit. Must equal the sum of the regime lengths. |
| `start_price` | `f64` | — | Opening price of the first bar. |
| `start_ts` | `i64` | `1700000000` | Unix timestamp (seconds) of the first bar. |
| `bar_secs` | `i64` | `3600` | Seconds per bar (the timestamp step). |
| `regimes` | `[Regime]` | — | Ordered list of market regimes; their lengths must sum to `bars`. |
| `microstructure` | `Microstructure` | — | Order-book, trade-flow and funding model. |

### Regime

| Field | Type | Meaning |
|-------|------|---------|
| `kind` | `"trend" \| "range" \| "crash" \| "vol"` | Price-path model — see [REGIMES.md](REGIMES.md). |
| `len` | `usize` | Number of bars this regime spans. |
| `drift` | `f64` | Per-bar log-return drift (interpretation depends on `kind`). |
| `vol` | `f64` | Per-bar volatility (standard deviation of the log-return shock). |

### Microstructure

| Field | Type | Meaning |
|-------|------|---------|
| `book_depth` | `usize` | Levels per side in each order-book snapshot. |
| `spread_bps` | `f64` | Baseline bid/ask spread in basis points of the mid. |
| `trade_rate` | `f64` | Mean trades per bar (a Poisson rate). |
| `funding` | `FundingSpec?` | Optional periodic funding schedule (omit for none). |

### FundingSpec

| Field | Type | Meaning |
|-------|------|---------|
| `interval_bars` | `usize` | Emit a funding sample every N bars. |
| `base_rate` | `f64` | Baseline funding rate. |
| `sensitivity` | `f64` | How strongly recent drift moves the rate. |

## Example

```json
{
  "seed": 42,
  "bars": 20,
  "start_price": 100.0,
  "regimes": [{ "kind": "trend", "len": 20, "drift": 0.002, "vol": 0.01 }],
  "microstructure": {
    "book_depth": 5, "spread_bps": 4.0, "trade_rate": 8.0,
    "funding": { "interval_bars": 8, "base_rate": 0.0001, "sensitivity": 0.5 }
  }
}
```

The regime lengths **must** sum to `bars`; otherwise `Synth::new` rejects the
spec. Source: `crates/synth-core/src/spec.rs`.

## See also

- [REGIMES.md](REGIMES.md) · [MICROSTRUCTURE.md](MICROSTRUCTURE.md) · [DETERMINISM.md](DETERMINISM.md)
