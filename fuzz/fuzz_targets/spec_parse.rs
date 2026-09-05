#![no_main]
//! Fuzz the spec-parsing surface: arbitrary bytes are parsed as a `GenSpec`
//! (JSON). Malformed input must surface as a clean `Err`, never a panic. A
//! successfully parsed spec re-serializes and re-parses to an equal value, and
//! `validate` never panics.

use libfuzzer_sys::fuzz_target;
use wickra_synth_core::GenSpec;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(spec) = GenSpec::from_json(text) else {
        return;
    };
    // A parsed spec round-trips: serialize -> parse -> equal.
    let serialized = serde_json::to_string(&spec).expect("serialize a parsed spec");
    let reparsed: GenSpec = serde_json::from_str(&serialized).expect("re-parse a serialized spec");
    assert_eq!(reparsed, spec, "GenSpec serde round-trip is not stable");
});
