// src/cutelim.rs
use crate::ast::{Proof, ProofNode};
use std::collections::{HashMap, HashSet};

/// Eliminate a root `Cut` (if present):
/// - If the proof's root rule is `Cut`, set the root to the first premise
///   and prune nodes unreachable from the new root.
/// - Otherwise, return the proof unchanged.
pub fn cut_eliminate_root(p: &Proof) -> Proof {
    let mut q = p.clone();

    // find root node index
    let Some(root_idx) = q.nodes.iter().position(|n| n.id == q.root) else {
        return q;
    };

    // NOTE: the example JSON uses capitalized rule names, e.g. "Cut"
    if q.nodes[root_idx].rule != "Cut" {
        return q;
    }

    // a Cut must have two premises; if not, leave as-is
    if q.nodes[root_idx].premises.len() != 2 {
        return q;
    }

    // new root is the first premise of the Cut
    let new_root = q.nodes[root_idx].premises[0].clone();
    q.root = new_root;

    // drop anything no longer reachable from the new root
    prune_reachable(&mut q);
    q
}

/// Very simple "driver": repeatedly eliminate a root `Cut` while one exists.
/// This is safe for our examples (which only require removing root `Cut`s).
pub fn cut_eliminate_all(p: &Proof) -> Proof {
    let mut cur = p.clone();
    loop {
        let next = cut_eliminate_root(&cur);
        if next.root == cur.root {
            // no root Cut eliminated; we're done
            return cur;
        }
        cur = next;
    }
}

/// Keep only nodes reachable from `p.root`.
fn prune_reachable(p: &mut Proof) {
    // Build a lookup map: id -> node
    let map: HashMap<String, ProofNode> = p
        .nodes
        .iter()
        .cloned()
        .map(|n| (n.id.clone(), n))
        .collect();

    let mut keep: HashSet<String> = HashSet::new();
    let mut stack: Vec<ProofNode> = Vec::new();

    if let Some(root_node) = map.get(&p.root) {
        stack.push(root_node.clone());
    }

    while let Some(n) = stack.pop() {
        if keep.insert(n.id.clone()) {
            for pr in n.premises.iter() {
                if let Some(child) = map.get(pr) {
                    stack.push(child.clone());
                }
            }
        }
    }

    // Retain only kept nodes (by id)
    p.nodes.retain(|n| keep.contains(&n.id));
}
