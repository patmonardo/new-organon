# Canonical Prints Taxonomy — Plan (canonicalPrintsTaxonomy)

## TL;DR ✅
Add a canonical **prints** taxonomy to the repo. Start with a **TS-first** Zod schema + unit tests (fast, low friction) and then optionally add a **Rust Serde** canonicalization in `gds/reality` (kernel-first) if we decide the kernel should be authoritative.

---

## Goals 🎯
- Provide a single, well-documented envelope for events/prints emitted by kernel runs, FormEvaluator outputs, ML pipelines, and graph algorithms.
- Make it easy to validate, test, and evolve prints without schema drift across TS and Rust boundaries.
- Enable traceable provenance: `kind`, `provenance`, `payload`, `metadata`, `proof`.

---

## Proposed envelope shape (high-level)
```text
PrintEnvelope {
  id: string,
  kind: enum("taw","facttrace","ml","graph","proof"),
  timestamp: ISO8601,
  provenance: { source: string, runId?: string, kernelVersion?: string },
  payload: object (kind-specific),
  metadata?: object,
  proof?: object (opaque or structured)
}
```

---

## Implementation steps (recommended order) 🔧
1. **TS-first schema (quick win)**
   - Add `task/src/schema/prints.ts` exporting `PrintEnvelopeSchema`, `PrintKind` and supporting types using Zod.
   - Export from `task/src/schema/index.ts`.
   - Add example fixtures in `task/test/fixtures/prints/` (TAW, FactTrace, ML, Graph).

2. **Unit tests (validate existing outputs)**
   - Add `task/test/prints.test.ts` which validates sample outputs (from `model` test fixtures or small synthetic data) against the Zod schema.

3. **Adapter in `model/`**
   - Add a converter in `model/src/kernel-adapter.ts` or similar that maps raw kernel JSON to `PrintEnvelope` and ensure tests assert successful conversion.

4. **Optional: Rust canonicalization (if kernel-first)**
   - Add `reality/src/prints.rs` with Serde structs mirroring the TS envelope and a small integration test in `gds` that serializes JSON matching the TS schema.

5. **Docs & governance**
   - Add `gds/doc/PRINTS.md` describing the envelope, ownership, schema lifecycle, and examples.
   - Add a CI check/test to ensure Rust JSON and TS schema remain compatible (simple roundtrip or fixture validation).

---

## Design decisions & considerations 💡
- **Ownership**: Decide early whether authoritative prints are emitted by the kernel (Rust) or by TS adapters (Model/Task). This determines whether we start with Serde-first or Zod-first.
- **Scope**: Keep one common envelope with `kind`-specific `payload` objects (easier to evolve) vs multiple separate schemas (safer typed guarantees). Prefer the single-envelope approach with typed payloads to start.
- **Compatibility**: Add test fixtures and roundtrip tests to prevent schema drift.

---

## Risks & mitigations ⚠️
- Duplicate definitions across languages → mitigate with a canonical source + generation pipeline or CI validation.
- Breaking changes to emitted prints → mitigate via tests and versioned schemas (e.g., `schemaVersion` in envelope).

---

## Quick next actions (pick one) ▶️
- A) Draft the **Zod `PrintEnvelope` schema** + unit test (fast). (Estimated: 1–2 days)
- B) Draft **Serde structs** in `reality/src/prints.rs` + small kernel integration test (canonical). (Estimated: 2–4 days)

Which do you prefer? If you pick A, I’ll draft `task/src/schema/prints.ts` and `task/test/prints.test.ts` with fixtures.

---

## Example fixture (graph print, trimmed)
```json
{
  "id": "print-0001",
  "kind": "graph",
  "timestamp": "2025-12-18T00:00:00Z",
  "provenance": { "source": "gds::pagerank", "runId": "run-42" },
  "payload": { "algo": "pagerank", "nodes": 1234, "top": [{"id":"n1","score":0.9}] }
}
```

---

## Notes / Questions to decide before implementation ❓
1. Kernel-first vs TS-first: which should be authoritative?  
2. Should ML-specific metrics (e.g., loss, eval metrics) be included inline in `payload` or referenced separately as `artifacts`? 

---

*File created for iterative refinement. No file headers or frontmatter included (ready to be used as an untitled prompt).*
