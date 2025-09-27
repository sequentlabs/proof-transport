//! Simple rule registry with time‑sliced enablement (Phase‑1).

/// Identifiers for the proof rules used across the crate.
/// Keep this list in sync with `validator::rule_from_str(...)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// A point-in-time snapshot of which rules are enabled.
///
/// The test suite constructs this with field names `t` and `enabled_rules`,
/// so we expose exactly those to stay source-compatible with tests.
#[derive(Debug, Clone)]
pub struct TimeSlice {
    /// Logical time of this snapshot.
    pub t: u64,
    /// Rules enabled at (and after) this `t` until the next slice.
    pub enabled_rules: Vec<RuleId>,
}

/// Registry holds an ordered list of time slices.
/// Tests construct this via a struct literal `{ times: vec![...] }`,
/// so `times` is public and uses that exact name.
#[derive(Debug, Clone, Default)]
pub struct Registry {
    /// Chronologically sorted time slices (ascending by `t`).
    pub times: Vec<TimeSlice>,
}

impl Registry {
    /// Return the set of rules enabled at logical time `when`.
    ///
    /// Semantics (Phase‑1):
    /// - If there is at least one slice with `t <= when`, the *last* such
    ///   slice applies.
    /// - If there is none, no rules are enabled.
    pub fn enabled_at(&self, when: u64) -> Vec<RuleId> {
        // Assumption (kept simple for Phase‑1): `times` is already sorted by `t`.
        // If not sorted, the linear scan still returns the last qualifying slice.
        let mut last_idx: Option<usize> = None;
        for (i, s) in self.times.iter().enumerate() {
            if s.t <= when {
                last_idx = Some(i);
            } else {
                break;
            }
        }
        match last_idx {
            Some(i) => self.times[i].enabled_rules.clone(),
            None => Vec::new(),
        }
    }
}
