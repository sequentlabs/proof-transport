use proof_transport::{
    ast::Proof, cutelim::cut_eliminate_all, frag::fragility_score, validator::validate_local_wf,
};
use serde_json::from_reader;
use std::fs::File;

#[test]
fn cut_elimination_rewrites_root_and_drops_fragility() {
    let p: Proof = from_reader(File::open("examples/proof_with_cut.json").unwrap()).unwrap();
    validate_local_wf(&p).unwrap();

    let before_score = fragility_score(&p);
    let q = cut_eliminate_all(&p);
    validate_local_wf(&q).unwrap();

    // Fragility must drop strictly (we removed the `Cut` root)
    let after_score = fragility_score(&q);
    assert!(
        after_score < before_score,
        "fragility did not drop: {} -> {}",
        before_score,
        after_score
    );
}
