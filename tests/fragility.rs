use proof_transport::{ast::Proof, frag::fragility_score};
use std::fs::File;
use serde_json::from_reader;

#[test]
fn fragility_score_nonzero_on_cut_proof() {
    let p: Proof = from_reader(File::open("examples/proof_with_cut.json").unwrap()).unwrap();
    assert!(fragility_score(&p) >= 1);
}
