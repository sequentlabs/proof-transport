# Proof-Transport: Architecture (v0.1)

**Goal:** keep proofs valid when the proof kernel evolves (e.g., disabling `Cut`).

## Modules

- `ast.rs`: JSON model for terms, formulas, sequents, and proof graphs.
- `registry.rs`: time-indexed rule registry; query `enabled_at(t)`.
- `validator.rs`: lightweight local checks (node ids, rules available, references).
- `cutelim.rs`: current toy *root-cut* elimination to demonstrate rewrite.
- `frag.rs`: toy fragility score = `nodes.len() + 10 * (#Cut nodes)`.
- `lib.rs`: crate exports.

## Transport sketch

Given `(proof.json, registry.json, from=t, to=t')`:

1. Parse & validate proof (shape + local well-formedness).
2. If `Cut` disabled at `t'`, apply cut-elimination steps.
3. Compute fragility before/after; ensure score does not worsen.
4. Output transported proof JSON.

**This repository currently demonstrates (1), (2) for root cuts, and (3) via tests.**
