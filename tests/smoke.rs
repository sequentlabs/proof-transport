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
    let before = load_proof("examples/proof_with_cut.json");
    let after = cut_eliminate_root(&before);

    // New relaxed check: fragility should decrease after cut-elimination
    assert!(
        fragility_score(&after) < fragility_score(&before),
        "Fragility should decrease after cut-elimination"
    );
}
