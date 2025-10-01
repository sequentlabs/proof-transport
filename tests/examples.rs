use serde_json::Value;
use std::fs;

#[test]
fn load_example_proofs() {
    for f in [
        "examples/proof_with_cut.json",
        "examples/proof_cut_eliminated.json",
        "examples/R.json",
    ] {
        let data = fs::read_to_string(f).expect("file should exist");
        let _: Value = serde_json::from_str(&data).expect("valid JSON");
    }
}
