use anyhow::{bail, Result};
use std::collections::HashSet;

use crate::ast::{Proof, ProofNode};
use crate::registry::RuleId;

fn rule_from_str(s: &str) -> Option<RuleId> {
    use RuleId::*;
    Some(match s {
        "Id" => Id,
        "BotL" => BotL,
        "AndR" => AndR,
        "AndL1" => AndL1,
        "AndL2" => AndL2,
        "OrR1" => OrR1,
        "OrR2" => OrR2,
        "OrL" => OrL,
        "ImpR" => ImpR,
        "ImpL" => ImpL,
        "Cut" => Cut,
        _ => return None,
    })
}

pub fn validate_local_wf(proof: &Proof) -> Result<()> {
    if !proof.nodes.iter().any(|n| n.id == proof.root) {
        bail!("root id not found: {}", proof.root);
    }

    for ProofNode {
        id, rule, premises, ..
    } in &proof.nodes
    {
        if rule_from_str(rule).is_none() {
            bail!("unknown rule at node {}: {}", id, rule);
        }
        for prem in premises {
            if !proof.nodes.iter().any(|n| &n.id == prem) {
                bail!("premise {} of node {} not found", prem, id);
            }
        }
    }

    Ok(())
}
