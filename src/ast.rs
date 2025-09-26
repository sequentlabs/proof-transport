use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "tag", content = "fields")]
pub enum Term {
    Var(String),
    Func { name: String, args: Vec<Term> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sequent {
    pub ctx: Vec<Formula>,
    pub goal: Formula,
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
