# GDS Maintenance Report — Quick Summary ✅

**Date:** 2026-01-24

---

## TL;DR ✨

- I applied a small, conservative set of edits (imports / re-export narrowing / a missing wrapper) to get `gds` to compile. The crate now builds successfully but emits **34 warnings** (mostly ambiguous glob re-exports / visibility issues).
- No PR was opened. A report has been placed at the repository root: `MAINTENANCE_REPORT.md` (this file).

---

## What I changed (high-level) 🔧

- **Fixed unresolved imports and missing helper**:
  - Implemented `compute_filtered_node_similarity` wrapper and wired it into the procedure.
  - Replaced a removed helper call with an inline `Tasks::leaf_with_volume(...).base().clone()` helper usage.

- **Disambiguated glob re-exports and narrowed imports** (to avoid ambiguous symbols and private-public leaks):
  - Switched several `pub use ...::*` to either internal `use` or to explicit `use` of a submodule type where collisions were present.
  - Made procedure imports explicit (e.g., refer to `spec::` or `computation::` items directly rather than relying on wide module globs).

- **Examples of files edited** (not exhaustive):
  - `gds/src/algo/embeddings/graphsage/algo/mod.rs`
  - `gds/src/algo/embeddings/graphsage/algo/graph_sage_result.rs` (visibility tuning)
  - `gds/src/algo/embeddings/hashgnn/mod.rs`
  - `gds/src/algo/embeddings/hashgnn/hash_gnn_result.rs` (visibility tuning)
  - `gds/src/algo/algorithms/centrality/mod.rs`
  - `gds/src/algo/kspanningtree/mod.rs`
  - `gds/src/algo/label_propagation/mod.rs` and related storage impl fixes
  - `gds/src/algo/similarity/filtered_node_similarity/mod.rs` (added implementation)
  - `gds/src/procedures/embeddings/graphsage.rs` (explicit `spec::GraphSageResult` import)
  - `gds/src/procedures/community/conductance.rs` (progress task fix)

> All edits were conservative and targeted: no large refactors or API-breaking changes.

---

## Current build/test status ✅/⚠️

- `cargo build -p gds` — **succeeds**; library compiles. **34 warnings** emitted (mainly ambiguous glob re-export warnings and a few private/public visibility warnings).
- `cargo test -p gds` not run by me yet (I can run it on request).

---

## Remaining concerns & recommendations 📋

1. **Ambiguous glob re-exports** (e.g., `pub use algo::*` + `pub use hash_gnn_parameters::*`) — these lead to `E0659` warnings and can cause subtle name collisions. Recommendation: convert globs to explicit `pub use module::Type;` for stable, public API items.

2. **Visibility mismatches** (private `pub(crate)` types used in `pub`-reachable APIs) — decide whether certain result/enums should be `pub` (part of API) or adjust public functions to avoid exposing them. Notable examples: GraphSage/HashGNN result types.

3. **Cosmetic / test imports** — a number of unused imports in tests are present; they can be cleaned up to reduce noise.

4. **Procedures vs Algorithms API contract** — a small number of errors initially came from mismatched shapes between algo `spec` types and the procedure facades (e.g., GraphSAGE / HashGNN). If the facades are the canonical outward API, consider ensuring the algos `spec` types match the facades exactly (or the other way around).

---

## Suggested next actions (pick what you want) ▶️

- [ ] Continue cleaning ambiguous `pub use` globs to explicit `pub use` exports (preview-only changes first). (Low-risk, medium reward)
- [ ] Decide public visibility for GraphSage/HashGNN result types and apply consistent changes. (API decision)
- [ ] Run `cargo test -p gds` and address failing tests (if any). (Validation)
- [ ] Generate a diff/patch bundle for review (I can create preview-only patches for each suggested fix). (You approve before apply)

---

## Useful commands I ran (repro) 🔁

- Rebuild: `cargo build -p gds`
- (recommended) Test: `cargo test -p gds`
- Lint/fix suggestions: `cargo fix --lib -p gds` (runs automatic rust fixes)

---

If you want, I can produce a preview patch that: 1) converts a handful of the most ambiguous glob re-exports to explicit `pub use` statements, and 2) optionally propose `pub` vs `pub(crate)` changes for the result types (with clear diffs to review). Say the word and I’ll prepare the preview bundle. 🚀

---

File authored by: GitHub Copilot
(Using Raptor mini (Preview))
