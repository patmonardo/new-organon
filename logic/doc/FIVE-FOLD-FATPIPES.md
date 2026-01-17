**Five‑Fold FatPipes — High Level**

This document sketches the Five‑Fold FatPipes (Fichte‑Hegel inspired) as the canonical channels that map SDSL declarations through `logic` schemas, DB models, procedures, and runtime traces. Each fold has an intent, a schema contract, a recommended DB pattern, a procedure contract (procedure‑first controller surface), and a minimal example of usage.

Fold 1 — Form ↔ Entity (Principle ↔ Existence)

- Intent: encode principled Forms (schema, invariants) and their instantiated Entities.
- Schema contract: `Form` (principle id, fields, invariants) and `Entity.shape.formId`.
- DB pattern: dedicated `forms` table + `entities` table with `formId` FK.
- Procedure contract: validate entity against Form, emit `formValidation` diagnostics, upsert Entity.
- Example: SDSL declares `Person` form → procedure creates `form:person:v1` and validates `entity:person:alice`.

Fold 2 — Particulars / Instances

- Intent: record empirical particulars and link them as sublated particulars to Entities.
- Schema contract: `Particular` (id, type, data) and `Entity.particulars: EntityRef[]`.
- DB pattern: join table (recommended) `entity_particulars` with optional `role` metadata.
- Procedure contract: upsert particulars, create join rows, update `Entity.facets.particulars` for read optimization.

Fold 3 — Judgment / Evaluation

- Intent: capture assessments, truth-judgments, and evaluation traces applied to Entities.
- Schema contract: `Judgment {subject: EntityRef, predicate, value, confidence, actor}` and `Entity.facets.judgments`.
- DB pattern: `judgments` table with subject FK and index on actor/date.
- Procedure contract: atomic append of judgments, re-evaluation hooks that can mutate entity facets.

Fold 4 — Signature / Power (Operational Interface)

- Intent: expose an Entity's operational capabilities (actions, transforms, signatures).
- Schema contract: `Entity.signature` as structured actions with input/output types and permissions.
- DB pattern: `signatures` table or JSON column on `entities` for compact cases.
- Procedure contract: register operational handlers, authorize calls, record invocation traces.

Fold 5 — Dialectic / History (Trace & Synthesis)

- Intent: persist the dialectical history — rounds of sublation, corrections, syntheses.
- Schema contract: `DialecticRecord {entity, round, changes, outcome, actor, timestamp}`.
- DB pattern: append-only `dialectic_records` table; consider event-sourcing for full replay.
- Procedure contract: every procedure emits dialectic records; synthesizer operators can compute higher‑order Forms.

Cross‑fold concerns

- Identity & stability: keep stable `id` namespaces and deterministic `formId` generation.
- Referential integrity: prefer FK constraints for core relations and join tables for flexible reuse.
- Read optimization: maintain `facets` or denormalized JSON summaries on `entities` for fast reads.
- Testing: provide round‑trip tests (SDSL → DB → read → validate against Zod schemas).

Next actions (proposal)

1. Prototype Fold 1 (Form↔Entity) in `logic/examples`: add small SDSL snippet, Prisma schema, and a procedure that validates and persists.
2. Add round‑trip test demonstrating validation + `Entity.particulars` linking.
3. Iterate folds 2–5 once Fold 1 is stable.

This is a living spec — each fold should be refined into concrete SDSL extensions and generator rules (codegen) once we agree on the procedure contracts.
