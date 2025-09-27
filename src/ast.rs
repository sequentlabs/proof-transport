use serde::{Deserialize, Serialize};

//
// Terms
//
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "tag", content = "fields")]
pub enum Term {
    Var(String),
    Func { name: String, args: Vec<Term> },
}

// Helper shape for object-form Term deserialization
#[derive(Deserialize)]
#[serde(tag = "tag", content = "fields")]
enum RawTerm {
    Var(String),
    Func { name: String, args: Vec<Term> },
}

// Accept either tagged-object or string (string -> Var)
impl<'de> serde::Deserialize<'de> for Term {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper {
            Str(String),
            Obj(RawTerm),
        }

        match Helper::deserialize(de)? {
            Helper::Str(s) => Ok(Term::Var(s)),
            Helper::Obj(RawTerm::Var(s)) => Ok(Term::Var(s)),
            Helper::Obj(RawTerm::Func { name, args }) => Ok(Term::Func { name, args }),
        }
    }
}

//
// Formulae
//
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "tag", content = "fields")]
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

// Helper shape for object-form Formula deserialization
#[derive(Deserialize)]
#[serde(tag = "tag", content = "fields")]
enum RawFormula {
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

// Accept either tagged-object or string (string -> Var)
impl<'de> serde::Deserialize<'de> for Formula {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper {
            Str(String),
            Obj(RawFormula),
        }

        Ok(match Helper::deserialize(de)? {
            Helper::Str(s) => Formula::Var(s),
            Helper::Obj(RawFormula::Var(s)) => Formula::Var(s),
            Helper::Obj(RawFormula::Bot) => Formula::Bot,
            Helper::Obj(RawFormula::Top) => Formula::Top,
            Helper::Obj(RawFormula::Pred { name, args }) => Formula::Pred { name, args },
            Helper::Obj(RawFormula::And(a, b)) => Formula::And(a, b),
            Helper::Obj(RawFormula::Or(a, b)) => Formula::Or(a, b),
            Helper::Obj(RawFormula::Imp(a, b)) => Formula::Imp(a, b),
            Helper::Obj(RawFormula::Forall(x, f)) => Formula::Forall(x, f),
            Helper::Obj(RawFormula::Exists(x, f)) => Formula::Exists(x, f),
        })
    }
}

//
// Sequents
//
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Sequent {
    pub ctx: Vec<Formula>,
    pub goal: Formula,
}

// Accept either object-form { goal, ctx? } or string-form "phi" (treated as goal; ctx := [])
impl<'de> serde::Deserialize<'de> for Sequent {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper {
            // object form; ctx optional -> defaults to []
            Obj {
                #[serde(default)]
                ctx: Vec<Formula>,
                goal: Formula,
            },
            // string form; interpret as goal; ctx := []
            Str(String),
        }

        match Helper::deserialize(de)? {
            Helper::Obj { ctx, goal } => Ok(Sequent { ctx, goal }),
            Helper::Str(s) => Ok(Sequent { ctx: Vec::new(), goal: Formula::Var(s) }),
        }
    }
}

//
// Proofs
//
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
