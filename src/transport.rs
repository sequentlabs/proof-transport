use anyhow::Result;

use crate::{
    ast::Proof,
    cutelim::cut_eliminate_all,
    frag::fragility_score,
    registry::Registry,
    validator::validate_local_wf,
};

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
    if !enabled_to.contains(&crate::registry::RuleId::Cut) {
        p = cut_eliminate_all(&p);
    }

    // Re-validate after transport
    validate_local_wf(&p)?;

    Ok(p)
}

/// Compute fragility delta across registry evolution.
pub fn fragility_delta(proof: &Proof, reg: &Registry, from: u64, to: u64) -> Result<i64> {
    let before = fragility_score(proof) as i64;
    let after_proof = transport(proof, reg, from, to)?;
    let after = fragility_score(&after_proof) as i64;
    Ok(after - before)
}
