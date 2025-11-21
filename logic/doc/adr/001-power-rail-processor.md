# ADR 001 — Power Rail: Processor as Synthesis, Morph, and Empowerment

Status: Proposed
Date: 2025-08-22

Context

The project models ideas as Shapes (schemas/Essence) and their applicability as Context (provenance, modality). Early work established Empowerment tokens and a Form → Processor pattern. We now need a canonical articulation of how these pieces compose into the runtime pipeline that "powers" the system—its power rail.

Decision

Treat the Processor as a deterministic, auditable "power rail" that synthesizes Shapes and Context into actionable Morphs applied to Ground (the repository/world).

Key concepts

- Shape: the Essence of a thing (Zod schema, Form wrapper). Defines invariants and expected structure.
- Context: provenance/modality/conditions for acting; a Knowledge Feature used for reasoning and trust.
- Morph: the instantiated transformation produced by Shape + Context — a typed, guarded operation that maps inputs to outputs and is safe to apply to Ground.
- Ground: the repository/world of Entity, Property, Relation instances. The Morph is the constructor/synthesizer that manipulates Ground.
- Empowerment: the enacted Kriya/Karma; acts that materialize Change on Ground. Empowerments are expressed as Events/Changes and recorded with provenance.
- Relation (Spectral): relations are constructed displays of Essence — spectral because they are shaped by both the Morph (intent) and Context (modality) and are therefore rendered rather than intrinsic.

Consequences

- The Processor will be the canonical place for deterministic selection, filtering, tie-breaking, and auditing of actions.
- Morphs must expose guards (authority, modality, scope) and deterministic transforms (Events/Changes). Engines call MorphProcessor to get persistent events.
- Provenance and modality are first-class and used for scoring, TTL, filtering and privilege logic; speculative transformations are gated behind explicit flags and always record critique metadata.

Implementation notes / next steps

1. Formalize `ProvenanceSchema` and wire `FormEmpowerment.getProvenance()` and `effectiveConfidence()`.
2. Define `FormMorph` API: id, inputShape, outputShape, guard(context): boolean, transform(payload, context): Event[].
3. Implement `MorphProcessor` that selects morphs deterministically, applies them to Ground via Repository, and emits audit traces.
4. Add tests for deterministic tie-breakers, authority-driven boosts, TTL filtering, and critique recording.

Speculative mapping: Container (Morph) ↔ Contained (Aspect)

The Processor's "power rail" also models a dyadic synthesis between a Container (Morph) and a Contained manifestation (Aspect). This ADR records that mapping in order to preserve the theoretical foundation and to keep engineering decisions aligned with the project's metaphysics.

- Morph (Container synthesis): produced by combining a Shape (schema/Essence) with Context (provenance/modality). A Morph is a guarded, typed transformation — an intent or plan describing how to construct or change Ground.
- Aspect (Contained manifestation): the Entity/Property/Relation state produced when a Morph is applied to Ground. Aspect is the rendered display of Essence under Context.

Pipeline (Container -> Contained -> Reflection)

1. Morph synthesis: select or construct Morph(s) from incoming Shape + Context.
2. Reflection/validation (dry run): verify guards, authority, modality, TTL and other policies deterministically.
3. Apply Morph to Ground: deterministic transform(s) emitted as Events persisted by the Repository.
4. Observe Aspect(s): retrieve created/updated Entity/Property/Relation artifacts with attached provenance.
5. Validate Aspect vs Morph: structural and semantic checks, provenance alignment, score thresholds.
6. Concept check: if Aspect meets Morph's expectations across structure, score, and provenance, construct a `Concept` (successful realization); otherwise emit a deterministic Critique object describing failures.

Concept — realized composite

Concept designates the deterministic predicate where a manifested Aspect fulfills the Morph's intent and constraints and is combined with Ground- and Relation-level observations to form a recorded `Concept` artifact. A Concept predicate is computed from:

- structuralMatch: the Aspect conforms to the Morph's expected aspect spec
- scoreOk: the Aspect's effective score meets or exceeds threshold
- provenanceOk: the Aspect's provenance satisfies authority/modality policies

When all predicates hold the system constructs and persists a `Concept` artifact (see Ground+Relation = Concept below); otherwise the system records critique metadata and may schedule corrective morphs.

Guardrails to avoid speculation churn

- No code generation or speculative transformations by default. Any model-driven or heuristic transform requires an explicit opt-in flag `enableSpeculation` and must append provenance.trace entries including model id and version.
- Deterministic tie-breaking: use an explicit comparator ordering (authority rank, provenance.confidence, recency, stable id order). Test the comparator exhaustively.
- Every transformation and critique is auditable: persist provenance, trace, critique reasons, and the exact deterministic inputs that produced the output.

No codegen yet

This ADR is intentionally descriptive and normative: do not introduce automated code generation or large speculative generator changes at this stage. We have a foundation to build on; respect it. Implementation work should be incremental, test-driven, and reversible.

Addendum: the Path from Ignorance to Knowledge

This ADR documents the Path from Ignorance to Knowledge in engineering terms: Shape (Essence) combined with Context (knowledge feature/provenance) produces Morph (plan), applied to Ground producing Aspect (manifestation), and validated into a `Concept` (knowledge realization) or critique (further work). Recording this as a first-class pipeline helps the project reason about cognition, action, and auditability in a controlled manner.

Ground + Relation = Concept

In our triaxial model a `Concept` is the composite of Ground-level synthesis (the Morph applied to Shape/Context producing an Aspect) and Relation-level observation (the Relation's rendering/observation of that Aspect). The `Concept` artifact pairs the synthesized Aspect with the Relation observation and records provenance, rationale, and Samadhi-like validation results.

Phenomenological mapping: Dharana / Dhyana

To preserve the project's philosophical framing, note a compact mapping between contemplative terms and system roles:

- Dharana (concentration) corresponds to the Ground-side discipline: the focused synthesis, construction, and constraint enforcement that produces an Aspect from a Morph.
- Dhyana (meditative absorption or contemplation) corresponds to the Relation-side observation and interpretive rendering that makes the Aspect legible and meaningful.

Ground and Relation operate asymmetrically but together; in the conceptual synthesis they are "sublated" into a `Concept` — the artifact "in itself" that records both the doing (Ground/Dharana) and the seeing/meaning (Relation/Dhyana). This keeps the metaphysical subtleties present in the documentation while preserving clear engineering contracts.

Terminology: Processor vs Engine

Historically the project used the term `core` to mean the processing layer— a deterministic Processor that performs selection, transformation, and auditing. To avoid confusion we standardize terminology:

- Processor: deterministic, audit-focused modules that implement the canonical pipelines (e.g., `empowerment-processor`, `morph-processor`). These live under `relative/core` and should mirror naming in `absolute/core`.
- Engine: higher-level orchestration components that compose processors, services, and drivers. Engines may coordinate I/O, long-running state, or integrate external systems.

Recommendation: keep `relative/core` and `absolute/core` aligned as Processor folders. Use `-processor` suffix for files that implement the canonical, deterministic logic. Reserve `engine` for orchestration and runtime composition.

Philosophical grounding: Fichte's "drive" and Dialectical Idealism

To tie the implementation model back to the project's philosophical grounding, note the resonance with Johann Gottlieb Fichte's emphasis on the "drive" (Trieb) — an intrinsic impetus that moves the subject into activity and self-positing. In our architecture this is intentionally reflected in two complementary ways:

- Drivers (practical adapters): psychologized as the external/operational expression of Drive — the bridge that brings Processor intent into real-world activity. Drivers translate, mediate, and motivate processor outputs to act on Ground.
- The Processor/Engine dialectic: Processors (deterministic synthesis) and Engines (orchestration) participate in a dialectical loop where Shape+Context produces Morphs (intent), Drivers carry that intent into the world, and resulting Aspects feed back into reflection and further morphogenesis.

Framing the system with Fichtean terminology clarifies intent: Drive is not a mystical authority but the dynamic tendency to act that must be made explicit, constrained, and audited. Naming the adapter layer `driver` is therefore psychologically apt: it captures the role of motivating, translating, and connecting abstract plans (Morphs) with embodied acts (Aspects) while preserving deterministic, auditable governance.

Rationale

This ADR keeps the architecture auditable, incremental, and deterministic while preserving expressive power: shapes remain pure, context informs authority, and morphs operationalize change.

Decision made by: project maintainers

Concept → Idea: Plan as Teleological Syllogism

The `Concept` produced by the Processor is not merely a record; it is the Realized Plan that becomes the operative Idea or Knowledge Agent in the system. In teleological terms the Concept functions as the syllogistic unity of Genus/Species (the classificatory intent embedded in Shape and Morph) and Subject/Predicate (the instantiated Aspect and its observed properties). This unity is the Plan — a determinate objectivity that extends subjectivity within Ground and enables subsequent reasoning and agency.

Practically: when a Processor constructs and persists a `Concept`, that artifact should include: a clear intent (plan), the morph/aspect provenance chain (why it was realized), a truth-of-empowerment indicator (the deterministic predicate that validated the realization), and an optional `agentRef` when the Concept is adopted by a Knowledge Agent for further action. Recording these fields preserves the teleological status of the Concept and enables downstream agents to act upon Ideas with auditable rationale.
