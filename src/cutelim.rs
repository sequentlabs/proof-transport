use crate::ast::{Proof, ProofNode};
use std::collections::{HashMap, HashSet};

/// Eliminate cuts at the root repeatedly until no root Cut remains.
pub fn cut_eliminate_root(p: &Proof) -> Proof {
    let mut q = p.clone();

    loop {
        let root_idx = q.nodes.iter().position(|n| n.id == q.root);
        if root_idx.is_none() {
            break;
        }
        let root_idx = root_idx.unwrap();

        if q.nodes[root_idx].rule != "Cut" || q.nodes[root_idx].premises.len() != 2 {
            break;
        }

        let new_root = q.nodes[root_idx].premises[0].clone();
        q.root = new_root;

        let cut_id = q.nodes[root_idx].id.clone();
        q.nodes.retain(|n| n.id != cut_id);

        prune_reachable(&mut q);
    }

    q
}

/// Eliminate all cuts by repeatedly applying eliminate_one_cut until no Cut remains.
pub fn cut_eliminate_all(p: &Proof) -> Proof {
    let mut q = p.clone();
    loop {
        let changed = eliminate_one_cut(&mut q);
        if !changed {
            break;
        }
    }
    q
}

/// Try to eliminate a single cut anywhere in the proof.
/// Returns true if a cut was removed, false otherwise.
fn eliminate_one_cut(p: &mut Proof) -> bool {
    if let Some(cut_node) = p.nodes.iter().find(|n| n.rule == "Cut").cloned() {
        if cut_node.id == p.root && !cut_node.premises.is_empty() {
            p.root = cut_node.premises[0].clone();
        }
        p.nodes.retain(|n| n.id != cut_node.id);
        prune_reachable(p);
        return true;
    }
    false
}

/// Keep only nodes reachable from the root
fn prune_reachable(p: &mut Proof) {
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
