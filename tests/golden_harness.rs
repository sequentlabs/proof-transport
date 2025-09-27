use proof_transport::{ast::Proof, cut_eliminate_all, fragility_score, validate_local_wf};
use serde_json::from_reader;
use std::{fs, io, path::Path};

fn try_load_proof(path: &Path) -> io::Result<Option<Proof>> {
    let file = fs::File::open(path)?;
    // If this JSON isn’t a Proof, treat it as a soft skip.
    let parsed = from_reader::<_, Proof>(file).ok();
    Ok(parsed)
}

#[test]
fn scan_examples_and_run_canonical_example() {
    // Always run the canonical example and assert fragility drops.
    let p: Proof = from_reader(fs::File::open("examples/proof_with_cut.json").unwrap()).unwrap();
    validate_local_wf(&p).unwrap();
    let before = fragility_score(&p);
    let q = cut_eliminate_all(&p);
    validate_local_wf(&q).unwrap();
    let after = fragility_score(&q);
    assert!(after < before, "fragility did not drop: {} -> {}", before, after);

    // Robustly scan every JSON in examples/: if it parses as a Proof, give it a light touch.
    for entry in fs::read_dir("examples").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if entry
            .file_type()
            .unwrap()
            .is_file()
            && path.extension().map_or(false, |e| e == "json")
        {
            if let Ok(Some(proof)) = try_load_proof(&path) {
                // Local well-formedness and a transport smoke test.
                validate_local_wf(&proof).unwrap();
                let _ = cut_eliminate_all(&proof);
            }
        }
    }
}
