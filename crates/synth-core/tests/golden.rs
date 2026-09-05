//! The golden invariant, from Rust: every `golden/specs/*.json` regenerates its
//! `golden/expected/*.json` byte-for-byte. `serde_json::to_string(&generate(..))`
//! is exactly what the CLI's `--format json` and every language binding produce,
//! so this file is the reference the whole cross-language corpus is pinned to.

use std::fs;
use std::path::PathBuf;

use wickra_synth_core::{generate, GenSpec};

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../golden")
}

#[test]
fn every_spec_matches_its_expected_output() {
    let dir = golden_dir();
    let mut checked = 0;
    let mut specs: Vec<_> = fs::read_dir(dir.join("specs"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    specs.sort();

    for spec_path in specs {
        let stem = spec_path.file_stem().unwrap().to_str().unwrap();
        let spec_json = fs::read_to_string(&spec_path).unwrap();
        let spec = GenSpec::from_json(&spec_json).unwrap();
        let got = serde_json::to_string(&generate(&spec).unwrap()).unwrap();

        let expected_path = dir.join("expected").join(format!("{stem}.json"));
        let expected = fs::read_to_string(&expected_path)
            .unwrap_or_else(|e| panic!("read {}: {e}", expected_path.display()));

        assert_eq!(
            got,
            expected.trim_end(),
            "golden mismatch for {stem}: the core output no longer matches the \
             committed fixture (re-bless if the change is intended)"
        );
        checked += 1;
    }
    assert_eq!(checked, 5, "expected 5 golden specs, checked {checked}");
}
