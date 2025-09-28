// tests/transport_invariants.rs
use std::fs;

use proof_transport::{
    ast::Proof,
    cutelim::cut_eliminate_all,
    frag::fragility_score,
    validator::validate_local_wf,
};

/// Tolerant loader for examples:
/// - Accepts JSON with comments / trailing commas (via json5)
/// - Then deserializes into our strongly-typed `Proof`
/// - Our `Sequent` deserializer is already permissive (string form,
///   2‑tuple `[ctx, thm]`, 3‑tuple `[ctx, "<sep>", thm]`, and object form).
fn load(path: &str) -> Proof {
    let s = fs::read_to_string(path).expect("open JSON");
    // Parse the source as JSON5 (allows comments, trailing commas)
    let v: serde_json::Value = json5::from_str(&s).expect("parse examples as JSON5");
    // Convert to the concrete type (this uses our tolerant `Sequent` impl)
    serde_json::from_value::<Proof>(v).expect("decode Proof")
}

/// On these inputs we intentionally have a `Cut` at/near the root,
/// so eliminating cuts must strictly drop fragility.
#[test]
fn fragility_strictly_drops_on_cut_examples() {
    let paths = [
        "examples/proof_with_cut.json",   // existing root Cut
        "examples/proof_cut_chain.json",  // nested/internal Cut
        "examples/proof_cut_pair.json",   // sibling Cuts
    ];

    for path in paths {
        let p = load(path);
        validate_local_wf(&p).expect("wf before");

        let before = fragility_score(&p);
        let q = cut_eliminate_all(&p);
        validate_local_wf(&q).expect("wf after");

        let after = fragility_score(&q);
        assert!(
            after < before,
            "expected fragility to strictly drop on {path}, got {before} -> {after}"
        );
    }
}

/// For inputs with no root Cut (or no relevant policy trigger),
/// elimination may be a no-op, but it must *never* increase fragility.
/// (This also guards against regressions in the scoring function.)
#[test]
fn fragility_never_increases_on_all_examples() {
    let paths = [
        "examples/proof_with_cut.json",
        "examples/proof_cut_chain.json",
        "examples/proof_cut_pair.json",
        "examples/proof_fo_quantifiers.json",
        "examples/proof_with_unreachable.json",
        "examples/proof_cut_free.json",
    ];

    for path in paths {
        let p = load(path);
        validate_local_wf(&p).expect("wf before");

        let before = fragility_score(&p);
        let q = cut_eliminate_all(&p);
        validate_local_wf(&q).expect("wf after");

        let after = fragility_score(&q);
        assert!(
            after <= before,
            "fragility increased on {path}: {before} -> {after}"
        );
    }
}

/// Explicitly assert that pruning removes unreachable nodes on the
/// `ghost` example we added to exercise graph cleanup.
#[test]
fn unreachable_nodes_are_pruned() {
    let p = load("examples/proof_with_unreachable.json");
    validate_local_wf(&p).expect("wf before");

    let before_nodes = p.nodes.len();
    let q = cut_eliminate_all(&p); // elimination + subsequent prune()
    validate_local_wf(&q).expect("wf after");
    let after_nodes = q.nodes.len();

    assert!(
        after_nodes < before_nodes,
        "expected prune to drop unreachable nodes: {before_nodes} -> {after_nodes}"
    );
}
