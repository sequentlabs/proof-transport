use anyhow::Result;

use crate::{
    ast::Proof,
    cutelim::cut_eliminate_all,
    frag::fragility_score,
    registry::{Registry, RuleId},
    validator::validate_local_wf,
};

/// Transport a proof between registry times.
/// Phase‑1 behavior:
/// - validate input
/// - if target time disables Cut, eliminate all cuts
/// - validate output
pub fn transport(proof: &Proof, reg: &Registry, _from: u64, to: u64) -> Result<Proof> {
    // Clone to avoid mutating input
    let mut p = proof.clone();

    // 1) Validate starting proof
    validate_local_wf(&p)?;

    // 2) Apply registry-aware transform: if Cut disabled at target => eliminate cuts
    let enabled_to = reg.enabled_at(to);
    if !enabled_to.contains(&RuleId::Cut) {
        p = cut_eliminate_all(&p);
    }

    // 3) Validate resulting proof
    validate_local_wf(&p)?;

    Ok(p)
}

/// Convenience helper for tests/metrics
pub fn fragility_delta(proof: &Proof, reg: &Registry, from: u64, to: u64) -> Result<i64> {
    let before = fragility_score(proof) as i64;
    let after_proof = transport(proof, reg, from, to)?;
    let after = fragility_score(&after_proof) as i64;
    Ok(after - before)
}
