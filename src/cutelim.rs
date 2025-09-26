// src/cutelim.rs
//
// Simple cut-elimination used by the tests:
// - If the root is a Cut with two premises, replace the root with the
//   first premise and drop all nodes unreachable from the new root.
// - Repeat until no more root-level cuts remain.

use crate::ast::{Proof, ProofNode};
use std::collections::{HashMap, HashSet};

/// Keep only nodes reachable from `p.root`.
fn prune_reachable(p: &mut Proof) {
    // Snapshot the graph so we can traverse without borrowing `p.nodes`.
    let map: HashMap<String, ProofNode> = p
        .nodes
        .iter()
        .cloned()
        .map(|n| (n.id.clone(), n))
        .collect();

    let mut keep: HashSet<String> = HashSet::new();
    let mut stack = vec![p.root.clone()];

    while let Some(id) = stack.pop() {
        if keep.insert(id.clone()) {
            if let Some(n) = map.get(&id) {
                for pr in &n.premises {
                    stack.push(pr.clone());
                }
            }
        }
    }

    // Drop all nodes that are not reachable from the (possibly new) root.
    p.nodes.retain(|n| keep.contains(&n.id));
}

/// If the root is a Cut(n1, n2), set root := n1 and prune unreachable nodes.
/// Otherwise, return the proof unchanged.
pub fn cut_eliminate_root(p: &Proof) -> Proof {
    let mut q = p.clone();

    // Locate the current root node.
    let Some(root_idx) = q.nodes.iter().position(|n| n.id == q.root) else {
        return q;
    };

    // Make the rule check case-insensitive to match "Cut"/"cut".
    let rule = q.nodes[root_idx].rule.to_ascii_lowercase();
    if rule != "cut" || q.nodes[root_idx].premises.len() != 2 {
        return q; // nothing to do
    }

    // Replace the root by the first premise of the Cut and clean up.
    let new_root_id = q.nodes[root_idx].premises[0].clone();
    q.root = new_root_id;
    prune_reachable(&mut q);
    q
}

/// Repeatedly apply `cut_eliminate_root` until it no-ops (fixpoint).
pub fn cut_eliminate_all(p: &Proof) -> Proof {
    let mut prev = p.clone();
    loop {
        let next = cut_eliminate_root(&prev);
        // Fixpoint reached if nothing structural changed.
        if next.root == prev.root && next.nodes.len() == prev.nodes.len() {
            return prev;
        }
        prev = next;
    }
}
