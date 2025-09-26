// src/lib.rs — central library API for proof-transport

pub mod ast;
pub mod registry;
pub mod validator;
pub mod frag;
pub mod cutelim;

// Re-export key types and functions so downstream crates & tests can use directly
pub use ast::*;
pub use registry::*;
pub use validator::validate_local_wf;
pub use frag::fragility_score;
pub use cutelim::{cut_eliminate_root, cut_eliminate_all};
