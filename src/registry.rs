//! Registry: rule enablement over (logical) time.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleId {
    Id,
    BotI,
    AndR,
    AndL,
    OrR1,
    OrR2,
    OrL1,
    OrL2,
    ImpR, // ← standard names
    ImpL, // ← standard names
    Cut,
}

/// A time slice declares which rules are enabled starting at logical time `t`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TimeSlice {
    pub t: u64,
    pub enabled_rules: Vec<RuleId>,
}

/// A registry is an ordered list of time slices.
/// The last slice with `t <= query_time` determines the set of enabled rules.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Registry {
    pub times: Vec<TimeSlice>,
}

impl Registry {
    /// Return the enabled rules at logical time `t`.
    /// If there is no slice with `t_i <= t`, return an empty set.
    pub fn enabled_at(&self, t: u64) -> Vec<RuleId> {
        self.times
            .iter()
            .take_while(|ts| ts.t <= t)
            .last()
            .map(|ts| ts.enabled_rules.clone())
            .unwrap_or_default()
    }
}
