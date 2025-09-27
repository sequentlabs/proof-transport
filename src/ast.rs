// src/ast.rs
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value; // for tolerant 3-tuple sequent [ctx, <sep>, thm]

/// ============================
/// Terms
/// ============================

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

/// ============================
/// Formulas
/// ============================

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

/// ============================
/// Sequents
/// ============================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequent {
    pub ctx: Vec<Formula>,
    /// Canonical output field name is `thm`; we accept `goal` on input.
    pub thm: Formula,
}

/// Accept three input encodings for a sequent:
///   1) object form: { ctx?: [...]|formula, thm|goal: formula }
///   2) tuple/array: [ ctx[..], thm ]
///   3) shorthand:   "φ"   (means ctx = [])
#[derive(Deserialize)]
#[serde(untagged)]
enum SequentDe {
    Full(FullDe),
    // Use the built-in tuple type; Serde implements Deserialize for (A, B).
    TupleMany((Vec<Formula>, Formula)),             // [ [ctx...], thm ]
    TupleOne((Formula, Formula)),                   // [   ctx   , thm ]
    TripleMany((Vec<Formula>, Value, Formula)),     // [ [ctx...], <sep>, thm ]
    TripleOne((Formula, Value, Formula)),           // [   ctx   , <sep>, thm ]
    Shorthand(Formula),
}

/// `ctx` may be either an array of formulas or a single formula.
#[derive(Deserialize)]
#[serde(untagged)]
enum CtxDe {
    Many(Vec<Formula>),
    One(Formula),
}

#[derive(Deserialize)]
struct FullDe {
    #[serde(
        default,
        rename = "ctx",
        alias = "context",
        alias = "ants",
        alias = "assumptions"
    )]
    ctx: Option<CtxDe>,
    #[serde(rename = "thm", alias = "goal")]
    thm: Formula,
}

impl From<SequentDe> for Sequent {
    fn from(s: SequentDe) -> Self {
        match s {
            SequentDe::Full(FullDe { ctx, thm }) => {
                let ctx_vec = match ctx {
                    None => Vec::new(),
                    Some(CtxDe::Many(v)) => v,
                    Some(CtxDe::One(f)) => vec![f],
                };
                Sequent { ctx: ctx_vec, thm }
            }
            SequentDe::TupleMany((ctx, thm)) => Sequent { ctx, thm },
            SequentDe::TupleOne((ctx1, thm)) => Sequent { ctx: vec![ctx1], thm },
            SequentDe::TripleMany((ctx, _sep, thm)) => Sequent { ctx, thm },
            SequentDe::TripleOne((ctx1, _sep, thm)) => Sequent { ctx: vec![ctx1], thm },
            SequentDe::Shorthand(thm) => Sequent {
                ctx: Vec::new(),
                thm,
            },
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
        // Always write the canonical long form so the schema is stable.
        #[derive(Serialize)]
        struct Full<'a> {
            ctx: &'a [Formula],
            #[serde(rename = "thm")]
            thm: &'a Formula,
        }
        Full {
            ctx: &self.ctx,
            thm: &self.thm,
        }
        .serialize(s)
    }
}

/// ============================
/// Proofs
/// ============================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Proof {
    pub nodes: Vec<ProofNode>,
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofNode {
    pub id: String,
    pub rule: String,
    #[serde(default, rename = "premises", alias = "prems")]
    pub premises: Vec<String>,
    #[serde(rename = "sequent", alias = "seq")]
    pub sequent: Sequent,
}
