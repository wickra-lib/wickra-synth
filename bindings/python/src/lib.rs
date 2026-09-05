//! Python bindings for `wickra-synth`, exposed under the
//! `wickra_synth` package.
//!
//! Thin glue over the synth core's data-driven surface: build a
//! [`Synth`] from a spec JSON, drive it with a command JSON and read back
//! the response JSON. The same command protocol crosses every binding, so a
//! Python front-end drives the exact same core as the native CLI.

// PyO3 protocol methods take `self` by value/ref regardless of use.
#![allow(clippy::needless_pass_by_value)]

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use wickra_synth_core::Synth;

/// A synth driven by JSON commands.
///
/// `unsendable`: the handle holds a stateful generator, so it is bound to the
/// thread that created it.
#[pyclass(name = "Synth", unsendable)]
struct PySynth {
    inner: Synth,
}

#[pymethods]
impl PySynth {
    /// Build a synth from a spec JSON string.
    #[new]
    fn new(spec_json: &str) -> PyResult<Self> {
        Synth::new(spec_json)
            .map(|inner| Self { inner })
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    /// Apply a command JSON and return the resulting response JSON.
    fn command(&mut self, cmd_json: &str) -> PyResult<String> {
        self.inner
            .command_json(cmd_json)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    /// The library version.
    #[staticmethod]
    fn version() -> &'static str {
        Synth::version()
    }
}

/// The native module (`wickra_synth._wickra_synth`).
#[pymodule]
fn _wickra_synth(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_class::<PySynth>()?;
    Ok(())
}
