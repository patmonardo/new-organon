---
title: Reciprocal Action (Property-as-Connector)
---

Summary
-------

This short concept note maps the Hegelian passage on "reciprocal action of things" into the processor's vocabulary.

Key mappings
------------

- "Thing-in-itself exists in concreto by essence" → Entities are given determinateness through computed `essence` and `properties`.
- "Property is this reciprocal connecting reference" → Properties are the connectors that mediate influence and causation between entities.
- "Reciprocal determination, the middle term" → Actions are the concrete expression of middle-terms (mediating relations) computed by the Action stage.

Practical takeaways
-------------------

- Treat properties as causal connectors in engine logic (computed views), not necessarily as persisted causal edges.
- Surface reciprocal effects as `Action` objects carrying provenance and signatures.
- Keep the reflective/grounded pipeline intact; insert Action stage after grounding so actions operate on derived relations and reflective facets.

Examples (informal)
-------------------

- If `Entity A` has property `temperature > 100` and `Entity B` has `flammability = high`, an Action rule can produce an `ignite` action from A→B with confidence computed from property strengths and reflective determining scores.

Relation to existing modules
----------------------------

- `reflect` provides facets and signatures useful for scoring action confidence and provenance.
- `ground` produces relations and particulars that the Action stage consumes to build directed actions.
- `knowledge` can record `vritti` events for actions when emission is enabled.
