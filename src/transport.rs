use anyhow::Result;

use crate::{
    ast::Proof,
    cutelim::cut_eliminate_all,
    registry::{Registry, RuleId},
    validator::validate_local_wf,
};

/// Transport a proof between registry times.
///
/// Phase‑1 behavior:
/// - validate input
/// - if target time disables Cut, eliminate all cuts
/// - validate output
pub fn transport(proof: &Proof, reg: &Registry, from: u64, to: u64) -> Result<Proof> {
    // Keep `from` in the public API; silence -D unused-variables for now.
    let _ = from;

    // clone so we don't mutate the input
    let mut p = proof.clone();

    // 1) validate starting proof
    validate_local_wf(&p)?;

    // 2) if Cut is disabled at target time, eliminate all cuts
    if !reg.enabled_at(to).contains(&RuleId::Cut) {
        p = cut_eliminate_all(&p);
    }

    // 3) validate resulting proof
    validate_local_wf(&p)?;
    Ok(p)
}

/// Convenience: change in fragility across a transport step.
pub fn fragility_delta(proof: &Proof, reg: &Registry, from: u64, to: u64) -> Result<i64> {
    let before = crate::frag::fragility_score(proof) as i64;
    let after_proof = transport(proof, reg, from, to)?;
    let after = crate::frag::fragility_score(&after_proof) as i64;
    Ok(after - before)
}
