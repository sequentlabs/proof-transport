// src/lib.rs — central library API

pub mod ast;
pub mod registry;
pub mod validator;
pub mod frag;
pub mod cutelim;

// Re-export commonly used items at the crate root
pub use ast::Proof;
pub use registry::*;
pub use validator::validate_local_wf;
pub use frag::fragility_score;
pub use cutelim::{cut_eliminate_root, cut_eliminate_all};
