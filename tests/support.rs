use std::fs::File;
use serde_json::from_reader;

use proof_transport::ast::Proof;

/// Read a proof from a JSON file (the name older tests expect).
pub fn parse_proof(path: &str) -> Proof {
    from_reader(File::open(path).expect("open JSON")).expect("parse proof")
}

/// Newer name used in other tests; keep both for compatibility.
pub use parse_proof as load;
