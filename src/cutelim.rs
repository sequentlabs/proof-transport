use crate::ast::{Proof, ProofNode};
use std::collections::{HashMap, HashSet};

/// Eliminate the root if it is a Cut with two premises.
/// Returns a new proof with the root replaced.
pub fn cut_eliminate_root(p: &Proof) -> Proof {
    let mut q = p.clone();
    if let Some(root_node) = q.nodes.iter().find(|n| n.id == q.root) {
        if root_node.rule == "Cut" && !root_node.premises.is_empty() {
            // Replace root by first premise of the Cut
            q.root = root_node.premises[0].clone();
            // Drop the old Cut node
            q.nodes.retain(|n| n.id != root_node.id);
            // Prune unreachable nodes
            prune_reachable(&mut q);
        }
    }
    q
}

/// Eliminate all cuts by repeatedly applying cut_eliminate_root until no Cut remains.
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
/// Returns true if a cut was removed, false if none found.
fn eliminate_one_cut(p: &mut Proof) -> bool {
    // Map for quick node lookup
    let map: HashMap<String, ProofNode> =
        p.nodes.iter().cloned().map(|n| (n.id.clone(), n)).collect();

    // Find any Cut node
    if let Some(cut_node) = p.nodes.iter().find(|n| n.rule == "Cut") {
        // Replace root if the cut is root
        if cut_node.id == p.root && !cut_node.premises.is_empty() {
            p.root = cut_node.premises[0].clone();
        }
        // Remove this cut node from list
        p.nodes.retain(|n| n.id != cut_node.id);
        // Prune unreachable nodes
        prune_reachable(p);
        return true;
    }
    false
}

/// Keep only nodes reachable from root
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
