use crate::ast::{Proof, ProofNode};
use std::collections::{HashMap, HashSet};

/// Cut-eliminate when the *root* node is a `Cut` with two premises.
/// We set the new root to the first premise and prune unreachable nodes.
/// This is a toy step but demonstrates a real rewrite.
pub fn cut_eliminate_root(p: &Proof) -> Proof {
    let mut q = p.clone();
    // fast exit
    let Some(root_idx) = q.nodes.iter().position(|n| n.id == q.root) else {
        return q;
    };
    if q.nodes[root_idx].rule != "Cut" {
        return q;
    }
    if q.nodes[root_idx].premises.len() != 2 {
        return q;
    }

    // new root = first premise of the Cut
    let new_root = q.nodes[root_idx].premises[0].clone();
    q.root = new_root;

    prune_reachable(&mut q);
    q
}

/// Keep only nodes reachable from `root`
fn prune_reachable(p: &mut Proof) {
    let map: HashMap<String, ProofNode> =
        p.nodes.iter().cloned().map(|n| (n.id.clone(), n)).collect();

    let mut stack = vec![p.root.clone()];
    let mut keep: HashSet<String> = HashSet::new();

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
