# ADR 0004 — AbsoluteConcept as Concept Generator / Attention Pipeline

Status: Proposed
Date: 2025-08-19

## Context

Work in `logic/src/absolute/concept` introduced pure, deterministic helpers that evaluate aspects and produce judgements, bhumi assessments, and realm classifications. The emerging shape of `AbsoluteConcept` resembles a generative concept pipeline: an a priori semantic synthesizer that weights evidence, composes judgements, and emits candidate intents and planning hints.

Philosophical resonance: this design parallels classical Yogic ideas (Samyama, Concept, Samadhi, Dhyana) and modern metaphors for attention and generative models. Fichte's notion of the "I" as productive activity and the Abhidharmakosa's treatment of essences both suggest a metaprocess that synthesizes elemental appearances into knowledge of Essence.

## Decision

We record the following design decision as a guiding principle for `AbsoluteConcept`:

- Treat `AbsoluteConcept` not only as an integrator/controller, but also as a pure, testable GENAI-like Concept Generator — a deterministic, configurable attention pipeline that produces Path Knowledge (Jnana) from phenomenal inputs.

- The pipeline's stages are intentionally explicit and small:
  1. Input normalization: accept `relations`, `properties`, `qualquant` and provenance.
  2. Projection & conflict detection: derive higher-order aspects and incompatibilities.
  3. Judgement synthesis: assemble S–P–O style Judgments from aspects and properties.
  4. Evidence weighting: compute `aspectTruth` via weighted aggregation of qualquant and derived signals.
  5. Attention & prioritization: attention-like weighting of evidence drives teleology and candidate intents.
  6. Path mapping: compute bhumi (11-stage Samyama seed) and realm assignments (Nature/Spirit/Logic) as separate, composable outputs.

- Keep all stages pure, deterministic, and side-effect free. External systems orchestrate persistence and action.

## Rationale

- Generativity: a Concept Generator captures the emergent behavior already present in the integrator — composition of judgments, ranking, and plan hint emission.
- Attention analogy: attention is a natural metaphor for how qualquant, truth, and provenance combine to surface certain aspects and suppress others; by making attention explicit we open the design to configurable weighting strategies without embedding black-box ML.
- Philosophical alignment: modeling Samyama as disciplined attention and Jnana as knowledge-of-dharma gives a clear narrative for why AbsoluteConcept outputs bhumi and realm mappings.

## Constraints and guardrails

- Purity and auditability remain required. Any attention or generator strategy must be pluggable, deterministic, and accompanied by provenance metadata.
- Keep yogic terminology documented and optional. The core API uses neutral terms (`aspectTruth`, `teleology`, `intentCandidates`, `bhumis`, `aspectRealms`) so downstream consumers can interpret them culturally.
- Do not conflate realm assignment with bhumi (Jnana): these are orthogonal outputs and require separate provenance.

## Consequences

- Short-term: improves conceptual clarity for developers and provides a roadmap for configurable attention heuristics.
- Mid-term: encourages adding provenance, attention-weight metadata, and a small demo pipeline demonstrating concept generation.
- Long-term: enables richer, explainable concept synthesis while keeping the core integrator purely functional.

## Next steps

- Update `README.md` in `logic/src/absolute/concept` with the attention/concept-generator mapping.
- Add provenance metadata and a `generatorMetadata` structure to `processAbsoluteConcept` outputs (optional, gated behind a minor API change).
- Prototype an attention-weighted truth aggregator as a pluggable strategy and add tests.

References
- Fichte: Foundations of the Wissenschaftslehre (productive I as activity)
- Abhidharmakosa: classic treatments of dharmas and hierarchical cognition
- Samyama / Samadhi / Dhyana / Jnana: Yogic practices related to attention and knowledge
