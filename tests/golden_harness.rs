// tests/golden_harness.rs
use proof_transport::{
    ast::Proof,
    cutelim::cut_eliminate_all,
    fragility_score,
    validate_local_wf,
};

use serde_json::{from_reader, Value};
use std::{fs::File, fs, path::{Path, PathBuf}};

/// Collect candidate JSON proofs to run. We always include the known-good example,
/// and then opportunistically scan tests/golden/pairs/*/{before.json,after.json}.
fn collect_candidates() -> Vec<PathBuf> {
    let mut out = Vec::new();

    // Always include the repo example.
    let example = Path::new("examples/proof_with_cut.json");
    if example.exists() {
        out.push(example.to_path_buf());
    }

    // Optional: scan golden pairs if present.
    let pairs_root = Path::new("tests/golden/pairs");
    if pairs_root.exists() {
        if let Ok(entries) = fs::read_dir(pairs_root) {
            for e in entries.flatten() {
                let dir = e.path();
                if dir.is_dir() {
                    for name in ["before.json", "after.json"] {
                        let p = dir.join(name);
                        if p.exists() {
                            out.push(p);
                        }
                    }
                }
            }
        }
    }

    out
}

/// Check a JSON has the *shape* of a `Proof`: top-level "root" (string) and "nodes" (array).
fn looks_like_proof_json(path: &Path) -> bool {
    let f = match File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    let v: Value = match from_reader(f) {
        Ok(v) => v,
        Err(_) => return false,
    };

    v.get("root").is_some() && v.get("nodes").map(|n| n.is_array()).unwrap_or(false)
}

#[test]
fn golden_transport_pairs() {
    let candidates = collect_candidates();

    // Filter to files that actually look like Proof jsons.
    let mut ran = 0usize;
    for path in candidates {
        if !looks_like_proof_json(&path) {
            // Skip configs or other JSON files that are not proofs.
            continue;
        }

        // Now deserialize as Proof.
        let p: Proof = from_reader(File::open(&path).unwrap_or_else(|e| {
            panic!("failed to open {}: {e}", path.display())
        }))
        .unwrap_or_else(|e| {
            panic!("failed to parse {} as Proof: {e}", path.display())
        });

        // Local well-formedness must hold.
        validate_local_wf(&p).unwrap_or_else(|e| {
            panic!("validate_local_wf failed on {}: {e:?}", path.display())
        });

        // Transport and re‑validate.
        let before = fragility_score(&p);
        let q = cut_eliminate_all(&p);
        validate_local_wf(&q).unwrap_or_else(|e| {
            panic!("validate_local_wf(after) failed on {}: {e:?}", path.display())
        });

        let after = fragility_score(&q);
        assert!(
            after < before,
            "fragility did not drop for {}: {} -> {}",
            path.display(),
            before,
            after
        );

        ran += 1;
    }

    // We should have run at least once (the example is always there).
    assert!(ran > 0, "no proof candidates found to run");
}
