use proof_transport::registry::{Registry, RuleId, TimeSlice};
use std::collections::HashSet;

#[test]
fn registry_enabled_at_works() {
    let reg = Registry {
        times: vec![
            TimeSlice {
                t: 0,
                enabled_rules: vec![RuleId::Id, RuleId::Cut],
            },
            TimeSlice {
                t: 1,
                enabled_rules: vec![RuleId::Id],
            },
        ],
    };

    let at0: HashSet<_> = reg.enabled_at(0);
    assert!(at0.contains(&RuleId::Cut));

    let at1: HashSet<_> = reg.enabled_at(1);
    assert!(!at1.contains(&RuleId::Cut));
}
