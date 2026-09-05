//! Serde conformance: every public spec/output type round-trips through JSON,
//! and the spec format rejects unknown and missing fields — the JSON contract is
//! the binding surface, so a silent shape change here would break every binding.

use wickra_synth_core::{
    generate, BookSnapshot, Candle, Event, FundingSample, FundingSpec, GenSpec, Level,
    Microstructure, Regime, RegimeKind, Side, Trade,
};

const SPEC: &str = r#"{
    "seed": 42, "bars": 6, "start_price": 100.0,
    "regimes": [{ "kind": "trend", "len": 6, "drift": 0.002, "vol": 0.01 }],
    "microstructure": {
        "book_depth": 3, "spread_bps": 4.0, "trade_rate": 3.0,
        "funding": { "interval_bars": 3, "base_rate": 0.0001, "sensitivity": 0.5 }
    }
}"#;

fn roundtrip<T>(value: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(value).unwrap();
    let back: T = serde_json::from_str(&json).unwrap();
    assert_eq!(&back, value, "roundtrip mismatch via {json}");
}

#[test]
fn spec_roundtrips() {
    let spec = GenSpec::from_json(SPEC).unwrap();
    roundtrip(&spec);
}

#[test]
fn every_regime_kind_roundtrips() {
    for kind in [
        RegimeKind::Trend,
        RegimeKind::Range,
        RegimeKind::Crash,
        RegimeKind::Vol,
    ] {
        roundtrip(&Regime {
            kind,
            len: 4,
            drift: 0.001,
            vol: 0.02,
        });
    }
}

#[test]
fn microstructure_with_and_without_funding_roundtrips() {
    roundtrip(&Microstructure {
        book_depth: 5,
        spread_bps: 4.0,
        trade_rate: 8.0,
        funding: None,
    });
    roundtrip(&Microstructure {
        book_depth: 5,
        spread_bps: 4.0,
        trade_rate: 8.0,
        funding: Some(FundingSpec {
            interval_bars: 8,
            base_rate: 0.0001,
            sensitivity: 0.5,
        }),
    });
}

#[test]
fn output_types_roundtrip() {
    let out = generate(&GenSpec::from_json(SPEC).unwrap()).unwrap();
    roundtrip(&out);
    roundtrip(&out.candles[0]);
    roundtrip(&out.book_snapshots[0]);
    // Exercise the small leaf types explicitly.
    roundtrip(&Candle {
        ts: 1,
        open: 1.0,
        high: 2.0,
        low: 0.5,
        close: 1.5,
        volume: 10.0,
    });
    roundtrip(&Level {
        price: 100.0,
        qty: 3.0,
    });
    roundtrip(&BookSnapshot {
        ts: 1,
        bids: vec![Level {
            price: 99.0,
            qty: 1.0,
        }],
        asks: vec![Level {
            price: 101.0,
            qty: 1.0,
        }],
    });
    for side in [Side::Buy, Side::Sell] {
        roundtrip(&Trade {
            ts: 1,
            seq: 0,
            price: 100.0,
            qty: 1.0,
            side,
        });
    }
    roundtrip(&FundingSample { ts: 1, rate: 0.0 });
}

#[test]
fn event_variants_roundtrip() {
    let events = wickra_synth_core::generate_stream(&GenSpec::from_json(SPEC).unwrap()).unwrap();
    // The stream carries every event variant (trade, book, candle, funding).
    for ev in &events {
        roundtrip(ev);
    }
    // And each variant serializes with the internal `type` tag.
    let candle = events
        .iter()
        .find(|e| matches!(e, Event::Candle { .. }))
        .unwrap();
    assert!(serde_json::to_string(candle)
        .unwrap()
        .contains("\"type\":\"candle\""));
}

#[test]
fn unknown_field_is_rejected() {
    let bad = r#"{ "seed": 1, "bars": 2, "start_price": 100.0, "surprise": true,
        "regimes": [{ "kind": "trend", "len": 2, "drift": 0.0, "vol": 0.01 }],
        "microstructure": { "book_depth": 1, "spread_bps": 1.0, "trade_rate": 1.0 } }"#;
    assert!(
        GenSpec::from_json(bad).is_err(),
        "unknown field must be rejected"
    );
}

#[test]
fn missing_required_field_is_rejected() {
    // `bars` is required.
    let bad = r#"{ "seed": 1, "start_price": 100.0,
        "regimes": [{ "kind": "trend", "len": 2, "drift": 0.0, "vol": 0.01 }],
        "microstructure": { "book_depth": 1, "spread_bps": 1.0, "trade_rate": 1.0 } }"#;
    assert!(
        GenSpec::from_json(bad).is_err(),
        "missing field must be rejected"
    );
}

#[test]
fn unknown_regime_kind_is_rejected() {
    let bad = r#"{ "seed": 1, "bars": 2, "start_price": 100.0,
        "regimes": [{ "kind": "moon", "len": 2, "drift": 0.0, "vol": 0.01 }],
        "microstructure": { "book_depth": 1, "spread_bps": 1.0, "trade_rate": 1.0 } }"#;
    assert!(
        GenSpec::from_json(bad).is_err(),
        "unknown regime kind must be rejected"
    );
}
