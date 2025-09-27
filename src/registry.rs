use std::collections::HashSet;

/// All inference rules that the validator/transport may refer to.
/// (Phase 1: we keep the list broad; it’s OK if not all are used yet.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleId {
    Axiom,
    Id,
    BotL,
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

/// Minimal registry for Phase 1: a flat set of enabled rules.
/// We can evolve this in Phase 2 to time‑varying policies, provenance, etc.
#[derive(Debug, Clone)]
pub struct Registry {
    enabled: HashSet<RuleId>,
}

impl Default for Registry {
    fn default() -> Self {
        let mut set = HashSet::new();
        set.insert(RuleId::Axiom);
        set.insert(RuleId::Id);
        set.insert(RuleId::BotL);
        set.insert(RuleId::AndR);
        set.insert(RuleId::AndL1);
        set.insert(RuleId::AndL2);
        set.insert(RuleId::OrR1);
        set.insert(RuleId::OrR2);
        set.insert(RuleId::OrL);
        set.insert(RuleId::ImpR);
        set.insert(RuleId::ImpL);
        set.insert(RuleId::Cut);
        Self { enabled: set }
    }
}

impl Registry {
    /// Rules enabled at logical time `t`.
    /// Phase 1: same set for all `t`.
    pub fn enabled_at(&self, _t: u64) -> HashSet<RuleId> {
        self.enabled.clone()
    }

    /// Convenience helpers (optional, useful in tests or future code).
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
