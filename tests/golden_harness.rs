use proof_transport::{ast::Proof, cut_eliminate_all, validate_local_wf};
use serde_json::from_reader;
use std::{fs, path::Path};

#[test]
fn golden_transport_pairs() {
    // Walk every JSON file in examples/. If it parses as a Proof, check local WF and
    // that transport produces a locally well‑formed proof as well.
    for entry in fs::read_dir("examples").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if entry
            .file_type()
            .unwrap()
            .is_file()
            && path.extension().map_or(false, |e| e == "json")
        {
            if let Some(proof) = try_load_proof(&path) {
                // Local well‑formedness and a transport smoke test.
                validate_local_wf(&proof).unwrap();
                let transported = cut_eliminate_all(&proof);
                validate_local_wf(&transported).unwrap();
            }
        }
    }
}

fn try_load_proof(path: &Path) -> Option<Proof> {
    let file = std::fs::File::open(path).ok()?;
    from_reader::<_, Proof>(file).ok()
}
