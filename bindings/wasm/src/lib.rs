//! WebAssembly bindings for `wickra-synth` (wasm-bindgen).
//!
//! Generate synthetic market microstructure in the browser: create a `Synth`
//! from a spec JSON, drive it with a command JSON (`set_spec`, `generate`,
//! `generate_stream`, `version`) and read back the response JSON. The same
//! command protocol crosses every binding, so a browser front-end runs against
//! the exact same core as the native CLI.
//!
//! The generator runs single-threaded here (no worker thread pool in a browser
//! sandbox), which is byte-identical to the native run — the exact
//! cross-language golden check.

use wasm_bindgen::prelude::*;

use wickra_synth_core::Synth as CoreSynth;

/// A synth driven by JSON commands.
#[wasm_bindgen]
pub struct Synth {
    inner: CoreSynth,
}

#[wasm_bindgen]
impl Synth {
    /// Build a synth from a spec JSON string.
    #[wasm_bindgen(constructor)]
    pub fn new(spec_json: &str) -> Result<Synth, JsError> {
        CoreSynth::new(spec_json)
            .map(|inner| Self { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Apply a command JSON (`{"cmd":"...", ...}`) and return the response JSON.
    pub fn command(&mut self, cmd_json: &str) -> Result<String, JsError> {
        self.inner
            .command_json(cmd_json)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// The library version.
    #[wasm_bindgen(js_name = version)]
    pub fn instance_version(&self) -> String {
        CoreSynth::version().to_string()
    }
}

/// The library version.
#[wasm_bindgen]
pub fn version() -> String {
    CoreSynth::version().to_string()
}
