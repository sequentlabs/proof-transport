// tests/success.rs
//
// Purpose: "does the public API actually work end-to-end?"
//
// This test loads a real example proof, calls the public entry points
// (fragility scoring and validator), and asserts basic properties.
// No function-pointer casts here, so we avoid brittle typing issues.

use std::fs::File;
use std::io::Read;

use proof_transport::{
    ast::Proof,
    frag::fragility_score,
    validator::validate_local_wf,
};

fn load_proof(path: &str) -> Proof {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let full = format!("{}/{}", manifest, path);

    let mut s = String::new();
    File::open(&full)
        .expect("open example JSON")
        .read_to_string(&mut s)
        .expect("read example JSON");

    serde_json::from_str(&s).expect("decode example JSON")
}

#[test]
fn public_api_works_on_example() {
    // use the same toy example the other tests use
    let p = load_proof("examples/proof_with_cut.json");

    // validator should succeed
    validate_local_wf(&p).expect("validator should accept the example");

    // scoring should be defined and non-zero for the example with a root Cut
    let score = fragility_score(&p);
    assert!(
        score >= 1,
        "fragility score should be >= 1 for a proof with a root Cut; got {}",
        score
    );
}
