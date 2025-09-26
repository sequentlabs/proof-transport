use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RuleId {
    Id, BotL, AndR, AndL1, AndL2, OrR1, OrR2, OrL, ImpR, ImpL, Cut,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlice {
    pub t: u64,
    pub enabled_rules: Vec<RuleId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub times: Vec<TimeSlice>,
}

impl Registry {
    pub fn enabled_at(&self, t: u64) -> HashSet<RuleId> {
        let mut best: Option<&TimeSlice> = None;
        for ts in &self.times {
            if ts.t <= t { best = Some(ts); }
        }
        best.map(|ts| ts.enabled_rules.iter().cloned().collect())
            .unwrap_or_default()
    }
}
