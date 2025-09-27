use anyhow::{bail, Result};

use crate::{
    ast::{Proof, ProofNode},
    registry::{RuleId, RuleId::*},
};

/// Convert textual rule names (from JSON/examples) to our internal RuleId.
/// Be liberal in what we accept: handle case/spacing and common aliases
/// used across tests (e.g. `Axiom`, `OrR1`, `OrR2`, …).
fn rule_from_str(s: &str) -> Option<RuleId> {
    let norm = s.trim();
    let lower = norm.to_ascii_lowercase();

    // ASCII/case-insensitive aliases
    let ascii_hit = match lower.as_str() {
        // Axiom/Identity
        "id" | "axiom" | "ax" => Some(Id),

        // ⊥ introduction
        "boti" | "bottomi" | "bottomintro" => Some(BotI),

        // ∧ left 1/2, right
        "andl1" | "andleft1" | "∧l1" => Some(AndL1),
        "andl2" | "andleft2" | "∧l2" => Some(AndL2),
        "andr" | "andright" | "∧r" => Some(AndR),

        // ∨ left, right-1, right-2
        "orl" | "orleft" | "∨l" => Some(OrL),
        "or1" | "orr1" | "orright1" | "∨r1" => Some(Or1),
        "or2" | "orr2" | "orright2" | "∨r2" => Some(Or2),

        // → left/right
        "impl" | "impliesl" | "→l" => Some(ImpL),
        "impr" | "impliesr" | "→r" => Some(ImpR),

        // Cut
        "cut" => Some(Cut),

        _ => None,
    };

    if ascii_hit.is_some() {
        return ascii_hit;
    }

    // Exact unicode-labelled hits (when case-normalization above wouldn't match)
    match norm {
        "Id" | "ID" | "Axiom" | "Ax" | "AX" => Some(Id),

        "⊥I" => Some(BotI),

        "AndL1" | "∧L1" => Some(AndL1),
        "AndL2" | "∧L2" => Some(AndL2),
        "AndR" | "∧R" => Some(AndR),

        "OrL" | "∨L" => Some(OrL),
        "Or1" | "OrR1" | "∨R1" => Some(Or1),
        "Or2" | "OrR2" | "∨R2" => Some(Or2),

        "ImpL" | "→L" => Some(ImpL),
        "ImpR" | "→R" => Some(ImpR),

        "Cut" | "CUT" => Some(Cut),

        _ => None,
    }
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
