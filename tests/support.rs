<<<<<<< HEAD
=======
<<<<<<< HEAD
// tests/support.rs
>>>>>>> 479a615 (tests: restore support.rs; keep transport(_from) stable)
use std::fs::File;
use serde_json::from_reader;

use proof_transport::ast::Proof;

<<<<<<< HEAD
=======
/// Load a proof JSON from disk.  This is intentionally minimal: our
/// tolerant `Sequent` deserializer lives in `src/ast.rs`, so a normal
/// serde parse is enough for all examples.
=======
use std::fs::File;
use serde_json::from_reader;
use proof_transport::ast::Proof;
>>>>>>> acc7593 (tests: restore support.rs; keep transport(_from) stable)
>>>>>>> 479a615 (tests: restore support.rs; keep transport(_from) stable)
pub fn load(path: &str) -> Proof {
    from_reader(File::open(path).expect("open JSON")).expect("parse proof")
}
