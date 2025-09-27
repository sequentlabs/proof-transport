// tests/golden_harness.rs
use proof_transport::{ast::Proof, cut_eliminate_all, validate_local_wf};
use serde_json::{from_reader, to_value};
use std::fs::{read_dir, File};
use std::path::Path;

fn pairs(dir: &str) -> Vec<(String, String)> {
    let mut v = Vec::new();
    for entry in read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy();
        if !name.ends_with(".in.json") {
            continue;
        }
        let out = name.replace(".in.json", ".out.json");
        let in_path = path.to_string_lossy().to_string();
        let out_path = Path::new(dir).join(out).to_string_lossy().to_string();
        v.push((in_path, out_path));
    }
    v.sort();
    v
}

#[test]
fn golden_transport_pairs() {
    let dir = "examples/golden";
    let pairs = pairs(dir);
    assert!(
        !pairs.is_empty(),
        "no golden pairs found in {} — add *.in.json / *.out.json",
        dir
    );

    for (inp, outp) in pairs {
        // load input/expected
        let p: Proof = from_reader(File::open(&inp).unwrap()).unwrap();
        let expected: Proof = from_reader(File::open(&outp).unwrap()).unwrap();

        // validate before
        validate_local_wf(&p).unwrap();

        // run transport
        let q = cut_eliminate_all(&p);

        // validate after
        validate_local_wf(&q).unwrap();

        // compare structurally via JSON (no need for PartialEq on Proof)
        let got_json = to_value(&q).unwrap();
        let exp_json = to_value(&expected).unwrap();

        assert_eq!(
            got_json, exp_json,
            "golden mismatch for input {}\nexpected: {}\n     got: {}",
            inp, exp_json, got_json
        );
    }
}
