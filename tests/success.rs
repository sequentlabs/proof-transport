use std::fs::File;
use std::io::Read;

use proof_transport::{ast::Proof, frag::fragility_score, validator::validate_local_wf};

fn load_proof(path: &str) -> Proof {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let mut file = File::open(format!("{}/{}", manifest, path)).expect("open example file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("read example file");
    serde_json::from_str(&buf).expect("decode JSON")
}

#[test]
fn lib_exports_compile() {
    let p = load_proof("examples/proof_with_cut.json");
    validate_local_wf(&p).expect("well-formed proof");
    let score = fragility_score(&p);
    assert!(score >= 1, "fragility score should be at least 1");
}
