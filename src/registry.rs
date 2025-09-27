use std::collections::{HashMap, HashSet};

/// Rules the registry can enable/disable over time.
/// Extend this enum as new rules are introduced.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RuleId {
    Cut,
}

/// A tiny, time-indexed rule registry.
///
/// Internally we keep, per rule, a chronologically ordered list of
/// `(timestamp, enabled)` toggles. At any time `t`, the effective state
/// is the last toggle at or before `t`. If there is no toggle, we treat
/// the rule as **enabled by default** (defensive, lenient baseline).
#[derive(Clone, Debug)]
pub struct Registry {
    toggles: HashMap<RuleId, Vec<(u64, bool)>>,
}

impl Registry {
    /// Create an empty registry (no toggles recorded).
    /// With no toggles, all rules are treated as **enabled**.
    pub fn new() -> Self {
        Self {
            toggles: HashMap::new(),
        }
    }

    /// Convenience default used in tests and examples.
    /// Equivalent to [`Registry::new`].
    pub fn default() -> Self {
        Self::new()
    }

    /// Record that `rule` becomes enabled at logical time `at`.
    pub fn enable(&mut self, rule: RuleId, at: u64) {
        let v = self.toggles.entry(rule).or_default();
        v.push((at, true));
        v.sort_by_key(|(ts, _)| *ts);
    }

    /// Record that `rule` becomes disabled at logical time `at`.
    pub fn disable(&mut self, rule: RuleId, at: u64) {
        let v = self.toggles.entry(rule).or_default();
        v.push((at, false));
        v.sort_by_key(|(ts, _)| *ts);
    }

    /// Return the set of rules enabled at logical time `t`.
    ///
    /// Default policy: if a rule has **no** recorded toggle by time `t`,
    /// we consider it **enabled** (so the system behaves permissively
    /// unless a rule is explicitly turned off).
    pub fn enabled_at(&self, t: u64) -> HashSet<RuleId> {
        let mut enabled: HashSet<RuleId> = HashSet::new();

        // Start from the permissive baseline: all known rules "on" unless disabled.
        // For now we only have `Cut`; add more as new rules appear.
        enabled.insert(RuleId::Cut);

        // Apply any recorded toggles up to time `t`.
        for (rule, changes) in &self.toggles {
            let mut current = None;
            for (ts, on) in changes.iter().filter(|(ts, _)| *ts <= t) {
                current = Some((*ts, *on));
            }
            if let Some((_, on)) = current {
                if on {
                    enabled.insert(*rule);
                } else {
                    enabled.remove(rule);
                }
            }
        }

        enabled
    }
}

/// Standard library-style default so callers can do `Registry::default()`
/// and so `Default::default()` works in generics.
impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
