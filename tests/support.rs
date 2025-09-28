// tests/support.rs
use std::fs::File;
use std::path::Path;

use proof_transport::ast::Proof;
use serde_json::{from_reader, Result as JsonResult};

/// Parse a proof JSON file and return a Result so callers can `.expect(...)`.
/// Accepts &str, &Path, or anything that implements AsRef<Path>.
pub fn parse_proof<P: AsRef<Path>>(path: P) -> JsonResult<Proof> {
    let file = File::open(path.as_ref()).expect("open JSON");
    from_reader(file)
}

/// Back‑compat helper that panics on error and returns a Proof directly.
pub fn load(path: &str) -> Proof {
    parse_proof(path).expect("parse proof")
}
