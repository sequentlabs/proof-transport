use proof_transport::{ast::Proof, frag::fragility_score};
use serde_json::from_reader;
use std::fs::File;

#[test]
fn fragility_score_nonzero_on_cut_proof() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/examples/proof_with_cut.json", manifest);
    let p: Proof = from_reader(File::open(path).expect("open")).expect("decode");
    assert!(fragility_score(&p) >= 1);
}
