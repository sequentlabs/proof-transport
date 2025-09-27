use crate::ast::{Proof, ProofNode};
use std::collections::{HashMap, HashSet};

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
/// Returns true if a cut was removed, false if none found.
fn eliminate_one_cut(p: &mut Proof) -> bool {
    // Clone nodes for searching to avoid borrow conflicts
    let snapshot: Vec<ProofNode> = p.nodes.clone();

    // Find any Cut node in the snapshot
    if let Some(cut_node) = snapshot.iter().find(|n| n.rule == "Cut") {
        // Replace root if the cut is the root
        if cut_node.id == p.root && !cut_node.premises.is_empty() {
            p.root = cut_node.premises[0].clone();
        }

        // Remove the cut node from the actual list
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
