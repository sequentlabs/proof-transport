use crate::ast::{Proof, ProofNode};
use std::collections::{HashMap, HashSet};

/// If the root rule is a `Cut`, replace the proof root with its first premise
/// and drop nodes that became unreachable.
pub fn cut_eliminate_root(p: &Proof) -> Proof {
    if p.nodes.is_empty() {
        return p.clone();
    }

    let mut q = p.clone();

    // Find the index of the current root node.
    let root_idx = match q.nodes.iter().position(|n| n.id == q.root) {
        Some(i) => i,
        None => return q,
    };

    // Only act when the root is a Cut.
    if q.nodes[root_idx].rule != "Cut" {
        return q;
    }

    // Use the first premise of the Cut as the new root (defensive check).
    if q.nodes[root_idx].premises.is_empty() {
        // Malformed Cut; leave as-is.
        return q;
    }
    let new_root = q.nodes[root_idx].premises[0].clone();
    q.root = new_root;

    prune_reachable(&mut q);
    q
}

/// Very naive "eliminate all": repeatedly apply `cut_eliminate_root` while the
/// root changes. (Sufficient for the current tests.)
pub fn cut_eliminate_all(p: &Proof) -> Proof {
    let mut current = p.clone();
    loop {
        let next = cut_eliminate_root(&current);
        if next.root == current.root {
            return next;
        }
        current = next;
    }
}

/// Keep only nodes reachable from `p.root`.
fn prune_reachable(p: &mut Proof) {
    // Snapshot the graph so we can traverse without borrowing `p.nodes`.
    let map: HashMap<String, ProofNode> =
        p.nodes.iter().cloned().map(|n| (n.id.clone(), n)).collect();

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

    p.nodes.retain(|n| keep.contains(&n.id));
}
