//! A runnable Rust example: generate synthetic microstructure from a seeded spec
//! and print the first three candles. Every language example uses the same seed
//! and prints the same candles — that is the cross-language guarantee.
//!
//! ```bash
//! cargo run -p wickra-synth-example
//! ```

use wickra_synth_core::{generate, GenSpec};

const SPEC: &str = r#"{
    "seed": 42,
    "bars": 20,
    "start_price": 100.0,
    "regimes": [{ "kind": "trend", "len": 20, "drift": 0.002, "vol": 0.01 }],
    "microstructure": { "book_depth": 5, "spread_bps": 4.0, "trade_rate": 8.0,
        "funding": { "interval_bars": 8, "base_rate": 0.0001, "sensitivity": 0.5 } }
}"#;

fn main() {
    let spec: GenSpec = GenSpec::from_json(SPEC).expect("valid spec");
    let out = generate(&spec).expect("generate");

    println!("wickra-synth {}", wickra_synth_core::version());
    println!("bars: {}", out.candles.len());
    println!("first 3 candles:");
    for candle in out.candles.iter().take(3) {
        println!("  {}", serde_json::to_string(candle).unwrap());
    }
}
