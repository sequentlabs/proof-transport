use std::fs;
use serde_json::Value;

fn manifest_path(rel: &str) -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), rel)
}

#[test]
fn all_example_jsons_are_valid() {
    let files = [
        "examples/proof_with_cut.json",
        "examples/proof_cut_eliminated.json",
        "examples/R.json",
    ];

    for f in files {
        let data = fs::read_to_string(manifest_path(f)).expect("file should exist");
        let _: Value = serde_json::from_str(&data).expect("valid JSON");
    }
}
