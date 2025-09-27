use serde::{de, Deserialize, Deserializer, Serialize};

/// Phase‑1 treats formulas and terms as opaque text.
/// (Examples and the JSON schema feed plain strings.)
pub type Term = String;
pub type Formula = String;

/// A sequent. In our examples this is usually an object
/// `{ "ctx": [...], "goal": "..." }`, but some historical files
/// encode it as just a string (goal only). We accept *both*.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Sequent {
    pub ctx: Vec<Formula>,
    pub goal: Formula,
}

/// Internal helper representation used only for deserialization.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SequentRepr {
    // Normal object form
    Obj { ctx: Vec<Formula>, goal: Formula },
    // Legacy / minimal form: just a goal string
    GoalOnly(Formula),
}

impl<'de> Deserialize<'de> for Sequent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match SequentRepr::deserialize(deserializer)? {
            SequentRepr::Obj { ctx, goal } => Ok(Sequent { ctx, goal }),
            SequentRepr::GoalOnly(goal) => Ok(Sequent { ctx: Vec::new(), goal }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Proof {
    pub nodes: Vec<ProofNode>,
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofNode {
    pub id: String,
    pub rule: String,
    #[serde(default)]
    pub premises: Vec<String>,
    pub sequent: Sequent,
}
