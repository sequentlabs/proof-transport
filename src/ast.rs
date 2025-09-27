// src/ast.rs
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error as DeError;
use serde_json::Value; // used by the robust Sequent deserializer

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

// === Sequents ================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequent {
    pub ctx: Vec<Formula>,
    /// Canonical output field name is `thm`; we accept many aliases on input.
    pub thm: Formula,
}

impl<'de> Deserialize<'de> for Sequent {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = Value::deserialize(d)?;

        // Parse a single Formula from a JSON value.
        fn parse_formula<E: DeError>(v: Value) -> Result<Formula, E> {
            serde_json::from_value::<Formula>(v).map_err(E::custom)
        }

        // Parse ctx as either an array of formulas or a single formula.
        fn parse_ctx<E: DeError>(v: Value) -> Result<Vec<Formula>, E> {
            match v {
                Value::Null => Ok(Vec::new()),
                Value::Array(items) => items
                    .into_iter()
                    .map(|x| serde_json::from_value::<Formula>(x).map_err(E::custom))
                    .collect(),
                other => Ok(vec![serde_json::from_value::<Formula>(other).map_err(E::custom)?]),
            }
        }

        // Accept multiple encodings:
        //  1) "φ"                        (shorthand; ctx=[])
        //  2) [ctx, thm]                 (ctx can be a single formula or an array)
        //  3) [ctx, <sep>, thm]          (<sep> ignored, e.g., "⊢", "=>")
        //  4) { ctx?: [...]/formula, thm|goal|rhs|succ|conclusion|... }
        let seq = match v {
            // 1) Shorthand string
            Value::String(s) => Sequent { ctx: Vec::new(), thm: Formula::Text(s) },

            // 2) and 3) Tuple forms
            Value::Array(mut arr) => match arr.len() {
                2 => {
                    let thm = parse_formula::<D::Error>(arr.remove(1))?;
                    let ctx = parse_ctx::<D::Error>(arr.remove(0))?;
                    Sequent { ctx, thm }
                }
                3 => {
                    let thm = parse_formula::<D::Error>(arr.remove(2))?;
                    let ctx = parse_ctx::<D::Error>(arr.remove(0))?;
                    // arr[1] is a separator like "⊢" or "=>"; ignore.
                    Sequent { ctx, thm }
                }
                _ => return Err(D::Error::custom("Sequent array must be [ctx, thm] or [ctx, sep, thm]")),
            },

            // 4) Object form with tolerant key names and a fallback heuristic
            Value::Object(mut obj) => {
                // Common synonyms seen in examples / golden data.
                const THM_KEYS: &[&str] = &[
                    "thm", "goal", "rhs", "succ", "succedent", "conclusion", "cons", "consequent",
                ];
                const CTX_KEYS: &[&str] = &[
                    "ctx", "context", "ants", "assumptions", "lhs", "left", "antecedent", "gamma", "Γ",
                ];

                // Try the synonyms first.
                let mut thm_opt = THM_KEYS.iter().find_map(|k| obj.remove(*k));
                let mut ctx_opt = CTX_KEYS.iter().find_map(|k| obj.remove(*k));

                // Fallback heuristic if keys aren't present:
                // pick an array value as ctx and a non-array as thm.
                if thm_opt.is_none() {
                    thm_opt = obj
                        .iter()
                        .find_map(|(_, v)| if !v.is_array() { Some(v.clone()) } else { None });
                }
                if ctx_opt.is_none() {
                    ctx_opt = obj
                        .iter()
                        .find_map(|(_, v)| if v.is_array() { Some(v.clone()) } else { None });
                }

                let thm_val = thm_opt.ok_or_else(|| {
                    D::Error::custom(
                        "Sequent object missing a recognizable theorem field (e.g., thm/goal/rhs/succ/conclusion)",
                    )
                })?;

                let ctx = match ctx_opt {
                    Some(v) => parse_ctx::<D::Error>(v)?,
                    None => Vec::new(),
                };

                let thm = parse_formula::<D::Error>(thm_val)?;
                Sequent { ctx, thm }
            }

            other => {
                return Err(D::Error::custom(format!(
                    "invalid sequent value: expected string/array/object, got {other:?}"
                )))
            }
        };

        Ok(seq)
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
        Full { ctx: &self.ctx, thm: &self.thm }.serialize(s)
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
