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

    // Run both elimination functions to verify consistency
    let q_all = cut_eliminate_all(&p);
    let q_root = cut_eliminate_root(&p);

    validate_local_wf(&q_all).unwrap();
    validate_local_wf(&q_root).unwrap();

    let after_all = fragility_score(&q_all);
    let after_root = fragility_score(&q_root);

    assert!(
        after_all < before,
        "fragility did not drop (cut_eliminate_all): {} -> {}",
        before,
        after_all
    );

    assert!(
        after_root < before,
        "fragility did not drop (cut_eliminate_root): {} -> {}",
        before,
        after_root
    );
}
