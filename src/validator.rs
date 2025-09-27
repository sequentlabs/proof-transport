use anyhow::{bail, Result};

use crate::{
    ast::{Proof, ProofNode},
    registry::{RuleId, RuleId::*},
};

/// Map rule strings (as they appear in JSON/examples) to RuleId.
/// Accept a few common aliases and case variations used in older files.
fn rule_from_str(s: &str) -> Option<RuleId> {
    let t = s.trim();

    // Normalize a couple of frequent aliases and case variants.
    let lower = t.to_ascii_lowercase();

    // Identity / Axiom
    if matches!(lower.as_str(), "id" | "axiom" | "ax") {
        return Some(Id);
    }

    // Bottom introduction
    if matches!(lower.as_str(), "boti" | "⊥i" | "bot_i") {
        return Some(BotI);
    }

    // And rules
    if matches!(lower.as_str(), "andl1" | "and_l1" | "∧l1") {
        return Some(AndL1);
    }
    if matches!(lower.as_str(), "andl2" | "and_l2" | "∧l2") {
        return Some(AndL2);
    }
    if matches!(lower.as_str(), "andr" | "and_r" | "∧r") {
        return Some(AndR);
    }

    // Or rules (left and the two right-introductions)
    if matches!(lower.as_str(), "orl" | "or_l" | "∨l") {
        return Some(OrL);
    }
    if matches!(lower.as_str(), "or1" | "orr1" | "or_r1" | "∨r1") {
        return Some(Or1);
    }
    if matches!(lower.as_str(), "or2" | "orr2" | "or_r2" | "∨r2") {
        return Some(Or2);
    }

    // Implication rules
    if matches!(lower.as_str(), "impl" | "imp_l" | "→l" | "->l") {
        return Some(ImpL);
    }
    if matches!(lower.as_str(), "impr" | "imp_r" | "→r" | "->r") {
        return Some(ImpR);
    }

    // Cut
    if matches!(lower.as_str(), "cut") {
        return Some(Cut);
    }

    None
}

/// Minimal local well‑formedness:
///  - root id exists
///  - each rule name is known
///  - each premise id exists
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
