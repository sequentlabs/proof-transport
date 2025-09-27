use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize};

//
// Terms
//

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "tag", content = "fields")] // canonical serialization form
pub enum Term {
    Var(String),
    Func { name: String, args: Vec<Term> },
}

// Accept both canonical {tag,fields} and permissive shorthands (e.g. "x")
impl<'de> Deserialize<'de> for Term {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "tag", content = "fields")]
        enum Canon {
            Var(String),
            Func { name: String, args: Vec<Term> },
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Any {
            Canon(Canon),
            // Shorthand for variables: "x"
            Str(String),
            // Alternative external-tag-like spellings people sometimes write:
            VarObj { Var: String },
            FuncObj { Func: FuncAux },
        }
        #[derive(Deserialize)]
        struct FuncAux {
            name: String,
            args: Vec<Term>,
        }

        match Any::deserialize(de)? {
            Any::Canon(Canon::Var(v)) => Ok(Term::Var(v)),
            Any::Canon(Canon::Func { name, args }) => Ok(Term::Func { name, args }),
            Any::Str(s) => Ok(Term::Var(s)),
            Any::VarObj { Var: v } => Ok(Term::Var(v)),
            Any::FuncObj {
                Func: FuncAux { name, args },
            } => Ok(Term::Func { name, args }),
        }
    }
}

//
// Formulae
//

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "tag", content = "fields")] // canonical serialization form
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

// Accept canonical {tag,fields} *and* permissive shorthands:
//  - a plain string is treated as a propositional variable
//  - external-tag-like spellings {Var: "..."} or {Pred: {name, args}}.
impl<'de> Deserialize<'de> for Formula {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "tag", content = "fields")]
        enum Canon {
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

        #[derive(Deserialize)]
        struct PredAux {
            name: String,
            args: Vec<Term>,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Any {
            Canon(Canon),
            // Plain string => variable
            Str(String),
            // External-tag-like alternates
            VarObj { Var: String },
            PredObj { Pred: PredAux },
            AndObj { And: (Box<Formula>, Box<Formula>) },
            OrObj { Or: (Box<Formula>, Box<Formula>) },
            ImpObj { Imp: (Box<Formula>, Box<Formula>) },
            ForallObj { Forall: (String, Box<Formula>) },
            ExistsObj { Exists: (String, Box<Formula>) },
            BotObj { Bot: () },
            TopObj { Top: () },
        }

        Ok(match Any::deserialize(de)? {
            Any::Canon(Canon::Var(v)) => Formula::Var(v),
            Any::Canon(Canon::Bot) | Any::BotObj { Bot: () } => Formula::Bot,
            Any::Canon(Canon::Top) | Any::TopObj { Top: () } => Formula::Top,
            Any::Canon(Canon::Pred { name, args }) => Formula::Pred { name, args },
            Any::Canon(Canon::And(a, b)) | Any::AndObj { And: (a, b) } => Formula::And(a, b),
            Any::Canon(Canon::Or(a, b)) | Any::OrObj { Or: (a, b) } => Formula::Or(a, b),
            Any::Canon(Canon::Imp(a, b)) | Any::ImpObj { Imp: (a, b) } => Formula::Imp(a, b),
            Any::Canon(Canon::Forall(x, f)) | Any::ForallObj { Forall: (x, f) } => {
                Formula::Forall(x, f)
            }
            Any::Canon(Canon::Exists(x, f)) | Any::ExistsObj { Exists: (x, f) } => {
                Formula::Exists(x, f)
            }
            Any::VarObj { Var: v } | Any::Str(v) => Formula::Var(v),
            Any::PredObj { Pred: PredAux { name, args } } => Formula::Pred { name, args },
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

// Accept:
//   - canonical object { ctx: [...], goal: ... }
//   - object { goal: ... } (ctx defaults to [])
//   - bare Formula (interpreted as goal with empty ctx)
impl<'de> Deserialize<'de> for Sequent {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Canon {
            #[serde(default)]
            ctx: Vec<Formula>,
            goal: Formula,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Any {
            Canon(Canon),
            GoalOnly { goal: Formula },
            JustGoal(Formula),
        }

        match Any::deserialize(de)? {
            Any::Canon(Canon { ctx, goal }) => Ok(Sequent { ctx, goal }),
            Any::GoalOnly { goal } => Ok(Sequent { ctx: vec![], goal }),
            Any::JustGoal(goal) => Ok(Sequent { ctx: vec![], goal }),
        }
    }
}

//
// Proof trees
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
