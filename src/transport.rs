use anyhow::Result;

use crate::{
    ast::Proof,
    cutelim::cut_eliminate_all,
    frag::fragility_score,
    registry::{Registry, RuleId},
    validator::validate_local_wf,
};

/// Stability/trace information emitted by transport.
#[derive(Debug, Clone, Default)]
pub struct Certificate {
    /// Best-effort count of `Cut` nodes removed.
    pub removed_cuts: usize,
    /// Nodes pruned by reachability (before - after).
    pub pruned_nodes: usize,
    /// Fragility score before transport (signed to ease diffs).
    pub fragility_before: i64,
    /// Fragility score after transport.
    pub fragility_after: i64,
}

/// Transport a proof from one registry time `from` to another `to`.
/// If the target disables `Cut`, all cuts are eliminated.
/// Returns the transformed proof.
pub fn transport(proof: &Proof, reg: &Registry, from: u64, to: u64) -> Result<Proof> {
    // Clone to avoid mutating input
    let mut p = proof.clone();

    // Always validate the starting proof
    validate_local_wf(&p)?;

    // If Cut is disabled at target time, eliminate cuts
    let enabled_to = reg.enabled_at(to);
    if !enabled_to.contains(&RuleId::Cut) {
        p = cut_eliminate_all(&p);
    }

    // Re-validate after transport
    validate_local_wf(&p)?;

    Ok(p)
}

/// Same as `transport`, but also emits a `Certificate`.
pub fn transport_with_cert(
    proof: &Proof,
    reg: &Registry,
    from: u64,
    to: u64,
) -> Result<(Proof, Certificate)> {
    let before_nodes = proof.nodes.len();
    let before_frag = fragility_score(proof) as i64;
    let before_cuts = proof.nodes.iter().filter(|n| n.rule == "Cut").count();

    let q = transport(proof, reg, from, to)?;

    let after_nodes = q.nodes.len();
    let after_frag = fragility_score(&q) as i64;
    let after_cuts = q.nodes.iter().filter(|n| n.rule == "Cut").count();

    let cert = Certificate {
        removed_cuts: before_cuts.saturating_sub(after_cuts),
        pruned_nodes: before_nodes.saturating_sub(after_nodes),
        fragility_before: before_frag,
        fragility_after: after_frag,
    };

    Ok((q, cert))
}

/// Minimal convenience transport used by early tests/CLI:
/// validates, eliminates cuts unconditionally, validates again,
/// and emits a certificate.
pub fn transport_simple(proof: &Proof) -> (Proof, Certificate) {
    // pre-validate
    validate_local_wf(proof).expect("input proof is not locally well-formed");

    let before_nodes = proof.nodes.len();
    let before_frag = fragility_score(proof) as i64;
    let before_cuts = proof.nodes.iter().filter(|n| n.rule == "Cut").count();

    let q = cut_eliminate_all(proof);

    // post-validate
    validate_local_wf(&q).expect("output proof is not locally well-formed");

    let after_nodes = q.nodes.len();
    let after_frag = fragility_score(&q) as i64;
    let after_cuts = q.nodes.iter().filter(|n| n.rule == "Cut").count();

    let cert = Certificate {
        removed_cuts: before_cuts.saturating_sub(after_cuts),
        pruned_nodes: before_nodes.saturating_sub(after_nodes),
        fragility_before: before_frag,
        fragility_after: after_frag,
    };

    (q, cert)
}

/// Compute fragility delta across registry evolution.
pub fn fragility_delta(proof: &Proof, reg: &Registry, from: u64, to: u64) -> Result<i64> {
    let before = fragility_score(proof) as i64;
    let after_proof = transport(proof, reg, from, to)?;
    let after = fragility_score(&after_proof) as i64;
    Ok(after - before)
}
