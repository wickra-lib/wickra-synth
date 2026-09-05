//! Deterministic synthetic market-microstructure engine.
//!
//! `wickra-synth-core` turns a data-driven [`GenSpec`] into OHLCV [`Candle`]s plus
//! [`BookSnapshot`]s, [`Trade`]s and [`FundingSample`]s. All randomness flows
//! through one portable PRNG ([`DetRng`], seeded by SplitMix64), and nowhere
//! else, so a given seed yields the **byte-identical** stream on every platform
//! and through every language binding.
//!
//! Two output shapes, one seed: [`generate`] returns the whole [`GenOutput`] in
//! one batch; [`generate_stream`] returns the same data as an ordered
//! [`Event`] list. Both run the same draws in the same order.
//!
//! The [`Synth`] handle exposes everything through a single
//! [`command_json`](Synth::command_json) boundary — the data API every language
//! binding forwards to.
//!
//! The PRNG is a **non-cryptographic** generator for reproducible simulation and
//! must never be used for any security purpose.

mod config;
mod error;
mod generate;
mod microstructure;
mod output;
mod rng;
mod spec;
mod synth;
mod walk;

pub use config::Config;
pub use error::{Error, Result};
pub use generate::{generate, generate_stream};
pub use output::{BookSnapshot, Candle, Event, FundingSample, GenOutput, Level, Side, Trade};
pub use rng::{mix, DetRng, SplitMix64};
pub use spec::{FundingSpec, GenSpec, Microstructure, Regime, RegimeKind};
pub use synth::Synth;
pub use walk::walk;

#[cfg(feature = "validate")]
pub use generate::sanity_check;

/// The crate version, as declared in `Cargo.toml`.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
