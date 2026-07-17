# Determinism — the PRNG and draw-order contract

The whole promise of Wickra Synth is that a seed yields the **byte-for-byte
identical** market on every platform and in every language. Two invariants make
that true, and both are load-bearing: change either and the golden corpus breaks.

## 1. A portable PRNG, in the core only

All randomness flows through `crates/synth-core/src/rng.rs` and **nowhere else**
— no binding ever draws its own randomness. The generator is a `SplitMix64` seed
expander feeding an `xoshiro256++` stream (`DetRng`), written with explicit
wrapping 64-bit arithmetic so the bit sequence is identical on every target.

It is a **non-cryptographic** generator chosen for speed and reproducibility. It
must never be used for keys, tokens, or any security purpose.

### Reference vectors

These are pinned by unit tests; any drift is a determinism regression.

```
SplitMix64::new(0)    -> 0xE220A8397B1DCDAF, 0x6E789E6AA1B965F4, 0x06C45D188009454F
DetRng::from_seed(42) -> 0xD0764D4F4476689F, 0x519E4174576F3791
```

`next_normal` consumes **two** uniforms (Box-Muller). `next_poisson(lambda)`
returns `0` for `lambda <= 0`.

## 2. A fixed draw order per bar

For each bar the core consumes draws in exactly this order. This is the contract:

1. **Return shock** `z` — `next_normal` (two uniforms, Box-Muller).
2. **Intrabar range** — one uniform.
3. **Volume** — one uniform.
4. **Order book** — one uniform per level, all `book_depth` **bids first**, then
   all `book_depth` **asks**.
5. **Trade count** — one Poisson draw (`trade_rate`).
6. **Per trade** — two uniforms each: price offset, then quantity.

Funding (when scheduled) draws **no randomness**; it is a pure function of the
recent price path, so it never shifts the stream.

Because the RNG is portable and the order is fixed, every binding that forwards
the same `command_json` reproduces the same bytes — there is no per-language
reformatting, and no binding ever draws.

## See also

- [ARCHITECTURE.md](ARCHITECTURE.md) · [REGIMES.md](REGIMES.md) · [MICROSTRUCTURE.md](MICROSTRUCTURE.md)
