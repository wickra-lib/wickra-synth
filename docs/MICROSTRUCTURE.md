# Microstructure — order book, trades and funding

After the price walk produces a bar's candle, the core synthesizes the
per-bar microstructure around the bar's mid price. Source:
`crates/synth-core/src/microstructure.rs`.

## Effective spread

The quoted spread widens in turbulent bars:

```
spread = mid · (spread_bps / 10000) · (1 + realized_vol / (regime_vol + 1e-9))
```

so a bar whose realized volatility exceeds the regime's baseline quotes a wider
book than a calm one.

## Order book

Each snapshot has `book_depth` bid levels descending from the mid and
`book_depth` ask levels ascending. Level `k` sits at
`mid ∓ spread/2 ∓ k·tick`, where `tick` is a fixed fraction of the mid. Each
level's quantity decays away from the mid and is perturbed by **one uniform per
level** — bids first, then asks.

## Trades

The number of trades in a bar is a **Poisson draw** with mean `trade_rate`
(`next_poisson`). Each trade then draws **two uniforms**: a price offset within
the spread (`mid + spread/2·(2u − 1)`) and a quantity. A global sequence counter
labels trades across the whole run.

## Funding

If `funding` is set, every `interval_bars` bars the core emits a funding sample
computed from the **mean of the recent log-returns** (a momentum proxy):

```
rate = base_rate + sensitivity · mean(last interval_bars log-returns)
```

Funding draws **no randomness** — it is a deterministic function of the price
path, so it never perturbs the draw stream.

## See also

- [GENSPEC.md](GENSPEC.md) · [REGIMES.md](REGIMES.md) · [DETERMINISM.md](DETERMINISM.md)
