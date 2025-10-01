# Proof-Transport – Pitch

## Problem
Formal proofs are fragile: when a proof assistant kernel evolves (e.g. a rule like `Cut` is disabled), existing proofs can break. This creates “proof rot” and forces teams to re-verify work at high cost.

## Solution
Proof-Transport is a lightweight open-source engine that:
- **Eliminates unstable rules** (e.g. Cut) and transports proofs across versions
- **Issues stability certificates** showing when a proof remains valid
- **Provides a live demo** (GitHub Pages) and JSON-based schema for reproducibility

## Why It Matters
- **Verification teams** (Lean/Coq/Agda): maintain proof validity across kernel updates  
- **Auditors** (crypto/smart contracts): reduce cost of re-audit when standards evolve  
- **AI safety / compliance**: preserve long-lived assurance arguments under changing specs

## Current Status
- ✅ Core transport implemented  
- ✅ JSON schemas + golden examples  
- ✅ CI pipeline + integration test scaffolding  
- ✅ Web demo explorer online  
- ✅ First public release (`v0.1.0`) tagged

## Roadmap
- v0.2: FO correctness + golden tests  
- v0.3: Exporters (Lean/Coq) + benchmarks  
- v1.0: Polished web explorer + external trials

## Why Fund Us
We’ve already shipped a working kernel, demo, and reproducible artifact.  
Funding will accelerate the roadmap: FO correctness, exporters, benchmarks, and wider community adoption.

---

*Repository:* [github.com/sequentlabs/proof-transport](https://github.com/sequentlabs/proof-transport)  
*Live demo:* [sequentlabs.github.io/proof-transport](https://sequentlabs.github.io/proof-transport)
