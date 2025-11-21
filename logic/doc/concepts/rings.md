# Rings 0–3 — Relational Form Processor

Intent
- Divide the semantic pipeline into three virtual rings over a minimal Core (Ring 0).
- Keep principles immutable per run; do all mutation/propagation in E/P/R.

Ring 0 — Core (foundation)
- Services: Id, Clock/Time, EventBus, Repository<T>, Logging/Trace.
- Contracts:
  - Repository<T>: get(id) → T|null; create(doc):T; update(id, mutateFn, { expectedRevision? }):T; delete(id):boolean.
  - Event bus: publish/subscribe; no long-lived side effects here.
- UserLand views:
  - Cypher is provided as a convenience snapshot/adapter for inspection (materialize/export only).
  - Ring 0 runtime uses fast primitives (scan/filter/traverse), not Cypher execution.
- Invariants: pure contracts, no domain logic; reused by all rings.

Ring 1 — Essence (runtime)
- Seed: Shape → Entity (universe of discourse).
- Contextualize: Context → Property (contextual predicates; provenance { contextId, contextVersion }).
- Ground: Morph → Relation (Essential/Absolute Relations; constraint propagation to fixpoint).
- Ports:
  - TriadProxy exposes entity/property/relation repos and event bus over Ring 0.
- Invariants:
  - Principles (Shape/Context/Morph) are snapshotted inputs (immutable this run).
  - All writes go via Repository.update(id, () => next[, { expectedRevision }]) or create().
  - Context version bump invalidates prior Property variations.

Ring 2 — Supersensible I (read + propose)
- Model: projections/read models over E/P/R (indexes, views).
- Controller: derive candidate actions/policies from gaps/contradictions (no side effects).
- Invariants: read-only over Ring 1 outputs; proposals only.

Ring 3 — Supersensible II (plan)
- Task: compile actions to tasks.
- Workflow: order/schedule tasks into a plan (still declarative; executed externally).
- Invariants: no side effects; emit a plan artifact.

Flow (one cycle)
- Snapshot principles → Seed (Entities) → Contextualize (Properties) → Ground (Relations; fixpoint) → Model → Control → Plan.

Notes
- Morph emerges from Shape ∧ Context (Being-for-self as Ground); its truth is Relation (EssentialRelation).
- Properties are contextualized predicates; Relations are constraint-bearing ties; Entities are the universe.
- Keep execution outside: Rings 2–3 produce plans; another runner executes.

Triad as UserLand proxy to Core
- The Triad ports (entity/property/relation repos + bus) form a small, stable UserLand API over a vast Core (Ring 0) fabric.
- Core is a Graph/Data/Compute fabric (ML/NLP pipelines, KV/feature stores, schedulers). UserLand sees a minimal contract; Core can scale independently.

Minimal TODOs
- R0: confirm Repository.get returns null; update uses mutateFn; handle undefined vs null consistently.
- R1: ensure Context version recorded in Property provenance; invalidate properly.
- R2: add 1–2 projections; derive simple actions from missing/contradictory relations.
- R3: compile actions into a linear workflow as a start.

Short next steps
- Add a minimal GraphFabric in-memory impl for tests if needed.
- Keep Cypher strictly in userland (materialize only).
- Ensure Ring 1 uses only Repository + Bus via TriadProxy.
- Rings 2–3 remain read/plan layers over Ring 1 outputs.

This keeps performance in Ring 0, a usable interface for Ring 1, and Cypher as a friendly view without coupling the runtime to it.
