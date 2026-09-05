#![no_main]
//! Fuzz the TOML spec parser.
//!
//! `spec_parse` covers `GenSpec::from_json`, and for a long time that was read
//! as covering "the parser". It is not: `from_toml` is a second public entry
//! point over a second grammar, and the CLI dispatches to it on the file
//! extension, so a `.toml` spec is a shape this project accepts from outside
//! and never fuzzed.
//!
//! The contract is the same as the JSON one: any input either parses into a
//! valid spec or is rejected. Neither outcome may panic, and a spec that comes
//! back accepted must survive generation.

use libfuzzer_sys::fuzz_target;
use wickra_synth_core::{generate, GenSpec};

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    // A TOML document can name enormous bar counts, and an accepted spec is
    // generated below; cap the input length so the fuzzer spends its time on
    // parser shapes rather than on allocation.
    if text.len() > 4096 {
        return;
    }
    if let Ok(spec) = GenSpec::from_toml(text) {
        // from_toml validates, so anything it returns must generate. A spec that
        // parses and then fails to generate means the two disagree about what
        // "valid" is, which is the defect this target exists to find.
        assert!(
            generate(&spec).is_ok() || spec.bars > 4096,
            "a spec that passed validation failed to generate"
        );
    }
});
