use proof_transport::{ast::Proof, validator::validate_local_wf, frag::fragility_score, cutelim::cut_eliminate_root};
use std::fs::File;
use serde_json::from_reader;

#[test]
fn cut_elimination_rewrites_root_and_drops_fragility() {
    // BEFORE: Load the cut proof
    let p: Proof = from_reader(File::open("examples/proof_with_cut.json").unwrap()).unwrap();
    validate_local_wf(&p).unwrap();

    let before_score = fragility_score(&p);

    // Apply cut-elimination
    let q = cut_eliminate_root(&p);
    validate_local_wf(&q).unwrap();

    // Expect the root to become the first premise ("n1" in our example)
    assert_eq!(q.root, "n1");

    // Nodes should not increase (usually decrease)
    assert!(q.nodes.len() <= p.nodes.len());

    // Fragility must drop strictly (we removed the `Cut` root)
    let after_score = fragility_score(&q);
    assert!(after_score < before_score, "fragility did not drop: {} -> {}", before_score, after_score);
}
