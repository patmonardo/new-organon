# Whole and Parts — Truth of Absolute Ground (Essence)

Date: 2025-08-15

This note captures the engine-facing mapping of key Essence structures:
- Whole ⇄ Parts as the truth of Absolute Ground
- Force ⇄ Expression as the truth of Determinate Ground
- Essential Relation as the truth of Essential Being

It provides small invariants and processor hooks so the Form Processor can operationalize them.

## Essence mapping (very short)
- Reflection (1st moment of Essence): principles as Form — Shape, Context, Morph.
- Appearance (2nd moment): transactional expression — Conditions (Kriya) and their derived edges/properties.
- Ground (3rd moment): explanatory unity — Absolute relations (containers) that collect/justify conditions.
- Essential Relation (truth of Essential Being): the scientific Relation that carries constraints; our runtime shorthand for EssentialRelation is “Relation”.

## Whole ⇄ Parts as Absolute Ground
- Engine reading: an Absolute relation (Ground) is a Whole; the Essential relations that point to it via `particularityOf` are its Parts.
- Invariants:
  - Every Relation (EssentialRelation) marked actual must have `particularityOf` → a valid Absolute relation.
  - Every Absolute relation should have ≥ 1 Particular relation (part) referencing it; for synthesized Grounds, keep `contributingConditionIds`.
- Helpers in code:
  - `assertRelationHasAbsolute(relations)` — runtime check.
  - `findParticularsFor(absId)` and `findAbsoluteFor(relationId)` — navigation.
  - `groundToEssentialBridge(ground, candidates)` — small bridge that marks/creates the corresponding Relation when a Ground aggregates parts.

## Force ⇄ Expression as Determinate Ground (Kriya)
- Morph (Kriya) is the determinate Force; Expression are the derived relations/properties.
- groundStage applies Force (ruleSpec) to the Essence graph, producing Expressions deterministically and with provenance (context, modality-ready metadata).
- A later synthStage clusters Expressions to synthesize/confirm Grounds; modality ‘actual’ is assigned when evidence meets policy.

## Form, Matter, Content — how Whole–Parts is their truth
- Form (Shape/Context/Morph) provides admissibility and selection.
- Matter/Content arrives as Properties and Entity data.
- Whole–Parts is how the explanatory Form (Ground) captures and unifies the manifold Content as Parts (Relations) without erasing their particularity.
- The moment Outer (Expression) = Inner (Ground) we treat the Relation as Actual (modality attached in provenance).

## Minimal contracts (engine)
- Relation (EssentialRelation)
  - fields used: `id`, `sourceId`, `targetId`, `type`, `kind` ("relation"|"essential"), `particularityOf?`, `provenance?`
- Absolute relation (Ground container)
  - fields used: `id`, `sourceId`, `targetId`, `type: <X>:absolute`, `kind: 'absolute'`, `contributingConditionIds?[]`, `provenance?`
- Condition
  - fields used: `id`, `groundedBy?`, `provenance?`

## Processor hooks (present + near-term)
- present
  - groundStage: derive Relations + Absolute container per rule; set `particularityOf` links.
  - bridge: `groundToEssentialBridge` — recognizes a Whole and marks/creates its Relation (Actual).
- near-term
  - synthStage: cluster Expressions → synthesize Ground(s); compute modality; link condition.groundedBy; idempotent via signature hashing.

## Queries & checks
- `assertRelationHasAbsolute(relations)` — Relation (Essential) ⇒ Absolute container.
- `findParticularsFor(absId)`, `findAbsoluteFor(relId)` — Whole⇄Parts navigation.
- (planned) `findGroundFor(conditionId)`, `findConditionsFor(groundId)` — Expression⇄Force navigation.

## Policy knobs
- wholeThreshold (default 3): min parts to treat a Ground as Whole for the bridge.
- modality policy: clusterSize, provenance confidence, trusted sources.

## Next steps (optional)
- Implement `synthStage` with signature hashing + modality assignment.
- Add persistence tests for commit path (triad bus + repos) and invariants for Whole⇄Parts.
- Tighten types (introduce DTOs for processor working objects), replacing internal `any` casts.
