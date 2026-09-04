---
name: Feature request (detailed)
about: The long form — for a new regime, a microstructure model or an API change
title: "[feature] "
labels: enhancement
---

<!--
Use this form when the proposal touches the generated data or the command
protocol. Both are pinned by the golden corpus and cross-checked in ten
languages, so a change there is a change to a contract, not just an addition.
The short form (Feature request) is right for everything else.
-->

## The market you cannot generate today

<!--
Describe the data, not the API. "A session with a liquidity vacuum at the open"
says more than "add a parameter".
-->

## Proposed shape

<!--
If it is a spec change, write the JSON you would like to pass. If it is a
command, write the request and the response.
-->

```json
{
  "regimes": [{ "kind": "your_regime", "len": 20, "drift": 0.0, "vol": 0.02 }]
}
```

## Does it change existing output?

<!--
Answer one of:

  * No — new field, defaulted, existing specs generate the same bytes.
  * Yes — existing specs would generate different bytes.

The second is not a refusal; it is a re-bless of golden/expected plus a note in
the changelog, and it needs to be planned rather than discovered.
-->

## Which reaches does it touch?

<!--
Anything crossing the `command_json` boundary lands in all ten: Rust, Python,
Node.js, WASM, C, C++, C#, Go, Java, R. Say so if you believe yours does not.
-->

## Prior art

<!--
Papers, other generators, an exchange's own documented behaviour. A model with a
citation is much easier to review than one without.
-->

## Alternatives considered

<!-- Including composing it from what exists today. -->

## Willing to implement?

<!-- Entirely optional, and a "no" costs nothing. -->
