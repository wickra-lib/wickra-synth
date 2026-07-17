//! The output data model — OHLCV candles plus order-book snapshots, trades and
//! funding. Every type here is `serde` and the JSON form is the binding
//! contract: it must be byte-identical across all ten languages.
//!
//! The `Candle` shape (`ts, open, high, low, close, volume`) mirrors the OHLCV
//! form used across the Wickra ecosystem so a generated stream drops straight
//! into backtests, screeners and feature builders.

use serde::{Deserialize, Serialize};

/// A single OHLCV bar. `ts` is a unitless `i64` timestamp (seconds by default).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Candle {
    pub ts: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// One price/quantity level of an order book.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Level {
    pub price: f64,
    pub qty: f64,
}

/// A single order-book snapshot at a bar close.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BookSnapshot {
    pub ts: i64,
    /// Bid levels, descending by price (`bids[0]` is the best bid).
    pub bids: Vec<Level>,
    /// Ask levels, ascending by price (`asks[0]` is the best ask).
    pub asks: Vec<Level>,
}

/// The aggressor side of a trade print.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

/// A single trade print. `seq` is a global, monotonically increasing counter
/// giving a deterministic tie-break order within a single `ts`.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Trade {
    pub ts: i64,
    pub seq: u64,
    pub price: f64,
    pub qty: f64,
    pub side: Side,
}

/// A funding-rate sample at a bar close.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct FundingSample {
    pub ts: i64,
    pub rate: f64,
}

/// The complete batch output of a generation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GenOutput {
    /// One candle per bar (`len == spec.bars`).
    pub candles: Vec<Candle>,
    /// One snapshot per bar, taken at the bar close.
    pub book_snapshots: Vec<BookSnapshot>,
    /// All trades of all bars, ascending by `(ts, seq)`.
    pub trades: Vec<Trade>,
    /// Funding samples (empty when the spec has no `funding`).
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub funding: Vec<FundingSample>,
}

/// A single event in the streamed form of the same data. Per bar, events are
/// emitted as: all `Trade`s (in `seq` order), then `Book`, then `Candle`, then
/// `Funding` (when due).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    Candle { candle: Candle },
    Book { snapshot: BookSnapshot },
    Trade { trade: Trade },
    Funding { sample: FundingSample },
}

/// Round a value to a fixed 1e-8 grid before serialization. This is the
/// canonical number rule (see `docs/DETERMINISM.md`): it removes any last-bit
/// noise so the Rust CLI and the language bindings agree exactly.
#[must_use]
pub fn round_to(x: f64) -> f64 {
    (x * 1e8).round() / 1e8
}

#[cfg(test)]
mod tests {
    use super::{round_to, Candle, Event, Side, Trade};

    #[test]
    fn round_to_is_stable() {
        assert!((round_to(1.234_567_894_2) - 1.234_567_89).abs() < 1e-12);
        assert!((round_to(100.0) - 100.0).abs() < 1e-12);
    }

    #[test]
    fn candle_json_shape() {
        let c = Candle {
            ts: 1_700_000_000,
            open: 100.0,
            high: 101.5,
            low: 99.2,
            close: 101.0,
            volume: 1234.5,
        };
        let json = serde_json::to_string(&c).unwrap();
        assert_eq!(
            json,
            r#"{"ts":1700000000,"open":100.0,"high":101.5,"low":99.2,"close":101.0,"volume":1234.5}"#
        );
        let back: Candle = serde_json::from_str(&json).unwrap();
        assert_eq!(back, c);
    }

    #[test]
    fn side_serializes_snake_case() {
        assert_eq!(serde_json::to_string(&Side::Buy).unwrap(), r#""buy""#);
        assert_eq!(serde_json::to_string(&Side::Sell).unwrap(), r#""sell""#);
    }

    #[test]
    fn event_is_internally_tagged() {
        let e = Event::Trade {
            trade: Trade {
                ts: 1,
                seq: 0,
                price: 100.0,
                qty: 1.0,
                side: Side::Buy,
            },
        };
        let json = serde_json::to_string(&e).unwrap();
        assert!(json.starts_with(r#"{"type":"trade","trade":"#));
    }
}
