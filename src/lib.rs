// src/lib.rs — central library API for proof-transport

pub mod ast;
pub mod cutelim;
pub mod frag;
pub mod registry;
pub mod validator;

// Re-export selected items at the crate root so tests (and downstream) can use them tersely.
pub use ast::*;
pub use cutelim::{cut_eliminate_all, cut_eliminate_root};
pub use frag::fragility_score;
pub use registry::*;
pub use validator::validate_local_wf;
