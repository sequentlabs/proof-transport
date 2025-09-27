use std::fs::File;

use proof_transport::{ast::Proof, cut_eliminate_all, fragility_score, validate_local_wf};
use serde_json::from_reader;

#[test]
fn golden_example_runs() {
    // A concrete propositional example with a Cut at the root.
    let p: Proof = from_reader(File::open("examples/proof_with_cut.json").unwrap()).unwrap();
    validate_local_wf(&p).unwrap();

    let before = fragility_score(&p);
    let q = cut_eliminate_all(&p);
    validate_local_wf(&q).unwrap();
    let after = fragility_score(&q);

    // On this example the Cut is removed at the root, so fragility must strictly drop.
    assert!(after < before, "fragility did not drop: {} -> {}", before, after);
}
