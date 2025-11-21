# Cit · Citi · Citta — First Law of Science (operational mapping)

Short: encode the Yoga/Hegel mapping so it is actionable in ORGANON without changing schemas.

- Claim (philosophical): Sat · Sunya · Bhava → Cit · Citi · Citta
  - Sat (Being) / Sunya (Void) / Bhava (Becoming)
  - Cit (Absolute Consciousness) — the a priori, intellectual intuition (seed of form/essence)
  - Citi (Power of Reason) — apodictic inference, syllogistic return (judgment/syllogism)
  - Citta (Reflective Consciousness) — the lived reflective movement, vṛttis, appearances and their return as determinations

This is proposed as a “First Law of Science” in our metaphysical mapping: the engine must support the triadic movement from seed (Cit) to reason (Citi) to reflection (Citta).

Engine mappings (practical)

- Cit (seed / absolute)
  - Map to: Principle-level content and Shapes (Forms) / schema-level essence and content seeds.
  - Lives in: `schema/*` (essence/shape), creation helpers, principled morphs.
  - Role: provides the formal constraints that make judgments possible (the conceptual a priori).

- Citi (reason / apodictic inference)
  - Map to: judgment, syllogism, derive engines that close the return movement.
  - Lives in: `absolute/judgment/*`, `absolute/judgment/syllogism.ts`, actualization helpers.
  - Role: computes the Reason-view (Absolute Truth surface). Feeds modality decisions (via ADR 0008) when evidence supports necessary inference.

- Citta (reflective consciousness)
  - Map to: `reflectStage` (ADR 0011), the positing/external/determining triple, and vṛtti events.
  - Lives in: proposed `logic/src/absolute/reflect.ts` and `knowledge.*.reflected` / `knowledge.vritti` events.
  - Role: computes descriptive evidence, signatures, and transient facets that ground synthesis and truth‑actualization.

Secondary laws: Determinations of Reflection (Sattva : Tamas)

- Operationally treat Sattva/Tamas as qualitative gauges derived from dynamics:
  - Sattva-like signals: stability, repeatability, low‑variance across contexts, high coherence with essence/shape.
  - Tamasic signals: fragmentation, high context‑variance, brittle/contradictory evidence.
- Implementation: compute as a small vector [stability, activity, inertia] for Things/Properties and expose as reflective metadata (no schema change). Use these values for UX and to tune synth/actualize policies.

Practical recommendations

- Keep all Cit/Citi/Citta metadata and qualitative gauges as computed views and provenance events (transient), not required persisted fields.
- Use `reflectStage` to compute and emit Citta artifacts (vṛttis, signatures, facet evidence). These feed `groundStage`/`synthStage`/`actualizeStage` but do not mutate core Things unless `commit` is explicitly requested.
- Add light probes/tests showing the triad in action: seed shapes → derive syllogism → reflection alters determining confidence.

Connections

- ADR 0009 (Thing-as-Vastu) — Cit/Citi/Citta grounds the Thing design.
- ADR 0011 (Official Stages) — reflectStage is the Citta moment; synthStage/actualizeStage are Citi moves when they produce apodictic determinations.
- ADR 0010 (Property as Grounded Reflection) — properties are the site where Citta vṛttis are observed and evaluated by Citi.

If you want, I will now implement the `reflectStage` stub and add a tiny unit test that computes a vṛtti and a signature for a simple Thing; also can wire an opt‑in `knowledge.vritti` event from `groundStage`.
