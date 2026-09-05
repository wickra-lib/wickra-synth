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

## Validation

`GenSpec::validate` is the whole input contract, and it is public: a caller that
builds a spec in code rather than parsing one can ask before generating.
`from_json`, `from_toml` and every `command_json` path run it, so a spec that
reaches the generator has passed all of it. The generator may then assume
validity, which is why it contains no defensive checks of its own.

The first violated rule is the one reported.

| Rule | Message |
|------|---------|
| `bars > 0` | `bars must be > 0` |
| `start_price` finite and `> 0` | `start_price must be finite and > 0` |
| `bar_secs > 0` | `bar_secs must be > 0` |
| `regimes` non-empty | `regimes must not be empty` |
| every `regime.len > 0` | `regime len must be > 0` |
| every `regime.drift` finite | `regime drift must be finite` |
| every `regime.vol` finite and `>= 0` | `regime vol must be finite and >= 0` |
| regime lengths sum to `bars` | `sum of regime lengths must equal bars` |
| `book_depth > 0` | `book_depth must be > 0` |
| `spread_bps` finite and `>= 0` | `spread_bps must be finite and >= 0` |
| `trade_rate` finite and `>= 0` | `trade_rate must be finite and >= 0` |
| `funding.interval_bars > 0` | `funding.interval_bars must be > 0` |
| `funding.base_rate` and `sensitivity` finite | `funding base_rate and sensitivity must be finite` |
| the timeline fits: `start_ts + (bars - 1) * bar_secs` does not overflow `i64` | `start_ts + (bars - 1) * bar_secs overflows i64` |

The last rule is the one that is not obvious from the field list. The generator
walks `bar_ts += bar_secs` once per bar; without it, a spec near the end of
`i64` panicked in a debug build and wrapped in a release build, so the last
candle carried a timestamp before the first.

**What is deliberately not bounded:** `bars` and `book_depth` have no upper
limit. They are the caller's own sizes, and generating a billion bars allocates
a billion bars — that is the API doing what it was asked, not a defect. A
process that accepts specs from an untrusted source bounds them itself; see
[SECURITY.md](../SECURITY.md).

Source: `crates/synth-core/src/spec.rs`.

## See also

- [REGIMES.md](REGIMES.md) · [MICROSTRUCTURE.md](MICROSTRUCTURE.md) · [DETERMINISM.md](DETERMINISM.md)
