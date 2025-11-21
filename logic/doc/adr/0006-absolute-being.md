# ADR 0006 — Absolute Being (Kevela Brahma) — role & contract

Status: Proposed  
Date: 2025-08-20  
Deciders: maintainers / architect

Context
- The repository distinguishes noumenal (absolute) and phenomenal (relative/form) layers.
- `AbsoluteConcept` is the generative Path Knowledge integrator (Ideas, Judgments, Teleology).
- We need a minimal, well‑specified noumenal surface that can deterministically seed concept generation without relying on the Form/Repository layers.

Decision
- Introduce an Absolute Being surface (module: `logic/src/absolute/being`) with a small, pure API:
  - createSeed(id, essenceType?, meta?) -> AbsoluteBeingSeed
  - generateFromSeed({ seed, evidenceSubjective?, evidenceObjective?, config? }) -> AbsoluteBeingOutput
- Absolute Being is explicitly Fichtean / Kevela Brahma in design intent: a self‑enclosed, a‑priori generator of noumenal seeds consumed by AbsoluteConcept.
- Absolute Being must be pure (no side effects/persistence), deterministic, and emit structured provenance (algorithm, version, inputs, timestamp).

Rationale
- Keeps noumenal generation separate from Form/Repository responsibilities.
- Provides a testable bootstrap for concept/idea generation useful for deterministic unit tests and integration stubs.
- Preserves philosophical intent (seeded, pure, generative source) while remaining practical for engineering use.

Consequences
- The module is conservative by default (placeholder synthesis) and is opt‑in for AbsoluteConcept (no forced runtime changes).
- Outputs include Ideas, Aspect truth hints, optional bhumi priors, and provenance metadata.
- Evolution path: richer synthesis rules, provenance enrichment (ruleIds), and optional domain priors (Samyama bhumi mapping).

Implementation notes / contract
- Determinism: same inputs → same outputs.
- Purity: no DB/network calls; callers persist if needed.
- Provenance: include algorithm id/version, input references, and timestamp.
- API types are lightweight and schema‑referenced (Idea, AspectTruth, BhumiStage from `absolute/concept`).
- AbsoluteConcept.process accepts an optional AbsoluteBeingOutput to bootstrap Idea/truth hints.

Next steps
- Add README + examples in `logic/src/absolute/being/README.md`. (done)
- Add unit tests for deterministic behavior and provenance.
- Wire as an optional bootstrap in AbsoluteConcept.process behind a config flag.
