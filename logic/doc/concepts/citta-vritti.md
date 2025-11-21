# Citta / Citta‑vṛtti / Vṛtti — Mapping Yoga metaphors to the stages

This short note operationalizes the Yoga terms you described so they map cleanly to the stages and reflective contracts in ORGANON.

- Citta (Reflective Consciousness)
  - Engine analogue: the reflective faculty; the processor that computes the positing/external/determining triple.
  - Lives in: `reflectStage` (ADR 0011), `logic/src/absolute/reflect.ts` (proposed), docs ADR 0009.
  - Role: prepares evidence and signatures (vastuSignature/propertySignature); supplies the ``evidence`` that later stages reason over.

- Citta‑vṛtti (Citta‑vritti) — the movements of Citta; the specific reflective acts
  - Engine analogue: the transient computed facets and vritti events that represent particular appearances/measurements.
  - Lives in: `ReflectResult` returned by `reflectStage` and the event `knowledge.thing.reflected` / `knowledge.property.reflected` (proposed).
  - Role: Vṛttis are the processor's observations: the concrete appearances and their context-tagged evidence.

- Vṛtti (Appearance / the exitence of Citta)
  - Engine analogue: the immediate, contextual manifestation of a Thing/Property — the Relative Truth surface. Equivalent to Expressions/Conditions produced during `groundStage`.
  - Lives in: `groundStage` outputs (Expressions, Particulars) and in the appearance view returned by Kriya results.
  - Role: The thing you actually measure/observe; vṛttis feed synthesis (synthStage) and truth‑actualization.

Mapping summary (threefold):

- Positing (Citta as the power to posit) → intrinsic determinations (schema/core, sattva-like stable signs) → computed in `reflectStage` as positing-facets.
- External (Vṛtti / appearance) → relational/contingent determinations visible only in Context → appears as Expressions from `groundStage`.
- Determining (Citta's return/synthesis) → judgments, syllogisms, modality assignment (possible→actual) → produced by `synthStage` and `actualizeStage`.

Design notes and actionable suggestions

- Emit vṛtti events: when `groundStage` derives an Expression, publish a `knowledge.vritti` event with a pointer to the evidence and signature; useful for UX and tracing the flow from appearance→reason.
- Keep Citta metadata non‑intrusive: reflective facets and vritti events should be metadata and bus events, not required persisted schema fields.
- Use Context/World to represent the three times; vṛttis are context‑tagged so the same Thing can produce multiple vṛttis across horizons.

References

- ADR 0009 — Thing as Vastu
- ADR 0010 — Property as Grounded Reflection
- ADR 0011 — Official Stages & Pipeline

If you want, I can:
- add `knowledge.vritti` event emission to `groundStage` as an opt-in flag,
- or implement the `reflectStage` / `vritti` event stubs now and wire them into the orchestrator.
