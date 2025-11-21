# Model and Plan — Projections and Syntheses

Definitions
- Model (Projection): A contextual, purpose-driven view over E/P/R. Read-only, parametric in Context; may filter, aggregate, or re-index Entities, Properties, and Relations.
- Controller (Judgment): Compares Models against axioms/targets; emits candidate actions with provenance.
- Plan (Synthesis): Orders and composes actions into a Workflow (tasks + edges). No side effects; executed externally.

Invariants
- Models never mutate E/P/R.
- Plans are shaped by Essential Relations (constraints, dependencies).
- Provenance: include { contextId, contextVersion } and source model/action ids.
