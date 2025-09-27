use std::collections::HashSet;

/// All inference rules that the validator/transport refer to.
/// Keep this list in sync with the places that do structural checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleId {
    Axiom,
    Cut,
    // disjunction
    OrL,
    OrR1,
    OrR2,
    // implication
    ImpL,
    ImpR,
}

/// Minimal registry: a set of rules that are enabled at all logical times.
/// (Phase 1 only needs a flat set; we can evolve this to per-time policies in Phase 2.)
#[derive(Debug, Clone)]
pub struct Registry {
    enabled: HashSet<RuleId>,
}

impl Default for Registry {
    fn default() -> Self {
        let mut set = HashSet::new();
        set.insert(RuleId::Axiom);
        set.insert(RuleId::Cut);
        set.insert(RuleId::OrL);
        set.insert(RuleId::OrR1);
        set.insert(RuleId::OrR2);
        set.insert(RuleId::ImpL);
        set.insert(RuleId::ImpR);
        Self { enabled: set }
    }
}

impl Registry {
    /// Return the set of rules enabled at logical time `t`.
    /// Phase 1: same set for all `t`.
    pub fn enabled_at(&self, _t: u64) -> HashSet<RuleId> {
        self.enabled.clone()
    }

    /// Convenience helpers (not strictly required by Phase 1)
    pub fn enables(&self, r: RuleId) -> bool {
        self.enabled.contains(&r)
    }
    pub fn enable(&mut self, r: RuleId) {
        self.enabled.insert(r);
    }
    pub fn disable(&mut self, r: RuleId) {
        self.enabled.remove(&r);
    }
}

