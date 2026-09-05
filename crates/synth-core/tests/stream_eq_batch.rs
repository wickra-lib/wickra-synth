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

/// The per-bar event order is a contract: `docs/DETERMINISM.md` states it, and
/// every real-time consumer in ten languages folds the stream assuming it. The
/// reassembly test above cannot see it -- it sorts events into four per-kind
/// vectors, so cross-kind order is discarded before the comparison. Moving the
/// candle above the trades left the whole suite green.
#[test]
fn per_bar_event_order_is_pinned() {
    for spec_json in specs() {
        let spec = GenSpec::from_json(spec_json).unwrap();
        let events = generate_stream(&spec).unwrap();

        // Within one bar: all trades, then the book, then the candle, then
        // funding if it is due. Bars are delimited by the candle, so walking the
        // stream and resetting at each candle recovers the per-bar grouping.
        let mut seen_book = false;
        let mut seen_candle = false;
        for (i, ev) in events.iter().enumerate() {
            match ev {
                Event::Trade { .. } => {
                    assert!(
                        !seen_book,
                        "trade after the book in bar ending at event {i}"
                    );
                }
                Event::Book { .. } => {
                    assert!(!seen_book, "two books in one bar at event {i}");
                    seen_book = true;
                }
                Event::Candle { .. } => {
                    assert!(seen_book, "candle before the book at event {i}");
                    seen_candle = true;
                }
                Event::Funding { .. } => {
                    assert!(seen_candle, "funding before the candle at event {i}");
                    // Funding closes the bar.
                    seen_book = false;
                    seen_candle = false;
                    continue;
                }
            }
            if seen_candle {
                // A candle with no funding behind it also closes the bar, but
                // only once the next event proves the bar ended.
                if matches!(events.get(i + 1), Some(Event::Funding { .. })) {
                    continue;
                }
                seen_book = false;
                seen_candle = false;
            }
        }
    }
}

/// The number of events is not incidental: one candle and one book per bar, one
/// funding sample per interval, and every trade. A stream that dropped a book
/// would still reassemble correctly if the batch dropped it too.
#[test]
fn stream_carries_every_batch_record() {
    for spec_json in specs() {
        let spec = GenSpec::from_json(spec_json).unwrap();
        let batch = generate(&spec).unwrap();
        let events = generate_stream(&spec).unwrap();
        let expected = batch.candles.len()
            + batch.book_snapshots.len()
            + batch.trades.len()
            + batch.funding.len();
        assert_eq!(
            events.len(),
            expected,
            "stream and batch record counts differ"
        );
    }
}
