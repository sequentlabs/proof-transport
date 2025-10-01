// tests/support.rs
use std::{fs::File, io::Read, path::Path};

use proof_transport::ast::Proof;

/// Parse a proof file.
///
/// Strategy:
///   1) try strict JSON (serde_json) from a file reader
///   2) if that fails, read the file to a string and try:
///        a) strict `serde_json::from_str` (handles some edge cases)
///        b) permissive `json5::from_str` (allows comments/trailing commas)
pub fn parse_proof<P: AsRef<Path>>(path: P) -> anyhow::Result<Proof> {
    let path = path.as_ref();

    // 1) strict via reader
    if let Ok(file) = File::open(path) {
        if let Ok(p) = serde_json::from_reader::<_, Proof>(file) {
            return Ok(p);
        }
    }

    // 2) read whole file and try again (strict, then permissive)
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    if let Ok(p) = serde_json::from_str::<Proof>(&s) {
        return Ok(p);
    }
    if let Ok(p) = json5::from_str::<Proof>(&s) {
        return Ok(p);
    }

    // If both fail, return the strict error message
    let file = File::open(path)?;
    let p: Proof = serde_json::from_reader(file)?;
    Ok(p)
}

/// Back‑compat helper: panic on error and return `Proof` directly.
pub fn load(path: &str) -> Proof {
    parse_proof(path).expect("parse proof")
}
