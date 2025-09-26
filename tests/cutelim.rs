use proof_transport::{
    ast::Proof, cutelim::cut_eliminate_root, frag::fragility_score, validate_local_wf,
};
use serde_json::from_reader;
use std::fs::File;

#[test]
fn cut_elimination_rewrites_root_and_drops_fragility() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let before_path = format!("{}/examples/proof_with_cut.json", manifest);
    let after_path = format!("{}/examples/proof_cut_eliminated.json", manifest);

    let p: Proof = from_reader(File::open(before_path).expect("open input")).expect("decode");
    validate_local_wf(&p).expect("well-formed proof");
    let before_score = fragility_score(&p);

    let q = cut_eliminate_root(&p);
    validate_local_wf(&q).expect("still well-formed");

    // Root should match the example 'cut eliminated' proof.
    let expected: Proof =
        from_reader(File::open(after_path).expect("open expected")).expect("decode");
    assert_eq!(q.root, expected.root);

    // Fragility must drop strictly (we removed the `Cut` root)
    let after_score = fragility_score(&q);
    assert!(
        after_score < before_score,
        "fragility did not drop: {} -> {}",
        before_score,
        after_score
    );
}
