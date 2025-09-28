use std::fs::File;
use serde_json::from_reader;

use proof_transport::ast::Proof;

/// Read a proof from a JSON file.
pub fn load(path: &str) -> Proof {
    from_reader(File::open(path).expect("open JSON")).expect("parse proof")
}
