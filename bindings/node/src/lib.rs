//! Node.js bindings for `wickra-synth` via napi-rs.
//!
//! A `Synth` is built from a spec JSON; `command` takes a request JSON and
//! returns the response JSON, so Node drives the exact same byte-identical
//! surface as every other binding.

use napi_derive::napi;

/// A synth driven by JSON commands.
#[napi]
pub struct Synth(synth_core::Synth);

#[napi]
impl Synth {
    /// Build a synth from a spec JSON string.
    #[napi(constructor)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(spec_json: String) -> napi::Result<Self> {
        synth_core::Synth::new(&spec_json)
            .map(Synth)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// Apply a command envelope (`{"cmd":"...", ...}`) and return the response JSON.
    #[napi]
    #[allow(clippy::needless_pass_by_value)]
    pub fn command(&mut self, cmd_json: String) -> napi::Result<String> {
        self.0
            .command_json(&cmd_json)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// The crate version.
    #[napi]
    pub fn version(&self) -> &'static str {
        synth_core::Synth::version()
    }
}
