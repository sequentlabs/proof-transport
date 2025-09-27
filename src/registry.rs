use std::collections::HashSet;

/// All inference rules we track in the registry.
/// Keep these variant names stable — tests and other modules rely on them.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RuleId {
    Id,
    BotI,
    AndR,
    AndL1,
    AndL2,
    OrR1,
    OrR2,
    OrL,
    ImplR,
    ImplL,
    Cut,
}

/// A point-in-time snapshot of which rules are enabled.
///
/// Tests construct this with a struct literal, so the fields must be `pub`.
#[derive(Clone, Debug, Default)]
pub struct TimeSlice {
    /// Logical time of this slice.
    pub at: u64,
    /// Rules enabled at (and after) this instant until the next slice.
    pub enabled: HashSet<RuleId>,
}

/// Registry is the piecewise-constant enablement schedule over logical time.
///
/// Tests build this via `Registry { times: vec![...] }`, so the field is `pub`.
#[derive(Clone, Debug, Default)]
pub struct Registry {
    pub times: Vec<TimeSlice>,
}

impl Registry {
    /// Return the set of enabled rules that applies at logical time `t`.
    ///
    /// We select the last slice with `at <= t`. If none applies, return empty.
    pub fn enabled_at(&self, t: u64) -> HashSet<RuleId> {
        // assumes `times` is in non-decreasing order of `at`
        let mut chosen: Option<&TimeSlice> = None;
        for s in &self.times {
            if s.at <= t {
                chosen = Some(s);
            } else {
                break;
            }
        }
        chosen
            .map(|s| s.enabled.clone())
            .unwrap_or_else(HashSet::new)
    }
}
