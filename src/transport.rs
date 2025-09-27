// src/transport.rs
use anyhow::Result;

use crate::{
    ast::Proof,
    cutelim::cut_eliminate_all,
    frag::fragility_score,
    registry::{Registry, RuleId},
    validator::validate_local_wf,
};

/// Transport a proof from one registry time `from` to another `to`.
/// - Always validates before and after.
/// - If the target time disables `Cut`, all cuts are eliminated.
/// - Returns the transformed proof.
///
/// Note: `_from` is currently unused but kept for API stability; we may
/// later add behaviors that depend on the source registry time.
pub fn transport(proof: &Proof, reg: &Registry, _from: u64, to: u64) -> Result<Proof> {
    // Work on a clone to avoid mutating the input
    let mut p = proof.clone();

    // Validate the starting proof
    validate_local_wf(&p)?;

    // If Cut is disabled at the target, eliminate all cuts
    let enabled_to = reg.enabled_at(to);
    if !enabled_to.contains(&RuleId::Cut) {
        p = cut_eliminate_all(&p);
    }

    // Validate after transport
    validate_local_wf(&p)?;
    Ok(p)
}

/// Compute the fragility delta when transporting a proof across registry evolution.
///
/// Returns: after_fragility - before_fragility (strictly negative if we removed `Cut`).
pub fn fragility_delta(proof: &Proof, reg: &Registry, from: u64, to: u64) -> Result<i64> {
    let before = fragility_score(proof) as i64;
    let after_proof = transport(proof, reg, from, to)?;
    let after = fragility_score(&after_proof) as i64;
    Ok(after - before)
}
