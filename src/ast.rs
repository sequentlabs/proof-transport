use serde::{de::Deserializer, Deserialize, Serialize};

/// -------------------------
/// Terms
/// -------------------------

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub enum Term {
    Var(String),
    Func { name: String, args: Vec<Term> },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "tag", content = "fields")]
enum TermTagged {
    Var(String),
    Func { name: String, args: Vec<Term> },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TermEither {
    Tagged(TermTagged),
    String(String),
}

impl<'de> Deserialize<'de> for Term {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match TermEither::deserialize(deserializer)? {
            TermEither::String(s) => Ok(Term::Var(s)),
            TermEither::Tagged(TermTagged::Var(s)) => Ok(Term::Var(s)),
            TermEither::Tagged(TermTagged::Func { name, args }) => Ok(Term::Func { name, args }),
        }
    }
}

/// -------------------------
/// Formulas
/// -------------------------

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[allow(clippy::large_enum_variant)]
pub enum Formula {
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

#[derive(Debug, Deserialize)]
#[serde(tag = "tag", content = "fields")]
enum FormulaTagged {
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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum FormulaEither {
    Tagged(FormulaTagged),
    String(String),
}

impl<'de> Deserialize<'de> for Formula {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match FormulaEither::deserialize(deserializer)? {
            // Treat any bare string as a variable formula.
            FormulaEither::String(s) => Ok(Formula::Var(s)),
            FormulaEither::Tagged(FormulaTagged::Var(s)) => Ok(Formula::Var(s)),
            FormulaEither::Tagged(FormulaTagged::Bot) => Ok(Formula::Bot),
            FormulaEither::Tagged(FormulaTagged::Top) => Ok(Formula::Top),
            FormulaEither::Tagged(FormulaTagged::Pred { name, args }) => {
                Ok(Formula::Pred { name, args })
            }
            FormulaEither::Tagged(FormulaTagged::And(a, b)) => Ok(Formula::And(a, b)),
            FormulaEither::Tagged(FormulaTagged::Or(a, b)) => Ok(Formula::Or(a, b)),
            FormulaEither::Tagged(FormulaTagged::Imp(a, b)) => Ok(Formula::Imp(a, b)),
            FormulaEither::Tagged(FormulaTagged::Forall(x, f)) => Ok(Formula::Forall(x, f)),
            FormulaEither::Tagged(FormulaTagged::Exists(x, f)) => Ok(Formula::Exists(x, f)),
        }
    }
}

/// -------------------------
/// Sequents
/// -------------------------

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Sequent {
    /// If omitted in JSON, defaults to `[]`.
    #[serde(default)]
    pub ctx: Vec<Formula>,
    pub goal: Formula,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SequentEither {
    // Normal object form
    Obj {
        #[serde(default)]
        ctx: Vec<Formula>,
        goal: Formula,
    },
    // A single formula means: ctx = [], goal = <that formula>
    OnlyFormula(Formula),
    // And for extra permissiveness: a bare string equals Var(string)
    OnlyString(String),
}

impl<'de> Deserialize<'de> for Sequent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match SequentEither::deserialize(deserializer)? {
            SequentEither::Obj { ctx, goal } => Ok(Sequent { ctx, goal }),
            SequentEither::OnlyFormula(goal) => Ok(Sequent { ctx: vec![], goal }),
            SequentEither::OnlyString(s) => Ok(Sequent {
                ctx: vec![],
                goal: Formula::Var(s),
            }),
        }
    }
}

/// -------------------------
/// Proofs
/// -------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Proof {
    pub nodes: Vec<ProofNode>,
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofNode {
    pub id: String,
    pub rule: String,
    /// If omitted, defaults to an empty list.
    #[serde(default)]
    pub premises: Vec<String>,
    pub sequent: Sequent,
}
