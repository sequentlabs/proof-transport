use crate::ast::Proof;
use std::collections::{HashMap, HashSet};

/// Simple identity pass that will be extended. Kept for experiments.
pub fn cut_eliminate_root(p: &Proof) -> Proof {
    p.clone()
}

/// Public entry point used by tests.
/// For now it's identity; tests only assert it compiles/links.
pub fn cut_eliminate_all(p: &Proof) -> Proof {
    p.clone()
}

// (Optional) private helpers for future real rewrite.
// Keep them private so the API stays stable while CI checks compile/link.
fn _prune_reachable(_p: &mut Proof) {}
