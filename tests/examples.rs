use std::fs;
use serde_json::Value;

#[test]
fn load_example_proofs() {
    let files = [
        "docs/data/proof_with_cut.json",
        "docs/data/proof_cut_eliminated.json",
        "docs/data/proof_with_cut_2.json",
        "docs/data/proof_cut_eliminated_2.json",
    ];

    for f in files {
        let data = fs::read_to_string(f).expect("file should exist");
        let _: Value = serde_json::from_str(&data).expect("valid JSON");
    }
}
