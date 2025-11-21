---
title: Law of Appearance and World — Concept Note
---

Summary
-------

- Appearance = essential concrete existence (reflected immediacy), unity of reflective shine and concrete existence.
- Law of appearance = the stable, self-identical content within flux; a compact identity binding diverse determinations.
- World = unity of appearance and law: the appearing world with its restful copy (kingdom of laws). Teleology (kingdom of ends) is downstream.

Mappings to engine
------------------

- Appearance → AppearanceSnapshot over EssenceGraph with reflective annotations; non-persistent by default.
- Law → L-Surface: normalized rules (equations, correlations, predicates) extracted from appearance, materialized as model projections.
- World → WorldView: a binder tying an Appearance snapshot to its Law set with signatures and evidence.
- Ends → Plans/goals derived from Laws; consumed by Control/Plan.

Minimal contracts
-----------------

- AppearanceSnapshot(graph, reflect?, window) → { snapshotId, signature, evidence }
- ExtractLaws(snapshot) → { laws[], signature, evidence }
- WorldStage(snapshot, laws) → { worldId, contentSignature, evidence }

Design notes
------------

- Keep everything computed and idempotent; attach provenance and signatures.
- Windowed appearance enables temporal analysis; signatures must include time bounds.
- Keep Law extraction pluggable: simple correlations now; proofs later.

Examples (informal)
-------------------

- Temperature/pressure series across a window yields Boyle-like correlations as candidate laws; Appearance carries the raw flux; Law holds normalized expressions; World binds both.

Next
----

- Draft ADR for Force and World of Forces; propose a minimal force mediation helper and tests.
