use anyhow::Result;

use crate::{
    ast::Proof,
    cutelim::cut_eliminate_all,
    frag::fragility_score,
    registry::{Registry, RuleId},
    validator::validate_local_wf,
};

/// Transport a proof from one registry time `from` to another `to`.
/// If the target disables `Cut`, all cuts are eliminated.
/// Returns the transformed proof.
///
/// Notes:
/// - We currently don’t use `from` directly; keep it in the signature for API
///   stability and future evolution, but mark it unused to keep CI happy.
pub fn transport(proof: &Proof, reg: &Registry, _from: u64, to: u64) -> Result<Proof> {
    // Clone to avoid mutating input
    let mut p = proof.clone();

    // Validate starting proof
    validate_local_wf(&p)?;

    // If `Cut` is disabled at target time, eliminate all cuts
    let enabled_to = reg.enabled_at(to);
    if !enabled_to.contains(&RuleId::Cut) {
        p = cut_eliminate_all(&p);
    }

    // Validate after transformation
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
