use std::fs::File;
use std::io::Read;

use proof_transport::{ast::Proof, frag::fragility_score, validate_local_wf};

fn load_proof(path: &str) -> Proof {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let full = format!("{}/{}", manifest, path);
    let mut data = String::new();
    File::open(full)
        .expect("open example")
        .read_to_string(&mut data)
        .expect("read");
    serde_json::from_str(&data).expect("decode JSON")
}

#[test]
fn example_proof_is_valid_and_has_fragility() {
    let p = load_proof("examples/proof_with_cut.json");
    validate_local_wf(&p).expect("well-formed proof");
    assert!(fragility_score(&p) >= 1);
}
