# Worked Example: root `Cut` elimination

**Input:** `examples/proof_with_cut.json`

---

root = "n0"  
n1: Id ⊢ A ⇒ A  
n2: Id ⊢ B ⇒ B  
n0: Cut( n1, n2 ) ⊢ A

---

```bash
# Run (CI/test does this automatically):
cargo test -q cut_elimination_rewrites_root_and_drops_fragility
