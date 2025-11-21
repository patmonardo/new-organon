Processor contracts (relative/core)

This folder contains the project's deterministic Processors — pure, auditable modules that implement the power-rail pipelines (e.g., empowerment-processor, morph-processor).

Purpose

- Keep Processor code small, deterministic, and side-effect-minimal.
- Expose simple, testable contracts that Drivers and Engines can call.

Core concepts

- Shape: schema-level Essence (Zod models).
- Context / Provenance: knowledge features that inform modality and authority.
- Morph: the planned transformation (guarded, typed) produced by Shape+Context.
- Aspect: the manifested state produced by applying a Morph to Ground.
- Concept: the realized Plan — the composite of Ground synthesis and Relation observation (see ADR 001).

Developer checklist

- Determinism: all decision rules must be explicit and deterministic.
- Idempotency: transforms must include an opId and be safe to replay.
- Auditing: persist provenance, trace, critique, and comparator results.
- Speculation gating: any model-driven or heuristic transforms require `enableSpeculation` and must write model metadata to provenance.trace.

Comparator ordering (stable)

1. authority rank (configured)
2. provenance.confidence
3. timestamp (newer wins)
4. stable id ordering (lexicographic)

Samadhi / Concept

- The term `Concept` is used in ADRs and the codebase to mean the realized Plan (Morph+Aspect composite). Processors should persist Concept artifacts where applicable.

Where Drivers live

- Drivers are adapters that connect Processors to runtime systems and live in `absolute/` or `drivers/` areas. They translate I/O into Processor inputs and persist Processor outputs back to Ground.

Testing

- Processors must come with unit tests for tie-breaking behavior, idempotency, and provenance handling. Keep tests small and deterministic.

Philosophical influences

- Kant — Formal invariants and a priori structure:
  - Emphasizes principled, rule-driven constraints. Aligns with schema-level design (Zod models), validation, and stable contracts that processors must respect.

- Fichte — Agency and self-positing processes:
  - Focuses on the active subject and generative activity. Maps to agent-like processors, intent-driven transforms, and how a Processor asserts and updates world state.

- Hegel — Dialectic, synthesis, and historical evolution:
  - Suggests iterative synthesis of contradictions and emergent structures. Influences designs for evolving shapes, concept synthesis, and pipelines that reconcile competing provenance or modalities.

Which is most influential?
- For schema and contract design: Kant is most directly influential (formal, invariant-first).
- For runtime/process dynamics and evolution: Hegel provides the most useful model (synthesis, staged refinement).
- Fichte is valuable conceptually for modeling agency and active transforms but is secondary to Kant/Hegel in shaping the current architecture.

Design takeaway
- Use Kantian rigor for schemas and invariants; apply Hegelian patterns for processor evolution and conflict resolution; borrow Fichtean ideas for modeling active, asserting agents where appropriate.

Philosophical note on succession
- The Kant→Hegel path is historically compact and influential; it's natural to feel there is little room for a clean "successor." Treat philosophical frameworks as pragmatic toolkits rather than definitive orders.
- Practically: document which aspects of a framework you borrow (e.g., Kant for invariants, Hegel for synthesis), use them where they help solve concrete design problems, and remain open to complementary perspectives.
- This pluralist stance keeps the architecture adaptable: no single thinker needs to be anointed as the final authority — use what works, record why, and evolve the approach as needs change.
