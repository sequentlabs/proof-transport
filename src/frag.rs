use crate::ast::Proof;

/// toy fragility score = nodes + 10 * (#Cut)
pub fn fragility_score(proof: &Proof) -> u64 {
    let mut cuts = 0;
    for n in &proof.nodes {
        if n.rule == "Cut" { cuts += 1; }
    }
    proof.nodes.len() as u64 + 10 * cuts
}
