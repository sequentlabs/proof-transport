// tests/smoke.rs
use std::fs::File;
use serde_json::from_reader;

// NOTE: the crate name in tests is the package name with '-' changed to '_':
// `proof-transport` -> `proof_transport`
use proof_transport::{
    ast::Proof,
    frag::fragility_score,
    validator::validate_local_wf,
};

#[test]
fn loads_and_scores_example() {
    let base = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/examples/proof_with_cut.json", base);

    let p: Proof = from_reader(File::open(&path).expect("open example")).expect("decode JSON");
    validate_local_wf(&p).expect("well-formed proof");
    assert!(fragility_score(&p) >= 1, "fragility should be non-zero with Cut present");
}
