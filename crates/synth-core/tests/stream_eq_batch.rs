//! `generate_stream` and `generate` are two views of the same data. Re-assemble
//! the event stream back into a `GenOutput` and it must serialize byte-for-byte
//! identically to the batch `generate` — the streaming path must never diverge.

use wickra_synth_core::{generate, generate_stream, Event, GenOutput, GenSpec};

fn specs() -> Vec<&'static str> {
    vec![
        // trend + funding
        r#"{ "seed": 42, "bars": 10, "start_price": 100.0,
            "regimes": [{ "kind": "trend", "len": 10, "drift": 0.002, "vol": 0.01 }],
            "microstructure": { "book_depth": 4, "spread_bps": 4.0, "trade_rate": 6.0,
                "funding": { "interval_bars": 3, "base_rate": 0.0001, "sensitivity": 0.5 } } }"#,
        // multi-regime, no funding
        r#"{ "seed": 2024, "bars": 12, "start_price": 250.0,
            "regimes": [
                { "kind": "range", "len": 4, "drift": 0.0, "vol": 0.008 },
                { "kind": "crash", "len": 4, "drift": 0.01, "vol": 0.03 },
                { "kind": "vol", "len": 4, "drift": 0.0, "vol": 0.02 }],
            "microstructure": { "book_depth": 3, "spread_bps": 5.0, "trade_rate": 2.0 } }"#,
    ]
}

fn reassemble(events: &[Event]) -> GenOutput {
    let mut out = GenOutput {
        candles: Vec::new(),
        book_snapshots: Vec::new(),
        trades: Vec::new(),
        funding: Vec::new(),
    };
    for ev in events {
        match ev {
            Event::Candle { candle } => out.candles.push(*candle),
            Event::Book { snapshot } => out.book_snapshots.push(snapshot.clone()),
            Event::Trade { trade } => out.trades.push(*trade),
            Event::Funding { sample } => out.funding.push(*sample),
        }
    }
    out
}

#[test]
fn stream_reassembles_to_batch() {
    for spec_json in specs() {
        let spec = GenSpec::from_json(spec_json).unwrap();
        let batch = generate(&spec).unwrap();
        let stream = generate_stream(&spec).unwrap();
        let rebuilt = reassemble(&stream);

        assert_eq!(
            serde_json::to_string(&rebuilt).unwrap(),
            serde_json::to_string(&batch).unwrap(),
            "reassembled stream diverged from the batch output"
        );
    }
}
