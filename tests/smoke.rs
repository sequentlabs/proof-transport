use serde_json::from_reader;
use std::fs::File;

use proof_transport::ast::Proof;
use proof_transport::cutelim::{cut_eliminate_all, cut_eliminate_root};
use proof_transport::frag::fragility_score;
use proof_transport::validate_local_wf;

fn load_proof(path: &str) -> Proof {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let full = format!("{}/{}", manifest, path);
    from_reader(File::open(full).expect("open example")).expect("decode JSON")
}

#[test]
fn loads_and_scores_example() {
    let p: Proof = load_proof("examples/proof_with_cut.json");
    validate_local_wf(&p).expect("well-formed proof");
    assert!(fragility_score(&p) >= 1, "fragility should be at least 1");
}

#[test]
fn lib_exports_compile() {
    // Ensure functions are reachable from the crate with the expected signatures.
    let _: fn(&Proof) -> u64 = fragility_score;
    let _: fn(&Proof) -> anyhow::Result<()> = validate_local_wf;
    let _: fn(&Proof) -> Proof = cut_eliminate_root;
    let _: fn(&Proof) -> Proof = cut_eliminate_all;
}
