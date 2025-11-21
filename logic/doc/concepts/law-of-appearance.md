---
title: The Law of Appearance — Concept (Jñānavijñāna)
---

Essentials
----------

- Appearance = reflected immediacy (essential concrete existence): flux that carries its own negativity and reflection.
- Two sides in appearance:
  - Positedness/external immediacy: accidental, transient side of the flux.
  - Self-identical positedness: permanent content within that flux.
- Law of Appearance = unity of those sides: identity across flux — the content that remains self-equal while determinations vary.
- Law is in appearance (not beyond it): the world is a kingdom of laws as its restful copy.

Jñānavijñāna mapping
--------------------

- Avyakta ↔ Essence/Ground (in-itself)
- Vyakta ↔ Appearance (manifest/reflected immediacy)
- Jñāna ↔ Lawful identity (substrate/content)
- Vijñāna ↔ Extraction/recognition of law within appearance (normalized invariants)

Engine surfaces (conceptual)
----------------------------

- AppearanceSnapshot (A-Surface): windowed view of EssenceGraph with reflective annotations and volatility metrics.
- LawSet (L-Surface): compact set of normalized rules/equations/correlations with signatures and evidence.
- WorldView: binder of A-Surface and L-Surface; asserts identity via content signatures.

Minimal contracts
-----------------

- appearanceSnapshot(graph, reflect?, window) → { snapshotId, signature, evidence }
- extractLaws(snapshot) → { laws[], signature, evidence }
- worldStage(snapshot, laws) → { worldId, contentSignature, evidence }

Notes
-----

- Keep computed-only and idempotent; no schema churn.
- Temporal windowing is essential; include time in signatures.
- Proof can be layered later; start with correlations/identities and provenance.
