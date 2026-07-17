#![no_main]
//! Fuzz the `command_json` FFI boundary that every binding calls. A `Synth` is
//! built from a fixed valid spec, then driven with arbitrary command bytes.
//! `command_json` is total: malformed or unknown commands come back in-band as
//! `{"ok":false,...}` JSON — it must never panic and never return `Err`.

use libfuzzer_sys::fuzz_target;
use synth_core::Synth;

const SPEC: &str = r#"{ "seed": 42, "bars": 8, "start_price": 100.0,
    "regimes": [{ "kind": "trend", "len": 8, "drift": 0.002, "vol": 0.01 }],
    "microstructure": { "book_depth": 3, "spread_bps": 4.0, "trade_rate": 3.0 } }"#;

fuzz_target!(|data: &[u8]| {
    let Ok(cmd) = std::str::from_utf8(data) else {
        return;
    };
    let mut synth = Synth::new(SPEC).expect("the fixed spec is valid");
    // Never errs: internal failures are encoded in the returned JSON.
    let response = synth.command_json(cmd).expect("command_json is total");
    // The response is always valid JSON.
    let _: serde_json::Value = serde_json::from_str(&response).expect("response is valid JSON");
});
