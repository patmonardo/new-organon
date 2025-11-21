# ADR 0010 — Concept subsystem: Determinate Concept & QualQuant Protocol

Status: Proposed
Date: 2025-08-19
Deciders: maintainers / architect

Context:
- Current system models runtime phenomena (Form layer: Entities, Contexts, Morphs, Aspects).
- We add a noumenal thread for Determinate Concept, Judgment, and basic Qual/Quant adjudication.
- It remains pure (no bus), consumed by Drivers/Engines via hints (conceptId, intent, confidence).

Decision:
- Introduce `logic/src/absolute/concept/absolute` as the AbsoluteConcept integrator and stubs (mechanism/chemism/teleology).
- Keep schemas canonical in `logic/src/schema` (Concept/Judgment); Form stays phenomenal (Aspects/Properties).
- Drivers attach noumenal hints on ActiveAspect.meta; Engines remain deterministic.

Consequences:
- Tests derive Judgments from Aspects/Properties; projections can materialize Higher Aspects.
- Non-contradiction enforced “in the same respect” via Aspect keys and projections (later).

Implementation status:
- absolute/concept: judgment/mechanism/chemism/teleology stubs present; integrator wired (pure).
- absolute/form: property-law projection (pure); test added.
- form/aspect: tests updated to import from aspect barrel; no symlinks.

Next steps:
- aspectKey/equalRespect helper and incompatibility table (for conflict projection).
- Seed concept registry + tiny adjudication (qual/quant) in AbsoluteConcept path.
- Draft Aspect-first driver surface (toActiveAspect/fromActiveAspect) and update tests to use it.
