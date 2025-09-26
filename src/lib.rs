// src/lib.rs — re-export library API for tests and downstream crates
pub mod ast;
pub mod registry;
pub mod validator;
pub mod frag;
pub mod cutelim;

// Re-export commonly used items at the crate root so tests can use
// `proof_transport::{ast::Proof, frag::fragility_score, validator::validate_local_wf, cutelim::cut_eliminate_all}`.

pub use ast::*;
pub use registry::*;
pub use validator::*;
pub use frag::*;
pub use cutelim::*;
