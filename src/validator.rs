use anyhow::{bail, Result};

use crate::ast::{Proof, ProofNode};
use crate::registry::RuleId;

/// Map textual rule names that appear in example JSON to our RuleId enum.
/// This is the single source‑of‑truth for the string↔enum mapping.
fn rule_from_str(s: &str) -> Option<RuleId> {
    use RuleId::*;
    Some(match s {
        "Id" => Id,
        "BotI" => BotI,
        "AndR" => AndR,
        "AndL" => AndL,
        "OrR1" => OrR1,
        "OrR2" => OrR2,
        "OrL1" => OrL1,
        "OrL2" => OrL2,
        "ImpR" => ImpR, // ← standardize on ImpR
        "ImpL" => ImpL, // ← standardize on ImpL
        "Cut"  => Cut,
        _ => return None,
    })
}

/// Local, structure-only well‑formedness:
/// * root id exists
/// * each node's rule name is known
/// * each premise id exists in the proof
pub fn validate_local_wf(proof: &Proof) -> Result<()> {
    // root must be present
    if !proof.nodes.iter().any(|n| n.id == proof.root) {
        bail!("root id not found: {}", proof.root);
    }

    // walk nodes and check rule + premises
    for ProofNode { id, rule, premises, .. } in &proof.nodes {
        let Some(_rid) = rule_from_str(rule.as_str()) else {
            bail!("unknown rule at node {}: {}", id, rule);
        };

        for prem in premises {
            if !proof.nodes.iter().any(|n| n.id == *prem) {
                bail!("premise {} of node {} not found", prem, id);
            }
        }
    }

    Ok(())
}
