// Bring rule IDs into scope for equals/pattern use (e.g., OrR2, ImpL).
use crate::registry::RuleId::*;
use crate::registry::RuleId;

use anyhow::{bail, Result};
use crate::ast::{Proof, ProofNode};

/// Best-effort parser from rule string to a known RuleId.
/// Phase 1: accept a broad set; unknown rules are rejected by validate_local_wf.
fn rule_from_str(s: &str) -> Option<RuleId> {
    use RuleId::*;
    Some(match s {
        "Axiom" => Axiom,
        "Id"    => Id,
        "BotL"  => BotL,
        "AndR"  => AndR,
        "AndL1" => AndL1,
        "AndL2" => AndL2,
        "OrR1"  => OrR1,
        "OrR2"  => OrR2,
        "OrL"   => OrL,
        "ImpR"  => ImpR,
        "ImpL"  => ImpL,
        "Cut"   => Cut,
        _ => return None,
    })
}

/// Very lightweight local well‑formedness:
///  - root id must exist in nodes
///  - every node's rule must be recognized
///  - every premise id must refer to an existing node
pub fn validate_local_wf(proof: &Proof) -> Result<()> {
    // Root must exist.
    if proof.nodes.iter().all(|n| n.id != proof.root) {
        bail!("root id not found: {}", proof.root);
    }

    // Node-by-node checks.
    for ProofNode { id, rule, premises, .. } in &proof.nodes {
        if rule_from_str(rule).is_none() {
            bail!("unknown rule at node {}: {}", id, rule);
        }

        // Each premise id must exist.
        for prem in premises {
            if proof.nodes.iter().any(|n| n.id == *prem) == false {
                bail!("premise {} of node {} not found", prem, id);
            }
        }
    }

    Ok(())
}
