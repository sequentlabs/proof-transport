use anyhow::{bail, Result};

use crate::{
    ast::{Proof, ProofNode},
    registry::RuleId,
    registry::RuleId::*,
};

/// Map rule strings (as they appear in JSON/examples) to RuleId.
/// Accept common historical aliases so old examples still load.
fn rule_from_str(s: &str) -> Option<RuleId> {
    Some(match s {
        // identity / axiom
        "Id" | "Axiom" | "AX" | "Ax" => Id,

        // bottom-introduction
        "BotI" | "⊥I" => BotI,

        // conjunction
        "AndL1" | "∧L1" => AndL1,
        "AndL2" | "∧L2" => AndL2,
        "AndR"  | "∧R"  => AndR,

        // disjunction
        "OrL" | "∨L" => OrL,
        "Or1" | "OrR1" | "∨R1" => Or1,
        "Or2" | "OrR2" | "∨R2" => Or2,

        // implication
        "ImpL" | "ImplL" | "→L" => ImpL,
        "ImpR" | "ImplR" | "→R" => ImpR,

        // cut
        "Cut" => Cut,

        _ => return None,
    })
}

/// Minimal local well‑formedness:
/// - root id exists
/// - each rule name is known
/// - each premise id exists
pub fn validate_local_wf(proof: &Proof) -> Result<()> {
    if !proof.nodes.iter().any(|n| n.id == proof.root) {
        bail!("root id not found: {}", proof.root);
    }

    for ProofNode { id, rule, premises, .. } in &proof.nodes {
        if rule_from_str(rule).is_none() {
            bail!("unknown rule at node {}: {}", id, rule);
        }
        for prem in premises {
            if !proof.nodes.iter().any(|n| n.id == *prem) {
                bail!("premise {} of node {} not found", prem, id);
            }
        }
    }

    Ok(())
}
