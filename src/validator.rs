use anyhow::{bail, Result};

use crate::ast::{Proof, ProofNode};
use crate::registry::RuleId;
use crate::registry::RuleId::*;

/// Map a rule string from JSON into a concrete `RuleId`.
/// (Accept a few common aliases; return `None` if unknown.)
fn rule_from_str(s: &str) -> Option<RuleId> {
    // Keep the match case-sensitive on the *right* hand side (enum variants),
    // but allow a couple of aliases on the *left*.
    match s {
        // identity / axiom
        "Id" | "Axiom" => Some(Id),

        // bottom-introduction (aliases kept for convenience)
        "BotI" | "BottomI" | "Bot" => Some(BotI),

        // conjunction
        "AndR" => Some(AndR),
        "AndI1" => Some(AndI1),
        "AndI2" => Some(AndI2),

        // disjunction
        "OrR1" => Some(OrR1),
        "OrR2" => Some(OrR2),
        "OrI1" => Some(OrI1),
        "OrI2" => Some(OrI2),
        "OrL"  => Some(OrL),

        // implication
        "ImpR" => Some(ImpR),
        "ImpL" => Some(ImpL),

        // structural
        "Cut" => Some(Cut),

        // unknown rule name
        _ => None,
    }
}

/// Local well-formedness checks that do **not** depend on registry time:
/// - root id exists in the node list
/// - every node uses a known rule name
/// - every premise id referenced by a node exists
pub fn validate_local_wf(proof: &Proof) -> Result<()> {
    // 1) Root must exist
    if !proof.nodes.iter().any(|n| n.id == proof.root) {
        bail!("root id not found: {}", proof.root);
    }

    // 2) For each node, validate rule + premises exist
    for ProofNode { id, rule, premises, .. } in &proof.nodes {
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
