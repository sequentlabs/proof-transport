use std::{fs, path::Path};

use proof_transport::{ast::Proof, cut_eliminate_all, fragility_score, validate_local_wf};

/// Try to load a `Proof` from `path`.  
/// Returns:
/// - `Ok(Some(proof))` if JSON parses as a `Proof`
/// - `Ok(None)` if it’s valid JSON but *not* a `Proof`
/// - `Err(io::Error)` for filesystem errors (file missing, permissions, etc.)
fn try_load_proof(path: &Path) -> std::io::Result<Option<Proof>> {
    let f = fs::File::open(path)?;
    let parsed: Result<Proof, _> = serde_json::from_reader(f);
    Ok(parsed.ok())
}

#[test]
fn examples_are_well_formed_and_transport_smoke() {
    // Scan every .json in examples/. If it parses as a Proof, give it a light touch:
    //  - local well‑formedness
    //  - cut elimination must not worsen fragility (monotone: after <= before)
    for entry in fs::read_dir("examples").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if entry.file_type().unwrap().is_file()
            && path
                .extension()
                .and_then(|s| s.to_str())
                .map_or(false, |e| e.eq_ignore_ascii_case("json"))
        {
            if let Ok(Some(p)) = try_load_proof(&path) {
                // Local well‑formedness.
                validate_local_wf(&p).unwrap();

                // Cut elimination should never increase fragility.
                let before = fragility_score(&p);
                let q = cut_eliminate_all(&p);
                validate_local_wf(&q).unwrap();
                let after = fragility_score(&q);

                assert!(
                    after <= before,
                    "fragility did not drop or stay the same for {}: {} -> {}",
                    path.display(),
                    before,
                    after
                );
            }
        }
    }
}
