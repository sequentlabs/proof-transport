use proof_transport::{ast::Proof, cut_eliminate_all, validate_local_wf};
use serde_json::from_reader;
use std::fs;
use std::fs::File;
use std::path::Path;

fn try_load_proof(path: &Path) -> Option<Proof> {
    let f = File::open(path).ok()?;
    from_reader::<_, Proof>(f).ok()
}

#[test]
fn scan_examples_and_smoke_transport() {
    for entry in fs::read_dir("examples").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if entry.file_type().unwrap().is_file() && path.extension().map_or(false, |e| e == "json") {
            if let Some(p) = try_load_proof(&path) {
                // Local well-formedness and a transport smoke test.
                validate_local_wf(&p).unwrap();
                let _ = cut_eliminate_all(&p);
            }
        }
    }
}
