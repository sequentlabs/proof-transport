use crate::ast::{Proof, ProofNode};
use std::collections::{HashMap, HashSet};

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

