use crate::ast::Proof;
use crate::registry::RuleId;
use anyhow::{bail, Result};

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
    let ids: HashSet<_> = proof.nodes.iter().map(|n| n.id.as_str()).collect();

    if !ids.contains(proof.root.as_str()) {
        bail!("root id not found: {}", proof.root);
    }

    for ProofNode { id, rule, premises, .. } in &proof.nodes {
        if rule_from_str(rule).is_none() {
            bail!("unknown rule at node {}: {}", id, rule);
        }
        for p in premises {
            if !ids.contains(p.as_str()) {
                bail!("premise {} referenced by {} not found", p, id);
            }
        }
    }
    Ok(())
}
