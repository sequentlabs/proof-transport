use serde_json::Value;
use std::fs;

#[test]
fn all_example_jsons_are_valid() {
    for f in [
        "examples/proof_with_cut.json",
        "examples/proof_cut_eliminated.json",
        "examples/R.json",
    ] {
        let data = fs::read_to_string(f).expect("read file");
        let _: Value = serde_json::from_str(&data).expect("valid JSON");
    }
}
