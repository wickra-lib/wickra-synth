# Roadmap

Wickra Synth is pre-1.0. The near-term focus is a stable, byte-deterministic core
and the ten-language binding surface; the themes below are directional, not
commitments, and may change.

## Toward 0.1.0

- The `synth-core` engine: `GenSpec` model, the portable SplitMix64 → xoshiro256++
  PRNG, the price walk, and order-book / trade / funding synthesis.
- The `command_json` boundary and the reference CLI.
- The ten-language binding surface (C ABI hub + native Python/Node/WASM).
- Golden fixtures pinning cross-platform and cross-language byte-parity.

## Beyond 0.1.0

- **Multi-symbol universes** — generate several correlated instruments at once.
- **Correlated regimes** — shared factors driving a basket, for portfolio-level
  testing.
- **More regime kinds** — additional volatility, trend and liquidity profiles.
- **Richer microstructure** — deeper order books, more realistic trade-flow and
  funding dynamics.

## Non-goals

- Any cryptographic or security use of the PRNG.
- Hosting or serving data — the library generates data locally; distribution is
  the caller's concern.
