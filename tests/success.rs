#[test]
fn lib_exports_compile() {
    let _ = proof_transport::frag::fragility_score
        as fn(&proof_transport::ast::Proof) -> u64;
    let _ = proof_transport::validator::validate_local_wf
        as fn(&proof_transport::ast::Proof) -> anyhow::Result<()>;
}
