// tests/golden.rs

use proof_transport::{
    ast::Proof,
    cut_eliminate_all,
    cut_eliminate_root,
    fragility_score,
    registry::{Registry, RuleId},
    validate_local_wf,
};

use serde_json::from_reader;
use std::fs::File;

#[test]
fn golden_example_runs() {
    // Load an example proof
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
