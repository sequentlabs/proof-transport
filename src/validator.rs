use anyhow::{bail, Result};

use crate::{
    ast::{Proof, ProofNode},
    registry::{RuleId, RuleId::*},
};

/// Map rule strings (as they appear in JSON/examples) to RuleId.
/// Names/case match tests exactly, plus a few common aliases.
fn rule_from_str(s: &str) -> Option<RuleId> {
    Some(match s {
        // Core spellings
        "Id" => Id,
        "BotI" => BotI,
        "AndR" => AndR,
        "AndL1" => AndL1,
        "AndL2" => AndL2,
        "OrL" => OrL,
        "Or1" => Or1,
        "Or2" => Or2,
        "ImpL" => ImpL,
        "ImpR" => ImpR,
        "Cut" => Cut,

        // Tolerated aliases used in examples/golden data
        "Axiom" | "Ax" => Id,
        "OrR1" | "∨R1" => Or1,
        "OrR2" | "∨R2" => Or2,
        "∨L" => OrL,
        "∧R" => AndR,
        "∧L1" => AndL1,
        "∧L2" => AndL2,
        "→L" => ImpL,
        "→R" => ImpR,

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
