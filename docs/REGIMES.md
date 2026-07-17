# Regimes — the price-path formulas

Each bar's log-return is drawn from the active regime. The core draws one
standard-normal shock `z` per bar (two uniforms via Box-Muller) and combines it
with the regime's `drift` and `vol`. The close is `open · exp(log_ret)`.

Source: `crates/synth-core/src/walk.rs`.

## Kinds

| Kind | Log-return | Behaviour |
|------|-----------|-----------|
| `trend` | `drift + vol·z` | Constant drift with Gaussian noise — a directional market. |
| `range` | `-drift·ln(price / regime_start) + vol·z` | Mean reversion toward the price at the start of the regime; `drift` is the reversion strength. |
| `crash` | `-\|drift\| + vol·(z − 0.5·\|z\|)` | Persistent negative drift with a down-skewed shock — fat left tail. |
| `vol` | `vol·z` | Zero-drift, pure volatility. |

`z` is a standard normal; `regime_start` is the close at the first bar of the
current regime (reset whenever a new regime begins).

## High / low and volume

After the return, the core draws:

- `u_range` (one uniform) → the intrabar range `r = max(|log_ret|, vol)·(0.5 + u_range)`;
  `high = max(open, close)·exp(r/2)`, `low = min(open, close)·exp(−r/2)`, so the
  candle always satisfies `low ≤ open, close ≤ high`.
- `vol_u` (one uniform) → `volume = 1000·(1 + 0.5·|z|)·(0.5 + vol_u)`, so volume
  rises with the magnitude of the move.

## Multi-regime specs

Regimes run back to back in the order given; a spec whose regime lengths do not
sum to `bars` is rejected. A common pattern is trend → range → crash → vol to
exercise every path in one run (see `golden/specs/mixed.json`).

## See also

- [GENSPEC.md](GENSPEC.md) · [MICROSTRUCTURE.md](MICROSTRUCTURE.md) · [DETERMINISM.md](DETERMINISM.md)
