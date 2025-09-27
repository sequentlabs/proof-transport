use std::collections::HashSet;

/// Identifier for inference rules used across the crate (validator, transport, tests).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleId {
    Id,
    BotT,
    AndR,
    AndL1,
    AndL2,
    OrR1,
    OrR2,
    OrL,
    ImpR,
    ImpL,
    Cut,
}

/// A snapshot of which rules are enabled at logical time `t`.
/// Tests construct this with a struct literal, so keep the field names/public-ness.
#[derive(Debug, Clone)]
pub struct TimeSlice {
    pub t: u64,
    pub enabled_rules: Vec<RuleId>,
}

/// Registry of rule enablement over (logical) time.
/// Tests construct this with a struct literal, so keep `times` public.
#[derive(Debug, Clone)]
pub struct Registry {
    pub times: Vec<TimeSlice>,
}

impl Registry {
    /// Convenience constructor.
    pub fn new(times: Vec<TimeSlice>) -> Self {
        Self { times }
    }

    /// Set of rules enabled at logical time `t`.
    ///
    /// Contract expected by tests and transport:
    /// - Find the most recent `TimeSlice` with `ts.t <= t`.
    /// - If none exist, return empty set.
    /// - The slice’s `enabled_rules` is interpreted as the full set at that time.
    pub fn enabled_at(&self, t: u64) -> HashSet<RuleId> {
        self.times
            .iter()
            .rev()
            .find(|ts| ts.t <= t)
            .map(|ts| ts.enabled_rules.iter().copied().collect())
            .unwrap_or_else(HashSet::new)
    }
}

impl Default for Registry {
    /// Sensible default used by unit/golden tests:
    /// - At `t = 0`: all rules (including `Cut`) are enabled
    /// - At `t = 1`: everything except `Cut` (so transports to `t >= 1` will cut-eliminate)
    fn default() -> Self {
        use RuleId::*;
        let all = vec![Id, BotT, AndR, AndL1, AndL2, OrR1, OrR2, OrL, ImpR, ImpL, Cut];
        let no_cut = vec![Id, BotT, AndR, AndL1, AndL2, OrR1, OrR2, OrL, ImpR, ImpL];

        Self {
            times: vec![
                TimeSlice { t: 0, enabled_rules: all },
                TimeSlice { t: 1, enabled_rules: no_cut },
            ],
        }
    }
}
