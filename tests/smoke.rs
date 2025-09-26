use std::fs::File;
use serde_json::from_reader;

use proof_transport::{
    ast::Proof,
    cutelim::cut_eliminate_all,
    frag::fragility_score,
    validator::validate_local_wf,
};

/// tiny helper: read a `Proof` from a file
fn read_proof(path: &str) -> Proof {
    let full = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path);
    let f = File::open(full).expect("open example json");
    from_reader::<_, Proof>(f).expect("deserialize proof")
}

#[test]
fn loads_and_scores_example() {
    let p = read_proof("examples/proof_with_cut.json");
    validate_local_wf(&p).expect("wf");
    let s = fragility_score(&p);
    assert!(s >= 1, "expected non-zero fragility score, got {s}");
}

#[test]
fn cut_elimination_rewrites_root_and_drops_fragility() {
    let p = read_proof("examples/proof_with_cut.json");

    let before = fragility_score(&p);
    let p2 = cut_eliminate_all(&p);
    validate_local_wf(&p2).expect("wf after cut-elim");
    let after = fragility_score(&p2);

    // toy guarantee for now: fragility should not increase, and root changes
    assert!(after <= before, "fragility did not drop: before={before}, after={after}");
    assert_ne!(p2.root, p.root, "root was expected to change after cut-elim");
}
