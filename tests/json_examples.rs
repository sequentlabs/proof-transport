// tests/json_examples.rs
use proof_transport::ast::Proof;
use std::fs;
use std::path::{Path, PathBuf};

/// Recursively collect `*.json` example files under `examples/`.
pub fn example_paths() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        if let Ok(rd) = fs::read_dir(dir) {
            for entry in rd.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    walk(&p, out);
                } else if p.extension().map_or(false, |e| e == "json") {
                    out.push(p);
                }
            }
        }
    }
    let mut v = Vec::new();
    walk(Path::new("examples"), &mut v);
    v.sort();
    v
}

/// Parse a proof from a file, trying strict JSON first, then JSON5 as a fallback.
/// Returns the original serde_json::Error if both fail, so callers' error types remain the same.
pub fn parse_proof(path: &Path) -> Result<Proof, serde_json::Error> {
    let s = fs::read_to_string(path).expect("failed to read example file");
    match serde_json::from_str::<Proof>(&s) {
        Ok(p) => Ok(p),
        Err(e_json) => match json5::from_str::<Proof>(&s) {
            Ok(p) => Ok(p),
            Err(_e_json5) => Err(e_json),
        },
    }
}
