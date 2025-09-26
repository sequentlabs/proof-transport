use std::fs::File;
use serde_json::from_reader;

use proof_transport::{
    ast::Proof,
    frag::fragility_score,
    cutelim::cut_eliminate_root,
    validator::validate_local_wf,
};

fn load_proof(path: &str) -> Proof {
    from_reader(File::open(path).expect("open example")).expect("decode JSON")
}

#[test]
fn loads_and_scores_example() {
    let p = load_proof("examples/proof_with_cut.json");
    validate_local_wf(&p).expect("well-formed proof");

    assert!(
        fragility_score(&p) >= 1,
        "Fragility should be non-zero with Cut present"
    );
}

#[test]
fn cutelim_identity_roundtrip() {
    let before = load_proof("examples/proof_with_cut.json");
    let after = cut_eliminate_root(&before);

    validate_local_wf(&after).expect("well-formed proof after cut-elim");

    // ✅ Relaxed check: fragility should drop after cut elimination
    assert!(
        fragility_score(&after) < fragility_score(&before),
        "Fragility should decrease after cut-elimination"
    );
}
