// tests/support.rs
use std::fs::File;
use serde_json::from_reader;

use proof_transport::ast::Proof;

/// Load a proof JSON from disk.  This is intentionally minimal: our
/// tolerant `Sequent` deserializer lives in `src/ast.rs`, so a normal
/// serde parse is enough for all examples.
pub fn load(path: &str) -> Proof {
    from_reader(File::open(path).expect("open JSON")).expect("parse proof")
}
