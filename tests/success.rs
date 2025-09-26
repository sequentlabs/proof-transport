use proof_transport::{ast::Proof, frag::fragility_score, validator::validate_local_wf};

#[test]
fn lib_exports_compile() {
    let _: fn(&Proof) -> u64 = fragility_score;
    let _: fn(&Proof) -> anyhow::Result<()> = validate_local_wf;
}
