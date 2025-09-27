use std::collections::HashSet;

/// Rule identifiers used throughout Phase‑1.
/// (Names match tests & JSON exactly.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleId {
    Id,
    BotI,
    AndL1,
    AndL2,
    AndR,
    OrL,
    Or1,
    Or2,
    ImpL,
    ImpR,
    Cut,
}

/// A point-in-time rule configuration used by tests:
/// TimeSlice { t, enabled_rules }
///
/// IMPORTANT: tests build TimeSlice with `vec![..]`, so we store a Vec here
/// and convert to a HashSet when answering queries.
#[derive(Debug, Clone)]
pub struct TimeSlice {
    pub t: u64,
    pub enabled_rules: Vec<RuleId>,
}

impl Default for TimeSlice {
    fn default() -> Self {
        Self {
            t: 0,
            enabled_rules: Vec::new(),
        }
    }
}

/// Registry holds an ordered set of time slices.
/// Phase‑1 needs only "what is enabled at logical time t".
#[derive(Debug, Default, Clone)]
pub struct Registry {
    pub times: Vec<TimeSlice>,
}

impl Registry {
    /// Return the set of rules enabled at logical time `t`.
    /// Semantics: last slice with `slice.t <= t` wins.
    pub fn enabled_at(&self, t: u64) -> HashSet<RuleId> {
        let mut current: HashSet<RuleId> = HashSet::new();
        for slice in &self.times {
            if slice.t <= t {
                // Replace with the slice's rules, de-duped via HashSet.
                current = slice.enabled_rules.iter().copied().collect();
            } else {
                break;
            }
        }
        current
    }
}
