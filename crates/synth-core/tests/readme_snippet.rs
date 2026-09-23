//! The README's opening snippet, compiled and run.
//!
//! A snippet in the README is a claim about the API; this is the claim being
//! checked, so a rename that would make the page wrong fails the suite instead.
use wickra_synth_core::{generate, GenSpec};

const SPEC: &str = r#"{
    "seed": 42,
    "bars": 20,
    "start_price": 100.0,
    "regimes": [{ "kind": "trend", "len": 20, "drift": 0.002, "vol": 0.01 }],
    "microstructure": { "book_depth": 5, "spread_bps": 4.0, "trade_rate": 8.0 }
}"#;

#[test]
fn the_readme_snippet_compiles_and_replays() -> Result<(), Box<dyn std::error::Error>> {
    let spec = GenSpec::from_json(SPEC)?;
    let out = generate(&spec)?;
    assert_eq!(out.candles.len(), 20);
    assert!(!out.trades.is_empty());
    assert_eq!(generate(&spec)?, out);
    Ok(())
}
