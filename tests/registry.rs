use proof_transport::registry::{Registry, RuleId, TimeSlice};
use std::collections::HashSet;

#[test]
fn registry_enabled_at_works() {
    let reg = Registry {
        times: vec![
            TimeSlice { t: 0, enabled_rules: vec![RuleId::Id, RuleId::Cut] },
            TimeSlice { t: 1, enabled_rules: vec![RuleId::Id] },
        ],
    };

    let t0 = reg.enabled_at(0);
    let t1 = reg.enabled_at(1);

    assert!(t0.contains(&RuleId::Id) && t0.contains(&RuleId::Cut));
    assert!(t1.contains(&RuleId::Id) && !t1.contains(&RuleId::Cut));
}
