use serde_json::from_reader;
use std::fs::File;

use proof_transport::ast::Proof;
use proof_transport::cutelim::{cut_eliminate_all, cut_eliminate_root};
use proof_transport::frag::fragility_score;
use proof_transport::validator::validate_local_wf;

fn load_proof(path: &str) -> Proof {
    from_reader(File::open(path).expect("open example")).expect("decode JSON")
}

#[test]
fn loads_and_scores_example() {
    let p = load_proof("examples/proof_with_cut.json");
    validate_local_wf(&p).expect("well-formed proof");
    assert!(
        fragility_score(&p) > 1,
        "fragility should be non-zero with Cut present"
    );
}

#[test]
fn cut_elimination_root_and_all_compile() {
    let p = load_proof("examples/proof_with_cut.json");
    let _ = cut_eliminate_root(&p);
    let _ = cut_eliminate_all(&p);
}
