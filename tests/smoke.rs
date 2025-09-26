use proof_transport::{
    ast::Proof,
    frag::fragility_score,
    validator::validate_local_wf,
    cutelim::cut_eliminate_all,
};
use std::fs::File;
use serde_json::from_reader;

#[test]
fn loads_and_scores_example() {
    let p: Proof = from_reader(File::open("./examples/proof_with_cut.json").unwrap()).unwrap();
    validate_local_wf(&p).unwrap();
    assert!(fragility_score(&p) >= 1);
}

#[test]
fn cutelim_identity_roundtrip() {
    let p: Proof = from_reader(File::open("./examples/proof_with_cut.json").unwrap()).unwrap();
    let q = cut_eliminate_all(&p);
    // For now, should just equal input (identity placeholder)
    assert_eq!(p.nodes.len(), q.nodes.len());
}
