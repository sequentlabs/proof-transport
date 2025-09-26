use crate::ast::{Proof, ProofNode};
use std::collections::{HashMap, HashSet};

/// Remove a `Cut` when the *root* node is a `Cut`.
/// This is not a full algorithm; it only rewrites the root and then drops
/// nodes that become unreachable from the (new) root.
pub fn cut_eliminate_root(p: &Proof) -> Proof {
    let root_idx = match p.nodes.iter().position(|n| n.id == p.root) {
        Some(i) => i,
        None => return p.clone(),
    };

    if p.nodes[root_idx].rule != "Cut" || p.nodes[root_idx].premises.len() != 2 {
        return p.clone();
    }

    // New root is the first premise of the Cut
    let new_root = p.nodes[root_idx].premises[0].clone();

    let mut q = p.clone();
    q.root = new_root;

    prune_reachable(&mut q);
    q
}

/// Repeatedly apply [`cut_eliminate_root`] until the root is not a `Cut`.
pub fn cut_eliminate_all(p: &Proof) -> Proof {
    let mut q = p.clone();
    loop {
        let root_idx = match q.nodes.iter().position(|n| n.id == q.root) {
            Some(i) => i,
            None => break,
        };
        if q.nodes[root_idx].rule != "Cut" || q.nodes[root_idx].premises.len() != 2 {
            break;
        }
        q = cut_eliminate_root(&q);
    }
    q
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
            if let Some(m) = map.get(&id) {
                for pr in &m.premises {
                    stack.push(pr.clone());
                }
            }
        }
    }

    p.nodes.retain(|n| keep.contains(&n.id));
}
