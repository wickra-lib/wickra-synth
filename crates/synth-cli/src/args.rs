//! Command-line arguments for `wickra-synth`.

use clap::Parser;
use std::path::PathBuf;

/// Deterministic synthetic market-microstructure generator.
///
/// Either pass `--spec <file.json|file.toml>`, or use the quick-spec flags to
/// build a single-regime spec on the command line.
#[derive(Parser, Debug)]
#[command(name = "wickra-synth", version, about)]
pub struct Args {
    /// Path to a spec file (`.json` or `.toml`). Takes precedence over the
    /// quick-spec flags.
    #[arg(long)]
    pub spec: Option<PathBuf>,

    /// Quick-spec: master seed.
    #[arg(long, default_value_t = 42)]
    pub seed: u64,
    /// Quick-spec: number of bars.
    #[arg(long, default_value_t = 20)]
    pub bars: usize,
    /// Quick-spec: starting price.
    #[arg(long = "start-price", default_value_t = 100.0)]
    pub start_price: f64,
    /// Quick-spec: regime kind (`trend`, `range`, `crash`, or `vol`).
    #[arg(long, default_value = "trend")]
    pub kind: String,
    /// Quick-spec: per-bar log-return drift.
    #[arg(long, default_value_t = 0.001)]
    pub drift: f64,
    /// Quick-spec: per-bar volatility.
    #[arg(long, default_value_t = 0.01)]
    pub vol: f64,
    /// Quick-spec: seconds per bar.
    #[arg(long = "bar-secs", default_value_t = 3600)]
    pub bar_secs: i64,
    /// Quick-spec: first bar timestamp.
    #[arg(long = "start-ts", default_value_t = 1_700_000_000)]
    pub start_ts: i64,
    /// Quick-spec: order-book levels per side.
    #[arg(long = "book-depth", default_value_t = 5)]
    pub book_depth: usize,
    /// Quick-spec: target spread in basis points.
    #[arg(long = "spread-bps", default_value_t = 4.0)]
    pub spread_bps: f64,
    /// Quick-spec: expected trades per bar.
    #[arg(long = "trade-rate", default_value_t = 8.0)]
    pub trade_rate: f64,

    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    pub format: Format,
    /// Emit the event stream instead of the batch output.
    #[arg(long)]
    pub stream: bool,
}

/// Output format for the generated data.
#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// A compact human-readable summary.
    Text,
    /// The full output as JSON (byte-identical to `synth_core::generate`).
    Json,
    /// The candles as CSV (`timestamp,open,high,low,close,volume`).
    Csv,
}
