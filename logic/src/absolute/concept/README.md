AbsoluteConcept — Samyama, Bhumis, and Realm Classification

Purpose

This folder contains pure, noumenal helpers used by the `AbsoluteConcept` integrator:

- `assessBhumiForAspects` — deterministic seed mapping of an `AspectTruth` score into an 11-stage Samyama bhumi assessment. This is intended as a conservative Jnana mapping and is configurable.
- `aspectRealms` classifier (in `absolute.ts`) — a lightweight heuristic to assign each Aspect to one of the three realms: `nature`, `spirit`, or `logic`.

Design Guarantees

- Purity: functions are pure and side-effect free. They don't persist or perform I/O.
- Determinism: mappings are deterministic for reproducible tests.
- Configurability: default names and heuristics are placeholders; consumers should provide `BhumiConfig` or override names where needed.
- Provenance: outputs should be extended with provenance metadata before being used by external systems.

When to use

`AbsoluteConcept` is intended for higher-order controllers, planners, and analysis that operate over aspects and derived judgements. Use these functions when you need a conservative, reproducible mapping from aspect evidence to bhumi stages or a realm assignment.

Future work

- Add provenance fields to all derived outputs (algorithm id/version, inputs, timestamp).
- Provide a canonical `bhumi.names.json` mapping and tests for cultural names.
- Replace heuristics with classifier hooks or pluggable strategies for richer domain mappings.

Concept Generator / Attention mapping

- The `AbsoluteConcept` integrator acts as a pure Concept Generator: it ingests aspects and qualquant, synthesizes judgments, weights evidence (attention), and emits plans and bhumi/realm mappings.
- Attention is modeled as prioritization derived from `qualquant` and `aspectTruth`. Configurable strategies may be plugged in later.
- See ADR 0004 in `logic/docs/adr` for the design rationale and philosophical references.

Practical notes

- Keep Samyama/Jnana terminology documented and optional — consumers may ignore or translate.
- Add provenance metadata before these outputs are consumed by persistence layers or planners.
