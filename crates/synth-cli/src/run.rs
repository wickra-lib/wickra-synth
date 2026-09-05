//! Load a spec, generate, and render the output.

use std::fmt::Write as _;
use std::fs;

use wickra_synth_core::{
    generate, generate_stream, Candle, Event, GenOutput, GenSpec, Microstructure, Regime,
    RegimeKind,
};

use crate::args::{Args, Format};

/// Run the CLI: load the spec, generate, and render to a string.
///
/// # Errors
/// Returns a human-readable message if the spec cannot be loaded, is invalid,
/// or generation fails.
pub fn run(args: &Args) -> Result<String, String> {
    let spec = load_spec(args)?;
    if args.stream {
        let events = generate_stream(&spec).map_err(|e| e.to_string())?;
        render_stream(&events, args.format)
    } else {
        let out = generate(&spec).map_err(|e| e.to_string())?;
        render_batch(&out, args.format)
    }
}

/// Load a spec from `--spec`, or build a single-regime quick-spec from the flags.
fn load_spec(args: &Args) -> Result<GenSpec, String> {
    if let Some(path) = &args.spec {
        let content =
            fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let is_toml = path
            .extension()
            .and_then(|s| s.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("toml"));
        let spec = if is_toml {
            GenSpec::from_toml(&content)
        } else {
            GenSpec::from_json(&content)
        };
        spec.map_err(|e| e.to_string())
    } else {
        Ok(GenSpec {
            seed: args.seed,
            bars: args.bars,
            start_price: args.start_price,
            start_ts: args.start_ts,
            bar_secs: args.bar_secs,
            regimes: vec![Regime {
                kind: parse_kind(&args.kind)?,
                len: args.bars,
                drift: args.drift,
                vol: args.vol,
            }],
            microstructure: Microstructure {
                book_depth: args.book_depth,
                spread_bps: args.spread_bps,
                trade_rate: args.trade_rate,
                funding: None,
            },
        })
    }
}

fn parse_kind(s: &str) -> Result<RegimeKind, String> {
    match s.to_ascii_lowercase().as_str() {
        "trend" => Ok(RegimeKind::Trend),
        "range" => Ok(RegimeKind::Range),
        "crash" => Ok(RegimeKind::Crash),
        "vol" => Ok(RegimeKind::Vol),
        other => Err(format!(
            "unknown regime kind '{other}' (expected trend|range|crash|vol)"
        )),
    }
}

fn render_batch(out: &GenOutput, format: Format) -> Result<String, String> {
    match format {
        Format::Json => serde_json::to_string(out).map_err(|e| e.to_string()),
        Format::Csv => Ok(candles_csv(&out.candles)),
        Format::Text => Ok(batch_summary(out)),
    }
}

fn render_stream(events: &[Event], format: Format) -> Result<String, String> {
    match format {
        // Not `json!({"events": events})`: that round-trips through
        // serde_json::Value and alphabetizes each event's keys, so the CLI
        // emitted `{"trade":{…},"type":"trade"}` where every binding emits
        // `{"type":"trade","trade":{…}}`. The core owns this envelope.
        Format::Json => wickra_synth_core::stream_json(events).map_err(|e| e.to_string()),
        Format::Csv => Ok(candles_csv(&candles_from_events(events))),
        Format::Text => Ok(stream_summary(events)),
    }
}

/// Render candles as `timestamp,open,high,low,close,volume` CSV — the form the
/// wickra ecosystem's CSV reader consumes.
fn candles_csv(candles: &[Candle]) -> String {
    let mut s = String::from("timestamp,open,high,low,close,volume\n");
    for c in candles {
        let _ = writeln!(
            s,
            "{},{},{},{},{},{}",
            c.ts, c.open, c.high, c.low, c.close, c.volume
        );
    }
    s
}

fn candles_from_events(events: &[Event]) -> Vec<Candle> {
    events
        .iter()
        .filter_map(|e| match e {
            Event::Candle { candle } => Some(*candle),
            _ => None,
        })
        .collect()
}

fn batch_summary(out: &GenOutput) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "bars:            {}", out.candles.len());
    let _ = writeln!(s, "book snapshots:  {}", out.book_snapshots.len());
    let _ = writeln!(s, "trades:          {}", out.trades.len());
    let _ = writeln!(s, "funding samples: {}", out.funding.len());
    if let Some(first) = out.candles.first() {
        let _ = writeln!(s, "first candle:    {}", candle_line(first));
    }
    if let Some(last) = out.candles.last() {
        let _ = writeln!(s, "last candle:     {}", candle_line(last));
    }
    s
}

fn stream_summary(events: &[Event]) -> String {
    let (mut candles, mut books, mut trades, mut funding) = (0u64, 0u64, 0u64, 0u64);
    for e in events {
        match e {
            Event::Candle { .. } => candles += 1,
            Event::Book { .. } => books += 1,
            Event::Trade { .. } => trades += 1,
            Event::Funding { .. } => funding += 1,
        }
    }
    let mut s = String::new();
    let _ = writeln!(s, "events:          {}", events.len());
    let _ = writeln!(s, "candle events:   {candles}");
    let _ = writeln!(s, "book events:     {books}");
    let _ = writeln!(s, "trade events:    {trades}");
    let _ = writeln!(s, "funding events:  {funding}");
    s
}

fn candle_line(c: &Candle) -> String {
    format!(
        "ts={} o={} h={} l={} c={} v={}",
        c.ts, c.open, c.high, c.low, c.close, c.volume
    )
}

#[cfg(test)]
mod tests {
    use super::{candles_csv, run};
    use crate::args::{Args, Format};

    fn quick_args(format: Format, stream: bool) -> Args {
        Args {
            spec: None,
            seed: 42,
            bars: 12,
            start_price: 100.0,
            kind: "trend".into(),
            drift: 0.002,
            vol: 0.01,
            bar_secs: 3600,
            start_ts: 1_700_000_000,
            book_depth: 4,
            spread_bps: 4.0,
            trade_rate: 5.0,
            format,
            stream,
        }
    }

    #[test]
    fn json_format_matches_generate_byte_for_byte() {
        use wickra_synth_core::{generate, GenSpec, Microstructure, Regime, RegimeKind};
        let out = run(&quick_args(Format::Json, false)).unwrap();
        let spec = GenSpec {
            seed: 42,
            bars: 12,
            start_price: 100.0,
            start_ts: 1_700_000_000,
            bar_secs: 3600,
            regimes: vec![Regime {
                kind: RegimeKind::Trend,
                len: 12,
                drift: 0.002,
                vol: 0.01,
            }],
            microstructure: Microstructure {
                book_depth: 4,
                spread_bps: 4.0,
                trade_rate: 5.0,
                funding: None,
            },
        };
        let expected = serde_json::to_string(&generate(&spec).unwrap()).unwrap();
        assert_eq!(out, expected);
    }

    #[test]
    fn text_summary_reports_bar_count() {
        let out = run(&quick_args(Format::Text, false)).unwrap();
        assert!(out.contains("bars:            12"));
    }

    #[test]
    fn stream_is_consistent_with_batch() {
        let batch = run(&quick_args(Format::Json, false)).unwrap();
        let stream = run(&quick_args(Format::Json, true)).unwrap();
        // The streamed events carry the same candles as the batch output.
        let batch_v: serde_json::Value = serde_json::from_str(&batch).unwrap();
        let stream_v: serde_json::Value = serde_json::from_str(&stream).unwrap();
        let batch_candles = batch_v["candles"].as_array().unwrap();
        let stream_candles: Vec<_> = stream_v["events"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|e| e["type"] == "candle")
            .map(|e| &e["candle"])
            .collect();
        assert_eq!(batch_candles.len(), stream_candles.len());
        for (b, s) in batch_candles.iter().zip(stream_candles) {
            assert_eq!(b, s);
        }
    }

    #[test]
    fn unknown_kind_is_error() {
        let mut args = quick_args(Format::Text, false);
        args.kind = "nope".into();
        assert!(run(&args).is_err());
    }

    #[test]
    fn csv_round_trips_through_wickra_data() {
        use wickra_synth_core::{generate, GenSpec, Microstructure, Regime, RegimeKind};
        let spec = GenSpec {
            seed: 7,
            bars: 8,
            start_price: 100.0,
            start_ts: 1_700_000_000,
            bar_secs: 60,
            regimes: vec![Regime {
                kind: RegimeKind::Vol,
                len: 8,
                drift: 0.0,
                vol: 0.02,
            }],
            microstructure: Microstructure {
                book_depth: 3,
                spread_bps: 5.0,
                trade_rate: 2.0,
                funding: None,
            },
        };
        let out = generate(&spec).unwrap();
        let csv = candles_csv(&out.candles);

        let mut reader = wickra_data::csv::CandleReader::from_reader(csv.as_bytes()).unwrap();
        let back = reader.read_all().unwrap();
        assert_eq!(back.len(), out.candles.len());
        for (mine, theirs) in out.candles.iter().zip(&back) {
            assert_eq!(mine.ts, theirs.timestamp);
            assert!((mine.open - theirs.open).abs() < 1e-6);
            assert!((mine.high - theirs.high).abs() < 1e-6);
            assert!((mine.low - theirs.low).abs() < 1e-6);
            assert!((mine.close - theirs.close).abs() < 1e-6);
            assert!((mine.volume - theirs.volume).abs() < 1e-6);
        }
    }

    /// The CLI is the eleventh consumer of the `generate_stream` envelope, and
    /// the only one that was building it itself. It emitted
    /// `{"trade":{…},"type":"trade"}` where the ten bindings emit
    /// `{"type":"trade","trade":{…}}`, because `json!` round-trips through
    /// `serde_json::Value` and alphabetizes. `golden/README.md` blesses the corpus
    /// through this binary, so the divergence was one `--stream` fixture away
    /// from being baked into what all ten languages are held to.
    #[test]
    fn stream_json_matches_the_command_boundary() {
        let spec_json = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../golden/specs/trend.json"),
        )
        .unwrap();

        let mut synth = wickra_synth_core::Synth::new(&spec_json).unwrap();
        let from_boundary = synth.command_json(r#"{"cmd":"generate_stream"}"#).unwrap();

        let spec = wickra_synth_core::GenSpec::from_json(&spec_json).unwrap();
        let events = wickra_synth_core::generate_stream(&spec).unwrap();
        let from_cli = super::render_stream(&events, Format::Json).unwrap();

        assert_eq!(from_cli, from_boundary);
    }

    /// The same for the batch path, which happens to agree today and has never
    /// been asserted. `golden/README.md` blesses `expected/*.json` with exactly
    /// this call, so if it ever stops agreeing the corpus is wrong rather than
    /// the bindings.
    #[test]
    fn batch_json_matches_the_command_boundary() {
        let spec_json = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../golden/specs/trend.json"),
        )
        .unwrap();

        let mut synth = wickra_synth_core::Synth::new(&spec_json).unwrap();
        let from_boundary = synth.command_json(r#"{"cmd":"generate"}"#).unwrap();

        let spec = wickra_synth_core::GenSpec::from_json(&spec_json).unwrap();
        let out = wickra_synth_core::generate(&spec).unwrap();
        let from_cli = super::render_batch(&out, Format::Json).unwrap();

        assert_eq!(from_cli, from_boundary);
    }
}
