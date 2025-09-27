use anyhow::Result;

use crate::{
    ast::Proof,
    cutelim::cut_eliminate_all,
    frag::fragility_score,
    registry::{Registry, RuleId},
    validator::validate_local_wf,
};

/// Transport a proof between registry times.
///
/// Phase‑1 behavior:
/// 1) validate input
/// 2) if the *target* time disables `Cut`, eliminate all cuts
/// 3) validate output
///
/// NOTE: The `from` parameter is part of the public API. We do not need it for
/// Phase‑1 semantics, but we *intentionally* exercise it here so `-D warnings`
/// does not fail the build.
pub fn transport(proof: &Proof, reg: &Registry, from: u64, to: u64) -> Result<Proof> {
    // Mark `from` as intentionally used (keep API, satisfy -D warnings)
    let _enabled_from = reg.enabled_at(from);

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

/// Convenience helper used by tests/metrics.
pub fn fragility_delta(proof: &Proof, reg: &Registry, from: u64, to: u64) -> Result<i64> {
    let before = fragility_score(proof) as i64;
    let after_proof = transport(proof, reg, from, to)?;
    let after = fragility_score(&after_proof) as i64;
    Ok(after - before)
}
