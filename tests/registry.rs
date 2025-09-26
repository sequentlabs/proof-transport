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

    let at0 = reg.enabled_at(0);
    let at1 = reg.enabled_at(1);

    assert!(at0.contains(&RuleId::Id));
    assert!(at0.contains(&RuleId::Cut));
    assert_eq!(at1, HashSet::from([RuleId::Id]));
}
