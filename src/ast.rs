use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// --- Terms -----------------------------------------------------------------

/// Accept either the structured `{ tag, fields }` or a permissive string form.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Term {
    /// Strict / schema form.
    Node(TermNode),
    /// Permissive: a bare string (keeps CI examples happy).
    Text(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "tag", content = "fields")]
pub enum TermNode {
    Var(String),
    Func { name: String, args: Vec<Term> },
}

/// --- Formulas ---------------------------------------------------------------

/// Accept either the structured `{ tag, fields }` or a permissive string form.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Formula {
    /// Strict / schema form.
    Node(FormulaNode),
    /// Permissive: a bare string such as "A", "(A ⇒ A)", etc.
    Text(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "tag", content = "fields")]
pub enum FormulaNode {
    Var(String),
    Bot,
    Top,
    Pred { name: String, args: Vec<Term> },
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Imp(Box<Formula>, Box<Formula>),
    Forall(String, Box<Formula>),
    Exists(String, Box<Formula>),
}

/// --- Sequents ---------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequent {
    pub ctx: Vec<Formula>,
    /// We serialize as `thm`, but accept `goal` on input as well.
    pub thm: Formula,
}

/// Helper used only for deserialization: either full object or shorthand.
#[derive(Deserialize)]
#[serde(untagged)]
enum SequentDe {
    // Full: { ctx?, thm }  (ctx defaults to [])
    Full {
        #[serde(default)]
        ctx: Vec<Formula>,
        #[serde(rename = "thm", alias = "goal")]
        thm: Formula,
    },
    // Shorthand: just a formula like "(A ⇒ A)" meaning ctx=[]
    Shorthand(Formula),
}

impl From<SequentDe> for Sequent {
    fn from(s: SequentDe) -> Self {
        match s {
            SequentDe::Full { ctx, thm } => Sequent { ctx, thm },
            SequentDe::Shorthand(thm) => Sequent { ctx: Vec::new(), thm },
        }
    }
}

impl<'de> Deserialize<'de> for Sequent {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(SequentDe::deserialize(d)?.into())
    }
}

impl Serialize for Sequent {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Full<'a> {
            ctx: &'a [Formula],
            #[serde(rename = "thm")]
            thm: &'a Formula,
        }
        Full { ctx: &self.ctx, thm: &self.thm }.serialize(s)
    }
}

/// --- Proofs -----------------------------------------------------------------

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
