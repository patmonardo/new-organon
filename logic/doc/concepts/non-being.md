# Dead Being → Principles (Adapter)

Intent
- Represent Being as simple, acromatic inputs and fix them as principles (Shape/Context/Morph) inside Essence for a processor run.

API
- assemblePrinciples({ shapes, contexts, morphs }) → { shapes[], contexts[], morphs[] }

Notes
- Inputs are minimal (type, name?); Context “axioms” remain opaque here since Context is a principle.
- Processor snapshots these principles (immutable during a run). All mutation/propagation lives in Entity/Property/Relation.
