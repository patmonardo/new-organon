# Projection — Revealing / Concealing ↔ Creation / Preservation / Destruction

Date: 2025-08-20

Short summary  
Projection is the bridge language between Absolute Form (theory, essence) and Relative Form (runtime, appearance). Treat Projection as a two-sided operation:

- Absolute side (meaning/essence): Revealing — making aspects of an essence explicit; Concealing — abstracting or hiding details (preserving invariants).
- Relative side (Kriya / action): Creation — instantiate/emit new forms; Preservation — update/maintain existing forms; Destruction — revoke/delete forms.

Decision / mapping
- Map Revealing → Creation: when an Absolute projection reveals previously implicit structure, the Relative form should perform a Creation (shape/entity/aspect.create).
- Map Concealing → Preservation or Destruction:
  - Concealing-as-preservation → Preservation: hide internal details while keeping identity (shape.*.setCore, setState).
  - Concealing-as-removal → Destruction: conceal by revoking (shape.delete / *.delete).

Canonical payload placement
- Attach projection metadata to canonical shapes under payload.shape.projection:
  - payload.shape.projection = { mode: 'revealing' | 'concealing', intent?: 'creation' | 'preservation' | 'destruction' }
- Engines/drivers normalize legacy payloads into this path so services/tests can inspect projection intent reliably.

Implementation patterns
- Schema: extend shape schema with optional projection object (Zod) and provide create/update helpers that accept projection.
- Drivers: when converting inputs → Active*, set projection based on heuristics or driver config.
- Engines: when emitting events, include payload.shape.projection and ensure create emits include projection.mode === 'revealing'.
- Services/tests: read payload.shape.projection to decide UI/UX and orchestration behavior.

Example (driver normalization)
```ts
// filepath: /home/pat/VSCode/organon/logic/src/absolute/core/driver.ts
// ...existing code...
// when producing an event from engine, ensure projection exists
const normalized = this.normalizeEvent(evt);
normalized.payload.shape = normalized.payload.shape ?? {};
normalized.payload.shape.projection = normalized.payload.shape.projection ?? {
  mode: inferredMode, // 'revealing' | 'concealing'
  intent: inferredIntent, // 'creation' | 'preservation' | 'destruction'
};
return normalized;
```

Why this helps
- Makes the metaphysical intent explicit and machine-readable.
- Stabilizes tests and UI logic: projection metadata signals whether an event is a new reveal, an ongoing maintenance, or a removal.
- Keeps ADR 0001 naming/payload conventions intact (use noun.verb + payload.shape.*).

Next steps
- Add a small Zod projection schema to `schema/shape.ts`.
- Sweep drivers/engines to populate/normalize payload.shape.projection.
- Add a test asserting driver/engine emits include payload.shape.projection for create/update/delete flows.
