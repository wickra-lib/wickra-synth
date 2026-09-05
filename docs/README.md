# Documentation

This directory holds the deep dives that belong next to the code, because they
describe contracts the tests enforce rather than usage a website can restate.

| Page | What it covers |
|------|----------------|
| [ARCHITECTURE.md](ARCHITECTURE.md) | Workspace layout and the core-only-randomness design. |
| [GENSPEC.md](GENSPEC.md) | The `GenSpec` input specification, field by field, and every validation rule. |
| [PROTOCOL.md](PROTOCOL.md) | The `command_json` boundary: the four commands, their envelopes, and how failure is detected. |
| [REGIMES.md](REGIMES.md) | The trend / range / crash / vol price-path formulas. |
| [MICROSTRUCTURE.md](MICROSTRUCTURE.md) | Order book, trades and funding. |
| [DETERMINISM.md](DETERMINISM.md) | The PRNG and the fixed per-bar draw order. |
| [Cookbook.md](Cookbook.md) | Practical recipes, CLI and per-language. |

Three of these are contracts, not descriptions. `DETERMINISM.md` states the draw
order that `golden/expected` pins in ten languages, `GENSPEC.md` states the input
the specs are validated against, and `PROTOCOL.md` states the one function every
binding forwards to. Changing any of them changes bytes that someone else's test
is asserting on, so all three are updated in the same pull request as the code,
never after it.

## What is not here

- **The API reference** is rustdoc, published per release at
  [docs.rs/wickra-synth-core](https://docs.rs/wickra-synth-core).
- **Runnable code** lives in [`../examples/`](../examples/), one per language,
  all printing the same three candles from seed 42.
- **The project page** is built from the separate
  [wickra-synth-site](https://github.com/wickra-lib/wickra-synth-site)
  repository. It is not deployed yet, so there is no URL to link; open a pull
  request there for anything that belongs on a website rather than beside the
  code.

Keeping this file short is deliberate. Without it, a reader opening `docs/` has
no way to tell which pages are contracts and which are prose, and a second
documentation tree grows in the repository beside the first.
