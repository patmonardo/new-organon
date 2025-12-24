**Measure / Form Architecture (sketch)

This document summarizes the intended architecture and a pragmatic migration plan
for making `/form` the canonical Measure engine and `projection/eval/form` the
Measure evaluator that orchestrates it.

**Overview**
- **Measure (role):** The mathematical evaluation layer that computes measures,
  series expansions, and higher-order summaries (Taylor-series-style evaluators).
- **Form package:** Hosts the Measure engine implementation (mathematics, series
  evaluators, numeric primitives, symbolic helpers).
- **projection/eval/form:** The Measure evaluator (executor) that invokes `/form`
  to compute and materialize measures as ResultStores. Acts as the kernel Rust
  executor for Measure workflows.
- **projection/eval/ml:** The Quantity evaluator (heavy numeric/ML orchestration)
  that produces quantitative artifacts (models, tensors) consumed by Measure.
- **projection/eval/procedure:** The Quality evaluator (graph algorithms, heuristics)
  that computes structural features feeding ML and Measure.

**Principles**
- Keep mathematics (algorithms, series expansion logic) inside `/form`.
- Keep orchestration (projection requests, store wiring, context, provenance)
  inside `projection/eval/form` and `projection/eval/*`.
- Maintain clear boundaries: `projection/eval/*` call into `/form` and `/ml` but
  do not reimplement heavy math.
- Tests and examples drive the API: expose minimal, well-tested primitives.

**Short Migration Plan**
- Phase 1 — Clarify API and Surface (1-2 days):
  - Define a small Measure API in `/form` (evaluate_series, evaluate_point, summary).
  - Add integration tests in `projection/eval/form` that call the new `/form` API.
  - Files to update/test: [gds/src/projection/eval/form/form_spec.rs](gds/src/projection/eval/form/form_spec.rs), [gds/src/projection/eval/form/executor.rs](gds/src/projection/eval/form/executor.rs), [gds/src/projection/eval/form/pure_executor.rs](gds/src/projection/eval/form/pure_executor.rs).

- Phase 2 — Extnoract Math (2-5 days):
  - Move numeric and series logic from `projection/eval/form` (if any) into `/form`.
  - Keep thin adapters in `projection/eval/form`.
  - Add property-based tests for series convergence and correctness.

- Phase 3 — Harden & Optimize (ongoing):
  - Profile hot paths in `/form` and introduce optimized backends (SIMD, BLAS)
    if needed, but behind feature flags.
  - Maintain reproducible test seeds for ML/Measure integration tests.

**Immediate Dev Tasks (this sprint)**
- Create `form::measure::SeriesEvaluator` trait and a minimal `evaluate` impl.
- Wire an integration test so `projection/eval/form` calls `form::measure::evaluate`.
- Continue Clippy triage focused on `projection/eval` (replace `.default()` on
  unit structs — done; replace `unwrap_or_else(|| json!(null))` — done).

**Clippy and Linting Guidance**
- Run `cargo clippy -p gds` (no `-D warnings`) to gather current non-fatal lints.
- Apply automated fixes: `cargo clippy --fix --lib -p gds --allow-dirty`.
- Triage remaining high-value warnings in `projection/eval` (range loops,
  needless returns, derivable impls). Prioritize changes that improve clarity
  and maintain backward compatibility.

**Testing and Validation**
- Run the `gds` crate tests: `cargo test -p gds`.
- Add focused integration tests that validate `form` results against small
  numeric baselines.

**Documentation & Communication**
- Add this document as [gds/doc/MEASURE_ARCHITECTURE.md](gds/doc/MEASURE_ARCHITECTURE.md).
- Add a short README section in `projection/eval` linking to `/form` and this
  architecture doc.

**Risks & Notes**
- Large-scale refactors should be staged: add adapters and deprecation notes
  before moving logic.
- Keep performance tests separate — microbenchmarks for `/form` will inform
  optimization decisions.

**Next actionable command suggestions**
- Run Clippy (non-fatal):

```bash
cargo clippy -p gds
```

- Apply automated fixes and re-run Clippy/tests:

```bash
cargo clippy --fix --lib -p gds --allow-dirty
cargo test -p gds
cargo clippy -p gds
```


---

If you want I can now implement the Phase 1 skeleton in `/form` and add the
integration test, or proceed with automated Clippy fixes and targeted triage.
