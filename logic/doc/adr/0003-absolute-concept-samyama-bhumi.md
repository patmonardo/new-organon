# ADR 0003 — AbsoluteConcept: Samyama, Bhumis, and Three Realms

Status: Accepted
Date: 2025-08-19

## Context

The `AbsoluteConcept` integrator in `logic/src/absolute/concept` models a noumenal, pure controller for converting Phenomenal Aspects (ActiveRelations/ActiveProperties) into Judgments, plans, and evaluative metadata. Recent work introduced two complementary cultural/phenomenological constructs: a) a conservative Samyama bhumi mapper (11-stage seed) and b) an Aspect realm classifier (Nature / Spirit / Logic).

Yogic terminology (Samyama, bhumis, dharmas, jnana/marga) has no direct analog in typical scientific ontologies but is essential to the project's metaphysical modeling: AbsoluteConcept is intended as Path Knowledge — the generative womb of individual dharmas — and must remain pure, auditable, and configurable.

## Decision

1. Record the Samyama/bhumi design here as an explicit Architectural Decision Record (ADR). This keeps the metaphysical assumptions visible and separate from implementation details.

2. Keep two separate, minimal, and pure mechanisms inside the AbsoluteConcept area:
   - `assessBhumiForAspects(...)` — a deterministic, score→bhumi mapper that seeds an 11-stage bhumi assessment per aspect. This is a Jnana-oriented mapping (path knowledge) and intentionally conservative.
   - `aspectRealms` classifier — a lightweight heuristic that assigns each aspect to one of three realms: `nature`, `spirit`, or `logic`. Realm assignment is orthogonal to bhumi and is used by higher-level controllers to route/ascribe domain-specific handling.

3. Design constraints and guarantees:
   - Purity: these functions are pure, deterministic, and side-effect free. They accept inputs (relations/properties/qualquant) and return structured outputs (judgments, bhumis, aspectRealms, aspectTruth, teleology, etc.).
   - Traceability/Provenance: every derived assessment (bhumi, realm, truth, teleology goal) must be amenable to provenance annotation. The ADR recommends adding provenance metadata fields to outputs (algorithm id/version, inputs considered, timestamp, and ruleId) before these become relied upon by other systems.
   - Minimal cultural assumptions: default bhumi names and realm heuristics are placeholders. Names, thresholds, and heuristics must be configurable via a `BhumiConfig` and/or project-level mapping file.
   - Separation of concerns: the AbsoluteConcept is *not* the runtime engine — it's a pure integrator used by higher-order controllers. It must not perform persistence, network IO, or depend on global state.

4. The Three Realms mapping: the project standard will be to treat realms in a priority order for classification and reasoning:
   - Spirit — phenomenology associated with subtle/astral/devic capacities, indicated by qualquant tags like `spirit`, `astral`, `devic`, or dialectical markers (e.g., `meta.dialectic === 'absolute'`).
   - Nature — ordinary phenomenal relations, social/physical/biological aspects, indicated by `nature` tags, relation types such as `linked_to` or `particular`, or provenance indicating worldly origin.
   - Logic — Transcendental/Discriminative logic (Jnana/Path Knowledge). Logic is assigned last (a residual) and represents the discriminative lens that defines what Nature or Spirit are not.

## Consequences

- Advantages:
  - Keeps Yogic metaphysics explicit and discoverable to future contributors.
  - Enables conservative experimental adoption: bhumi mapping and realm classification can evolve independently without breaking APIs.
  - Pure, testable building blocks make it easier to review and audit metaphysical mappings.

- Risks:
  - Cultural terminology may be misunderstood by contributors; documentation and ADRs must remain authoritative.
  - Early heuristics may be too coarse; plan for iterative refinement and tests.

## Implementation notes / guidelines

- Place ADRs in `logic/docs/adr` (this file).
- Keep `assessBhumiForAspects` names configurable; add a mapping file `logic/src/absolute/concept/bhumi.names.json` or accept a `BhumiConfig` object.
- Add provenance fields to outputs (e.g., `BhumiStage.provenance = { algorithm: 'samyama-v1', inputs: [...], timestamp }`).
- Add unit tests for:
  - Deterministic mapping of truth score → bhumi level edge cases (0, small, 0.5, 1.0).
  - Realm classification rules and tag-driven overrides.
- Mark these modules experimental in the absolute barrel export and add migration notes if naming/semantics change.

## Next steps

- Create a short README at `logic/src/absolute/concept/README.md` that documents the API contract, purity guarantees, and how to configure bhumi/realm mappings.
- Add provenance metadata to `BhumiStage` and `aspectRealms` entries; update tests accordingly.
- Add unit tests that codify expectations and regression protection for these cultural mappings.

---

Notes: This ADR captures the project's current, conservative approach to modeling Samyama and bhumis inside `AbsoluteConcept`. It is intentionally implementation-light and prescriptive about purity, provenance, and configurability.
