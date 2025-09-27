// src/lib.rs — central library API for proof-transport

pub mod ast;
pub mod cutelim;
pub mod frag;
pub mod registry;
pub mod validator;
pub mod transport;

// Re-export key types and functions so downstream crates & tests can use directly
pub use ast::*;
pub use cutelim::{cut_eliminate_all, cut_eliminate_root};
pub use frag::fragility_score;
pub use registry::*;
pub use transport::{
    fragility_delta, transport, transport_simple, transport_with_cert, Certificate,
};
pub use validator::validate_local_wf;
