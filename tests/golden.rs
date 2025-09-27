use proof_transport::{ast::Proof, cut_eliminate_all, fragility_score, validate_local_wf};
use serde_json::from_reader;
use std::fs::File;

#[test]
fn golden_example_runs() {
    // Load the canonical example that contains a Cut at the root.
    let p: Proof = from_reader(File::open("examples/proof_with_cut.json").unwrap()).unwrap();
    validate_local_wf(&p).unwrap();

    let before = fragility_score(&p);
    let q = cut_eliminate_all(&p);
    validate_local_wf(&q).unwrap();

    let after = fragility_score(&q);
    assert!(
        after < before,
        "fragility did not drop: {} -> {}",
        before,
        after
    );
}
