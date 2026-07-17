//! Error type for the synth core.

/// An error produced while parsing a spec/command or generating output.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A JSON or TOML document failed to parse.
    #[error("parse: {0}")]
    Parse(String),
    /// A spec parsed but violated an invariant (see [`crate::GenSpec`] validation).
    #[error("bad spec: {0}")]
    BadSpec(String),
    /// The generation produced a non-finite value (should never happen for a
    /// valid spec; surfaced instead of writing a silent `NaN`/`inf`).
    #[error("numeric: {0}")]
    Numeric(String),
}

/// Result alias for the synth core.
pub type Result<T> = core::result::Result<T, Error>;

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Parse(e.to_string())
    }
}
