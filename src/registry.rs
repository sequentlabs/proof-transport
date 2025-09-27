use std::collections::HashSet;

/// All inference rules our transport cares about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleId {
    Id,
    BotI,
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

/// Logical time (Phase‑1 keeps this as a simple counter).
pub type Time = u64;

/// A change point in the rule‑enablement timeline.
/// At logical time `at`, the set of enabled rules becomes `enabled`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeSlice {
    pub at: Time,
    pub enabled: Vec<RuleId>,
}

/// Rule‑enablement registry over logical time.
/// Tests construct this with a struct literal, so `times` must be public.
#[derive(Debug, Clone, Default)]
pub struct Registry {
    pub times: Vec<TimeSlice>,
}

impl Registry {
    /// Return the set of rules enabled at logical time `t`.
    /// If no slice exists at or before `t`, returns the empty set.
    pub fn enabled_at(&self, t: Time) -> HashSet<RuleId> {
        // Find the latest slice whose `at` <= t
        let mut latest: Option<&TimeSlice> = None;
        for ts in &self.times {
            if ts.at <= t {
                match latest {
                    Some(best) if best.at >= ts.at => {} // keep best
                    _ => latest = Some(ts),
                }
            }
        }

        match latest {
            Some(ts) => ts.enabled.iter().copied().collect(),
            None => HashSet::new(),
        }
    }
}
