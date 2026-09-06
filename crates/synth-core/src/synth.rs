//! The `Synth` handle and the `command_json` FFI boundary (§6.9).

use serde::Serialize;
use serde_json::Value;

use crate::error::{Error, Result};
use crate::generate::{generate, generate_stream};
use crate::output::{Event, GenOutput};
use crate::spec::GenSpec;

/// The `generate_stream` response envelope. Serializing through a struct (rather
/// than the `json!` macro, which round-trips via `serde_json::Value` and
/// alphabetizes nested keys) keeps each event's candle in the same field order
/// as the batch `generate` output — the two paths stay byte-for-byte consistent.
#[derive(Serialize)]
struct StreamResponse<'a> {
    events: &'a [Event],
}

/// Serialize an event list into the exact envelope `command_json` returns for
/// `generate_stream`.
///
/// Public because the CLI is the eleventh consumer of this protocol and had
/// rebuilt the envelope with the `json!` macro — the one thing the struct above
/// exists to avoid. Every consumer that emits a stream response calls this, so
/// there is one place the field order is decided.
///
/// # Errors
/// Propagates a serialization failure, which for these types cannot occur.
pub fn stream_json(events: &[Event]) -> Result<String> {
    Ok(serde_json::to_string(&StreamResponse { events })?)
}

/// A stateful synth handle. Holds an optional current spec; `command_json` is
/// the single dispatch entry point every language binding calls.
pub struct Synth {
    spec: Option<GenSpec>,
}

impl Synth {
    /// Construct a handle. `spec_json` may be empty or `{}` (no spec yet) or a
    /// full [`GenSpec`].
    ///
    /// # Errors
    /// Returns [`Error::Parse`]/[`Error::BadSpec`] if a non-empty spec is invalid.
    pub fn new(spec_json: &str) -> Result<Self> {
        let trimmed = spec_json.trim();
        let spec = if trimmed.is_empty() || trimmed == "{}" {
            None
        } else {
            Some(GenSpec::from_json(trimmed)?)
        };
        Ok(Self { spec })
    }

    /// Set the current spec.
    pub fn set_spec(&mut self, spec: GenSpec) {
        self.spec = Some(spec);
    }

    /// Generate the batch output from the current spec.
    ///
    /// # Errors
    /// Returns [`Error::BadSpec`] if no spec is set, else propagates generation errors.
    pub fn generate(&self) -> Result<GenOutput> {
        let spec = self
            .spec
            .as_ref()
            .ok_or_else(|| Error::BadSpec("no spec set".into()))?;
        generate(spec)
    }

    /// Generate the event stream from the current spec.
    ///
    /// # Errors
    /// Returns [`Error::BadSpec`] if no spec is set, else propagates generation errors.
    pub fn generate_stream(&self) -> Result<Vec<Event>> {
        let spec = self
            .spec
            .as_ref()
            .ok_or_else(|| Error::BadSpec("no spec set".into()))?;
        generate_stream(spec)
    }

    /// The crate version.
    #[must_use]
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Dispatch a `{"cmd": ...}` request and return a JSON string. Internal
    /// errors are returned in-band as `{"ok":false,"error":...}`; this method
    /// itself does not fail.
    ///
    /// # Errors
    /// Never returns `Err` — the `Result` is kept for signature symmetry with
    /// the rest of the ecosystem; errors are encoded in the returned JSON.
    pub fn command_json(&mut self, cmd_json: &str) -> Result<String> {
        Ok(self
            .dispatch(cmd_json)
            .unwrap_or_else(|e| error_json(&e.to_string())))
    }

    fn dispatch(&mut self, cmd_json: &str) -> Result<String> {
        let v: Value = serde_json::from_str(cmd_json)?;
        let cmd = v
            .get("cmd")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::BadSpec("missing cmd".into()))?;
        match cmd {
            "set_spec" => {
                let spec_v = v
                    .get("spec")
                    .ok_or_else(|| Error::BadSpec("set_spec requires a spec".into()))?;
                let spec: GenSpec = serde_json::from_value(spec_v.clone())?;
                spec.validate()?;
                self.spec = Some(spec);
                Ok(r#"{"ok":true}"#.to_string())
            }
            "generate" => {
                let spec = self.resolve_spec(&v)?;
                Ok(serde_json::to_string(&generate(&spec)?)?)
            }
            "generate_stream" => {
                let spec = self.resolve_spec(&v)?;
                let events = generate_stream(&spec)?;
                stream_json(&events)
            }
            "version" => Ok(format!(r#"{{"version":"{}"}}"#, Self::version())),
            other => Err(Error::BadSpec(format!("unknown cmd: {other}"))),
        }
    }

    /// Resolve the spec for a command: an inline `"spec"` field if present, else
    /// the handle's current spec.
    fn resolve_spec(&self, v: &Value) -> Result<GenSpec> {
        if let Some(spec_v) = v.get("spec") {
            let spec: GenSpec = serde_json::from_value(spec_v.clone())?;
            spec.validate()?;
            Ok(spec)
        } else {
            self.spec
                .clone()
                .ok_or_else(|| Error::BadSpec("no spec set".into()))
        }
    }
}

/// Build an `{"ok":false,"error":...}` JSON string with a properly escaped
/// message. Uses `Value`'s infallible `Display` so it cannot itself fail.
fn error_json(msg: &str) -> String {
    let escaped = Value::String(msg.to_string());
    format!(r#"{{"ok":false,"error":{escaped}}}"#)
}

#[cfg(test)]
mod tests {
    use super::Synth;
    use serde_json::Value;

    const SPEC: &str = r#"{ "seed": 42, "bars": 6, "start_price": 100.0,
        "regimes": [ { "kind": "trend", "len": 6, "drift": 0.002, "vol": 0.01 } ],
        "microstructure": { "book_depth": 3, "spread_bps": 4.0, "trade_rate": 3.0 } }"#;

    #[test]
    fn version_command() {
        let mut s = Synth::new("").unwrap();
        let r = s.command_json(r#"{"cmd":"version"}"#).unwrap();
        // Read the version rather than spelling it out: written as a literal,
        // this assertion failed on the first bump that touched it, which makes
        // the release the thing that discovers it. The shape is what is under
        // test here, and the shape is what a literal was proving.
        let expected = format!(r#"{{"version":"{}"}}"#, env!("CARGO_PKG_VERSION"));
        assert_eq!(r, expected);
    }

    #[test]
    fn set_then_generate() {
        let mut s = Synth::new("{}").unwrap();
        let set = s
            .command_json(&format!(r#"{{"cmd":"set_spec","spec":{SPEC}}}"#))
            .unwrap();
        assert_eq!(set, r#"{"ok":true}"#);
        let out = s.command_json(r#"{"cmd":"generate"}"#).unwrap();
        let v: Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["candles"].as_array().unwrap().len(), 6);
    }

    #[test]
    fn inline_spec_generate() {
        let mut s = Synth::new("").unwrap();
        let out = s
            .command_json(&format!(r#"{{"cmd":"generate","spec":{SPEC}}}"#))
            .unwrap();
        let v: Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["candles"].as_array().unwrap().len(), 6);
    }

    #[test]
    fn generate_stream_command() {
        let mut s = Synth::new("").unwrap();
        let out = s
            .command_json(&format!(r#"{{"cmd":"generate_stream","spec":{SPEC}}}"#))
            .unwrap();
        let v: Value = serde_json::from_str(&out).unwrap();
        assert!(v["events"].as_array().unwrap().len() >= 6);
    }

    #[test]
    fn stream_candle_keeps_struct_field_order() {
        // The streamed candle must serialize in the same field order as the
        // batch candle (ts, open, …), not alphabetized — the `json!` macro used
        // to round-trip events through `serde_json::Value` and reorder keys.
        let mut s = Synth::new("").unwrap();
        let out = s
            .command_json(&format!(r#"{{"cmd":"generate_stream","spec":{SPEC}}}"#))
            .unwrap();
        assert!(out.contains(r#"{"type":"candle","candle":{"ts":"#));
    }

    #[test]
    fn generate_without_spec_is_error_json() {
        let mut s = Synth::new("").unwrap();
        let out = s.command_json(r#"{"cmd":"generate"}"#).unwrap();
        let v: Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["ok"], false);
        assert!(v["error"].as_str().unwrap().contains("no spec"));
    }

    #[test]
    fn unknown_command_is_error_json() {
        let mut s = Synth::new("").unwrap();
        let out = s.command_json(r#"{"cmd":"nope"}"#).unwrap();
        let v: Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["ok"], false);
        assert!(v["error"].as_str().unwrap().contains("unknown cmd"));
    }

    #[test]
    fn bad_spec_is_error_json() {
        let mut s = Synth::new("").unwrap();
        let bad = r#"{"cmd":"generate","spec":{"seed":1,"bars":5,"start_price":100.0,
            "regimes":[{"kind":"trend","len":3,"drift":0.0,"vol":0.01}],
            "microstructure":{"book_depth":2,"spread_bps":1.0,"trade_rate":1.0}}}"#;
        let out = s.command_json(bad).unwrap();
        let v: Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["ok"], false);
    }

    #[test]
    fn malformed_json_is_error_json() {
        let mut s = Synth::new("").unwrap();
        let out = s.command_json("{not json").unwrap();
        let v: Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["ok"], false);
    }
}
