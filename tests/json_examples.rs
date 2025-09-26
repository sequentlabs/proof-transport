// tests/json_examples.rs
use std::fs;
use serde_json::Value;

#[test]
fn all_example_jsons_are_valid() {
    // Absolute base to the crate root so paths work in CI and locally.
    let base = env!("CARGO_MANIFEST_DIR");

    let files = [
        "examples/proof_with_cut.json",
        "examples/proof_cut_eliminated.json",
        "examples/R.json",
    ];

    for f in files {
        let path = format!("{}/{}", base, f);
        let data = fs::read_to_string(&path).expect("file should exist");
        let _: Value = serde_json::from_str(&data).expect("valid JSON");
    }
}
