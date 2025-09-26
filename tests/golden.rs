use std::fs::File;
use serde_json::from_reader;

use proof_transport::{ast::Proof, registry::Registry, transport::transport};

#[test]
fn golden_prop_cut_elimination() {
    let input: Proof =
        from_reader(File::open("examples/prop_cut_in.json").unwrap()).unwrap();
    let reg: Registry =
        from_reader(File::open("examples/R.json").unwrap()).unwrap();

    // Run transport from t=1 (Cut enabled) to t=3 (Cut disabled)
    let output = transport(&input, &reg, 1, 3).unwrap();

    let expected: Proof =
        from_reader(File::open("examples/prop_cut_out.json").unwrap()).unwrap();

    assert_eq!(
        output.root, expected.root,
        "Root mismatch: expected {}, got {}",
        expected.root, output.root
    );
    assert_eq!(output.nodes.len(), expected.nodes.len(), "Node count mismatch");
}
