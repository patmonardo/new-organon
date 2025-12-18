# ADR 000 — Power / Empowering in Organon (Platform-level semantics)

Status: proposed  
Date: 2025-08-22  
Authors: Organon team

## Context

"Power" (empowerment, agency, authority) appears in multiple layers of Organon:
- metaphysical notes (Absolute/Relative, Rajas as "middle" empowering principle),
- runtime semantics (who/what may change state, trigger transforms, persist events),
- engineering concerns (permission, quota, event provenance, ordering, side-effects),
- algorithmic concerns (force/energy analogues: cost, weight, confidence in transforms).

We need a clear, minimal, implementable definition that maps the philosophical insight to concrete design decisions across schema, form, engine, processor, and driver layers so experiments and later speculative extensions (Kantian/Hegelian plugins) have a stable contract.

## Decision

Adopt a layered, explicit "Empowerment" model:

1. Canonical Schema: add a small, orthogonal schema `Empowerment` (Zod) in `logic/src/schema/` that represents an empowerment token:
   - id, subject (actor id), scope (resource ids / patterns), actions (allowed transforms), weight (numeric potency), confidence, provenance, ttl/validUntil, meta.
   - Minimal invariant: subject + at least one action + non-negative weight.

2. Form Wrappers: `FormEmpowerment` in `logic/src/form` wraps the schema and provides:
   - check(scope, action, context?) → { allowed: boolean, reason?: string, score: number }.
   - combine(other: FormEmpowerment[]) → combined potency (sum/weighted) using deterministic, testable rules.

3. Runtime Engine Hooks:
   - Engines (e.g., ContextEngine, MorphEngine, AspectEngine) accept an optional `empowerments: FormEmpowerment[]` argument on process/handle methods.
   - Engines must factor empowerment into decision points:
     - whether to accept a proposed change,
     - how to score competing proposals (truth/ground/confidence),
     - whether to emit side-effect events.

4. Drivers and EventBus:
   - Drivers must pass collected empowerment tokens from their environment (repo metadata, bus subscribers, API callers) into engine calls.
   - Empowerment tokens are first-class in events (events may include an `empowerment?: Empowerment` field).

5. Processor responsibility (the Rajas/middle):
   - The FormProcessor implements deterministic combination rules (the "empowerment algebra") and is the canonical place for cross-cutting mediation.
   - Keep the algebra simple and extensible: base operators (sum, max, precedence by provenance, decay by ttl).

6. Tests & invariants:
   - Add unit tests for empowerment algebra, engine behavior under multiple tokens, and event propagation.
   - Add integration scenarios where empowerment changes outcome (e.g., competing ActiveAspects/morphs).

## Rationale

- Explicit tokens make metaphysical language actionable and avoid implicit, scattered checks.
- Separation of concerns: schema = canonical token, form = ergonomics/checks, processor = mediation algebra, engine = local decision points, driver = wiring to environment.
- Deterministic algebra keeps system debuggable and testable while allowing later speculative extensions to implement different philosophies (e.g., Dharmic Kriya weighting).

## Consequences

Positive:
- Clear hook for permissions, provenance, and choice resolution across the six pillars.
- Enables experiments with "power" metaphors (Rajas/Sattva/Tamas) as alternate algebra implementations plugged into Processor.
- Improves auditability: empowerment tokens carry provenance and can be logged in events.

Tradeoffs:
- Slight API surface growth (engines accept extra param; events include optional token).
- Need to retrofit some callers/tests to thread empowerment through flows.
- Algebra choices may constrain future speculative metaphysics; provide plugin points.

## Alternatives considered

- Keep empowerment implicit (controller/driver-level checks only): rejected — leads to scattering and unpredictable resolution behavior.
- Bake empowerment into repository records only: rejected — insufficient for transient/ephemeral empowerment (short-lived tokens, runtime grants).

## Implementation notes

- Schema (Zod 4 compatible):
  - file: `logic/src/schema/empowerment.ts`
- Form wrapper:
  - file: `logic/src/relative/core/empowerment-formt.ts`
- Processor hook & default algebra:
  - file: `logic/src/relative/core/empowerment-processor.ts`
  - default combine: score = sum(weight * confidence); tie-breaker by provenance priority then timestamp.
- Engine integration:
  - Add optional `empowerments?: FormEmpowerment[]` parameter to `process()`/`handle()` signatures where applicable.
- Tests:
  - unit: empowerment algebra edge cases
  - integration: two competing ActiveAspect updates with differing empowerment tokens

## Terminology (Action / Concept)

To keep a minimal, consistent vocabulary across code and docs we adopt two Sanskrit-derived terms with precise technical meanings:
- Action — maps to Empowerment in the runtime: tokens that enable, authorize, or weight action. Action tokens are active, transient or persistent, and carry weight/confidence/provenance.
- Concept — maps to Knowledge/confidence: the epistemic score or certainty attached to data, schema, or empowerment. Concept influences scoring in the empowerment algebra (e.g., combined score uses weight * certainty).

Rationale: this preserves the useful metaphors (Kriya as doing/empowering, Jnana as knowing) while keeping them small, explicit, and implementable in the schema/form/processor layers.

## Kriya as Shakti — Veiling and Revealing

We treat Kriya as the operative Shakti: the active power that mediates between the Universal and the Singular. In this view:
- Kriya (Shakti) effects a projection: it conceals certain universal structure while revealing a singular manifestation suitable for action.
- The platform's absolute/form layer is the locus of this projection: Absolute Shakti as the Power of Projection that governs which aspects of the Universal are exposed to relative/Form processing.
- Important distinction: the projection reveals the Singular (the lawful, abstracted manifestation) rather than an arbitrary Particular. That keeps empowerment deterministic and conceptually coherent.

Implementation notes:
- Map projection intent to empowerment tokens (e.g., a `projection` flag or `mode` in the Empowerment schema) so engines/processors can apply veiling/revealing rules deterministically.
- The Processor (the "middle") is responsible for applying projection rules: given an AbsoluteConcept + Empowerment tokens, compute the Singular view passed to relative/form engines.
- Tests should include scenarios where different projection modes yield different Singular views from the same Universal input, demonstrating veiling/revealing behavior.

Rationale: making Shakti-style projection explicit keeps metaphoric language actionable and ensures that the system's empowerment model cleanly mediates between Absolute descriptions and Relative processing.

## Next steps

1. Add this ADR to repo (this file).
2. Scaffold `empowerment` schema + Form wrapper + default processor.
3. Run unit tests for algebra and update two engines (ContextEngine and MorphEngine) to accept empowerment tokens.
4. Update a small set of driver tests to include empowerment scenarios.
