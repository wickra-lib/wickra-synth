# Golden fixtures

The cross-language golden corpus. Each `specs/<name>.json` is a `GenSpec`; each
`expected/<name>.json` is the exact `GenOutput` JSON that `wickra-synth-core::generate`
produces for it. Every language binding replays the same specs and must return
byte-for-byte the same `expected/<name>.json`.

## The seed is the whole input

There is **no `data/` directory**. A synthetic generator takes no market data —
the `seed` (plus the rest of the spec) *is* the complete input. Given a spec, the
output is fully determined by the portable seeded PRNG (SplitMix64 seeding
xoshiro256++) and the fixed per-bar draw order. See
[`docs/DETERMINISM.md`](../docs/DETERMINISM.md) for the PRNG contract and the
binding draw order that make this reproducible across all ten languages.

## Fixtures

| Spec | Regime(s) | Notes |
|------|-----------|-------|
| `trend` | trend | the §6.11 reference spec, with funding |
| `range` | range | mean-reverting, no funding |
| `crash` | crash | down-skewed jump, widened ranges |
| `vol` | vol | drift-free, high volatility |
| `mixed` | trend → range → crash → vol | multiple regimes + funding |

## Blessing

Regenerate the expected outputs from the specs with the CLI (JSON output is
byte-identical to `wickra-synth-core::generate`):

```bash
cargo build -p wickra-synth --release
for s in trend range crash vol mixed; do
  target/release/wickra-synth --spec golden/specs/$s.json --format json \
    > golden/expected/$s.json
done
```

**Never edit `expected/*.json` by hand.** They are machine-generated and pinned;
a mismatch means the core's output changed and every binding must be re-verified
against the new bless.
