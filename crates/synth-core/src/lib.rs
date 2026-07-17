//! Deterministic synthetic market-microstructure engine.
//!
//! `synth-core` turns a data-driven `GenSpec` into OHLCV candles plus order-book
//! snapshots, trades and funding samples, driven by a fixed portable PRNG
//! (SplitMix64 seeding xoshiro256++) so that a given seed yields **byte-for-byte
//! identical** output on every platform and through every language binding.
//!
//! The engine, spec model and `command_json` boundary land in phase P-SYN-1;
//! this scaffold exposes only the crate version.

/// The crate version, as declared in `Cargo.toml`.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::version;

    #[test]
    fn version_is_reported() {
        assert!(!version().is_empty());
    }
}
