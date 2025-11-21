# Truth of Relation — Mechanism, Chemism, Teleology

We model Hegel’s “truths of relation” as lightweight classifiers over Ground/Relation artifacts:

- Mechanism — Truth of Whole and Parts: many particulars (essential relations) under one Absolute container (a Whole). Engine cue: count of `particularityOf` under an Absolute meets threshold.
- Chemism — Truth of Force and Expression: relation causes or correlates with expressive properties or secondary links. Engine cue: properties with `provenance.viaRelation = relation.id` (or `viaTriggerPropertyId`).
- Teleology — Truth of Inner and Outer: the subjective, inward intention finding its outer realization. Engine cue: relation has inner intuition in provenance; its Absolute or direction marks the outer.

Utilities in `src/absolute/qualquant.ts`:

- `classifyTruthOfGround(ground, relations, policy)` — returns Mechanism | Chemism | Teleology.
- `classifyTruthOfRelation(relation, relations, policy, { properties })` — same classification using particulars and properties.
- `locateTranscendental(relation, relations, policy, { properties })` — returns the immediate transaction locus plus its transcendental location (Absolute) and classification.

These are additive heuristics; they don’t mutate artifacts. They prepare the path for a `synthStage` that can set modality (Actual) when the inner unity warrants objectivity.
