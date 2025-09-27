use std::fs::File;
use serde_json::from_reader;

use proof_transport::{
    ast::Proof,
    // cut elimination and analysis
    cutelim::{cut_eliminate_all, cut_eliminate_root},
    fragility_score,
    // registry & transport API (no policy mutation here; we only need an instance)
    registry::Registry,
    transport::{fragility_delta, transport},
    // local well-formedness validator
    validate_local_wf,
};

fn load(path: &str) -> Proof {
    let f = File::open(path).expect("example JSON should exist");
    from_reader::<_, Proof>(f).expect("example JSON should parse as Proof")
}

fn has_cut(p: &Proof) -> bool {
    p.nodes.iter().any(|n| n.rule == "Cut")
}

#[test]
fn cutelim_root_preserves_wf_and_never_increases_fragility() {
    // choose a proof where the root is a `Cut`
    let p = load("examples/proof_cut_pair.json");
    validate_local_wf(&p).unwrap();

    let before = fragility_score(&p);
    let q = cut_eliminate_root(&p);
    validate_local_wf(&q).unwrap();

    let after = fragility_score(&q);

    // Root cut rewritten ⇒ fragility cannot increase; it usually drops.
    assert!(
        after <= before,
        "fragility increased after cut_eliminate_root: {} -> {}",
        before,
        after
    );
}

#[test]
fn cutelim_all_is_idempotent_and_yields_cut_free() {
    // choose a proof with cuts to exercise elimination
    let p = load("examples/proof_cut_chain.json");
    validate_local_wf(&p).unwrap();
    assert!(has_cut(&p), "precondition: example should contain a Cut");

    let q = cut_eliminate_all(&p);
    validate_local_wf(&q).unwrap();
    assert!(!has_cut(&q), "cut_eliminate_all must yield a cut-free proof");

    // idempotent: running again does nothing
    let r = cut_eliminate_all(&q);
    validate_local_wf(&r).unwrap();
    assert_eq!(fragility_score(&q), fragility_score(&r));
    assert!(!has_cut(&r));
}

#[test]
fn transport_is_wf_and_idempotent_when_policy_is_unchanged() {
    // No reliance on time-varying policies: we transport at the same time.
    let reg = Registry::default(); // existing API already used in other tests
    let p = load("examples/proof_cut_pair.json");
    validate_local_wf(&p).unwrap();

    let t = 42u64;
    let q = transport(&p, &reg, t, t).expect("transport should succeed");
    validate_local_wf(&q).unwrap();

    // With no change in policy, transport is a no-op w.r.t. running twice
    let r = transport(&q, &reg, t, t).expect("transport should succeed");
    validate_local_wf(&r).unwrap();

    assert_eq!(
        fragility_score(&q),
        fragility_score(&r),
        "transport should be idempotent when from==to"
    );
}

#[test]
fn fragility_delta_matches_scores() {
    // The helper should equal (after - before) from a normal transport call.
    let reg = Registry::default();
    let p = load("examples/proof_cut_free.json"); // cut-free: safe & simple
    validate_local_wf(&p).unwrap();

    let from = 0u64;
    let to = 0u64;

    let before = fragility_score(&p);
    let after_proof = transport(&p, &reg, from, to).expect("transport ok");
    validate_local_wf(&after_proof).unwrap();
    let after = fragility_score(&after_proof);

    let delta = fragility_delta(&p, &reg, from, to).expect("delta ok");

    assert_eq!(delta, (after as i64) - (before as i64));
}
