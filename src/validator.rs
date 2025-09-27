use crate::ast::{Proof, ProofNode};
use crate::registry::RuleId;
use anyhow::{bail, Result};

// Bring variants into scope for readable pattern code below.
use RuleId::*;

/// Map rule strings (as they appear inside the JSON proof) to `RuleId`.
fn rule_from_str(s: &str) -> Option<RuleId> {
    Some(match s {
        "Id" => Id,
        "BotI" => BotI,
        "AndR" => AndR,
        "AndL1" => AndL1,
        "AndL2" => AndL2,
        "OrR1" => OrR1,
        "OrR2" => OrR2,
        "OrL" => OrL,
        "ImplR" => ImplR,
        "ImplL" => ImplL,
        "Cut" => Cut,
        _ => return None,
    })
}

/// Lightweight local well‑formedness checks:
///  * root id exists
///  * node rules are known
///  * each premise id exists in the proof
pub fn validate_local_wf(proof: &Proof) -> Result<()> {
    // 1) root must exist
    if !proof.nodes.iter().any(|n| n.id == proof.root) {
        bail!("root id not found: {}", proof.root);
    }

    // 2) per-node checks
    for ProofNode { id, rule, premises, .. } in &proof.nodes {
        if rule_from_str(rule).is_none() {
            bail!("unknown rule at node {}: {}", id, rule);
        }
        for pr in premises {
            if !proof.nodes.iter().any(|n| &n.id == pr) {
                bail!("premise {} of node {} not found", pr, id);
            }
        }
    }

    Ok(())
}
