# Essential Relation — Active Truth of Morph and of Entity:Property

Definition
- EssentialRelation is Relation as ActiveTruth: the realized output of Morph (Ground) and the binding that renders an Entity together-with its Properties a coherent Thing under a Context.

Two moments of Relation
- Truth of Morph (Ground): Morph rules specify admissible constructions; when actualized, they yield Essential relations. This is Relation-as-consequence of rule, carrying provenance of the derivation.
- Truth of Entity:Property: Relations articulate how properties stand in unity with the entity and with other entities. They are the truth-bearers that make the property-world communicable and testable.

Kinds and linking
- essential: the particular, constraint-carrying edge between sourceId → targetId with a type.
- absolute: the universal container that holds a cluster of particulars. Each essential may reference its container via `particularityOf`.
- alias: in the engine, "relation" is treated as an alias for "essential" in runtime checks for ergonomics.

Core properties (engine-oriented)
- Typed, directed references: `{ id, sourceId, targetId, type }`.
- Provenance: `{ ruleId?, source, timestamp, contextId?, contextVersion?, metaphysics?, modality? }`.
- Context governance: relations are valid only relative to a context; context version changes can invalidate or supersede them.
- Determinism: derived relations should be reproducible from the same inputs; identifiers should be stable given the rule and endpoints.

Invariants and safety rails
- Absolute container: every essential should point to an absolute container via `particularityOf`; the container mirrors the endpoints and expresses universality. The processor enforces this via an invariant check.
- Spectrum/Appearance: when available from Reflection, advisory spectrum metadata can be attached to both the essential and its absolute container for diagnostics and explanation.

Processor role (how it shows up in the engine)
- Ground (ActiveMorph) interprets Morph.ruleSpec over the working graph (entities, properties) and derives relations and properties to a fixpoint. It produces essentials and their absolute containers with provenance.
- Reflect (ActiveContext) can annotate relations with spectrum/appearance, aiding explanation without changing schema.
- World (ActiveWorld) aggregates and indexes relations to compute truths and enable propagation.

Compatibility with schema
- The schema-level Relation type remains the single source of structure; the engine adds advisory metadata and enforces invariants at runtime without changing the schema.
