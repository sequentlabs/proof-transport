use std::fs;
use serde_json::Value;

#[test]
fn load_example_proofs() {
    let files = [
        "examples/proof_with_cut.json",
        "examples/proof_cut_eliminated.json",
        "examples/R.json",
    ];

    for f in files {
        let data = fs::read_to_string(f).expect("file should exist");
        let _: Value = serde_json::from_str(&data).expect("valid JSON");
    }
}
