use proof_transport::{ast::Proof, validator::validate_local_wf, frag::fragility_score};
use std::fs::File;
use serde_json::from_reader;

#[test]
fn loads_and_scores_example() {
    let p: Proof = from_reader(File::open("examples/proof_with_cut.json").unwrap()).unwrap();
    validate_local_wf(&p).unwrap();
    assert!(fragility_score(&p) >= 1);
}
