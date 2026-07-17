//! Order-book, trade and funding synthesis (§6.6).
//!
//! Per bar, after the price walk, draws happen in this fixed order: (4) one
//! uniform per book level for the bids, then one per level for the asks; (5) a
//! Poisson count of trades; (6) two uniforms per trade (price offset, then
//! quantity). This order is part of the determinism contract — see
//! `docs/DETERMINISM.md`.

use crate::output::{round_to, BookSnapshot, FundingSample, Level, Side, Trade};
use crate::rng::DetRng;

/// Tick size as a fraction of the mid price (level-to-level price step).
const TICK_FRAC: f64 = 0.0001;
/// Baseline quantity at the best book level.
const BOOK_BASE_QTY: f64 = 5.0;
/// Per-level quantity decay away from the mid.
const BOOK_DECAY: f64 = 0.3;
/// Baseline trade quantity.
const TRADE_BASE_QTY: f64 = 1.0;

/// The effective spread for a bar, widened in high-realized-vol regimes.
pub(crate) fn effective_spread(mid: f64, spread_bps: f64, rv: f64, regime_vol: f64) -> f64 {
    mid * (spread_bps / 10000.0) * (1.0 + rv / (regime_vol + 1e-9))
}

/// Build one order-book snapshot: `book_depth` bid levels descending from the
/// mid, then `book_depth` ask levels ascending. Draws one uniform per level per
/// side (bids first, then asks).
pub(crate) fn build_book(
    rng: &mut DetRng,
    mid: f64,
    spread: f64,
    ts: i64,
    book_depth: usize,
) -> BookSnapshot {
    let tick = mid * TICK_FRAC;
    let mut bids = Vec::with_capacity(book_depth);
    for k in 0..book_depth {
        let u = rng.next_f64();
        let level = k as f64;
        let price = mid - spread / 2.0 - level * tick;
        let qty = BOOK_BASE_QTY * (-BOOK_DECAY * level).exp() * (1.0 + u);
        bids.push(Level {
            price: round_to(price),
            qty: round_to(qty),
        });
    }
    let mut asks = Vec::with_capacity(book_depth);
    for k in 0..book_depth {
        let u = rng.next_f64();
        let level = k as f64;
        let price = mid + spread / 2.0 + level * tick;
        let qty = BOOK_BASE_QTY * (-BOOK_DECAY * level).exp() * (1.0 + u);
        asks.push(Level {
            price: round_to(price),
            qty: round_to(qty),
        });
    }
    BookSnapshot { ts, bids, asks }
}

/// Build the trades of a bar: a Poisson count, each with a price offset within
/// the spread and a quantity. `seq` is the global counter, advanced per trade.
pub(crate) fn build_trades(
    rng: &mut DetRng,
    mid: f64,
    spread: f64,
    ts: i64,
    seq: &mut u64,
    trade_rate: f64,
) -> Vec<Trade> {
    let count = rng.next_poisson(trade_rate);
    let mut trades = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let u_price = rng.next_f64();
        let price = mid + spread / 2.0 * (2.0 * u_price - 1.0);
        let u_qty = rng.next_f64();
        let qty = TRADE_BASE_QTY * (0.5 + u_qty);
        let side = if price >= mid { Side::Buy } else { Side::Sell };
        trades.push(Trade {
            ts,
            seq: *seq,
            price: round_to(price),
            qty: round_to(qty),
            side,
        });
        *seq += 1;
    }
    trades
}

/// Build a funding sample from the recent drift (mean of the last
/// `interval_bars` log-returns). Draws no randomness.
pub(crate) fn build_funding(
    ts: i64,
    base_rate: f64,
    sensitivity: f64,
    recent_drift: f64,
) -> FundingSample {
    let rate = base_rate + sensitivity * recent_drift;
    FundingSample {
        ts,
        rate: round_to(rate),
    }
}

#[cfg(test)]
mod tests {
    use super::{build_book, build_funding, build_trades, effective_spread};
    use crate::output::Side;
    use crate::rng::DetRng;

    #[test]
    fn book_is_ordered_and_full_depth() {
        let mut rng = DetRng::from_seed(1);
        let depth = 5;
        let book = build_book(&mut rng, 100.0, 0.04, 1_700_000_000, depth);
        assert_eq!(book.bids.len(), depth);
        assert_eq!(book.asks.len(), depth);
        // Bids descend, asks ascend; best bid < best ask.
        for w in book.bids.windows(2) {
            assert!(w[0].price > w[1].price, "bids not descending");
        }
        for w in book.asks.windows(2) {
            assert!(w[0].price < w[1].price, "asks not ascending");
        }
        assert!(book.bids[0].price < book.asks[0].price);
        for lvl in book.bids.iter().chain(&book.asks) {
            assert!(lvl.price > 0.0 && lvl.qty > 0.0);
        }
    }

    #[test]
    fn trades_have_monotonic_global_seq_and_side_rule() {
        let mut rng = DetRng::from_seed(2);
        let mut seq = 0u64;
        let mid = 100.0;
        let trades = build_trades(&mut rng, mid, 0.5, 1_700_000_000, &mut seq, 20.0);
        assert!(!trades.is_empty());
        assert_eq!(seq, trades.len() as u64);
        for (i, t) in trades.iter().enumerate() {
            assert_eq!(t.seq, i as u64);
            let expected = if t.price >= mid {
                Side::Buy
            } else {
                Side::Sell
            };
            assert_eq!(t.side, expected);
        }
    }

    #[test]
    fn zero_trade_rate_yields_no_trades() {
        let mut rng = DetRng::from_seed(3);
        let mut seq = 0u64;
        let trades = build_trades(&mut rng, 100.0, 0.5, 1, &mut seq, 0.0);
        assert!(trades.is_empty());
        assert_eq!(seq, 0);
    }

    #[test]
    fn spread_widens_with_realized_vol() {
        let calm = effective_spread(100.0, 4.0, 0.0, 0.01);
        let wild = effective_spread(100.0, 4.0, 0.05, 0.01);
        assert!(wild > calm);
    }

    #[test]
    fn funding_reacts_to_drift() {
        let flat = build_funding(1, 0.0001, 0.5, 0.0);
        let up = build_funding(1, 0.0001, 0.5, 0.01);
        assert!(up.rate > flat.rate);
    }
}
