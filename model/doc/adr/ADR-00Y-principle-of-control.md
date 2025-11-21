# ADR-00Y: Principle of Control — Antakarana + Satkarya

## Status
Draft

## Context
Antakarana (inner instrument) and Satkarya (being-as-effect / truth-effect) together describe how pure (supersensible) principles connect to impure (empirical) manifestation. ORGANON requires a formal Principle of Control that mediates between @logic (pure) and @model/@agent (impure).

## Decision
Adopt Antakarana+Satkarya as the platform's Principle of Control:
- Antakarana: the inner controller / mediating faculty — maps to regulative provenance and supervisory policy.
- Satkarya: the truth-effect; the unity of control and manifestation — maps to the controller's declared intended outcome and its verifiable effect.

Controllers must therefore carry explicit metadata linking:
- Regulative provenance (which BEC principle / aishvarya profile they implement),
- Antakarana policies (supervisory hooks, adaptation rules),
- Satkarya assertions (declared truth-effect / outcome spec + verification hooks).

## Rationale
- Makes the connection between pure and impure explicit and auditable.
- Enables supervisory agents to reconcile regulative norms with operational behavior.
- Grounds control philosophically while remaining implementable.

## Consequences
- Controllers (code artifacts) must publish metadata and verification surfaces.
- EssenceGraph traversals can query antakarana and satkarya links for reasoning and audit.
- GenAI transformers can use these fields to synthesize controller behavior from BEC concepts.

## Next steps
1. Add controller metadata schema (see accompanying schema suggestion).
2. Update controller creation adapters to require regulativeRef, antakaranaPolicy, satkaryaSpec.
3. Prototype a verification pipeline that materializes satkarya truth-effects at runtime.
