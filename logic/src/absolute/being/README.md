# absolute/being — Absolute Being (Kevela Brahma) surface

Purpose
- A minimal, pure, deterministic noumenal generator to seed the AbsoluteConcept pipeline.
- Conceptual framing: Fichtean Absolute Being / Kevela Brahma — self‑enclosed seed that yields ideas, truth hints, and bhumi priors.

API (small, stable)
- createSeed(id: string, essenceType?: string, meta?: Record<string,unknown>) -> AbsoluteBeingSeed
- generateFromSeed(opts: {
    seed: AbsoluteBeingSeed;
    evidenceSubjective?: { id:string; note:string; weight?:number }[];
    evidenceObjective?: { id:string; aspectId?:string; score?:number }[];
    config?: { algorithm?:string; version?:string };
  }) -> AbsoluteBeingOutput

Design guarantees
- Pure functions: no side effects or IO.
- Deterministic: identical input → identical output.
- Provenance-rich outputs: algorithm, version, inputs, timestamp.
- Opt-in: AbsoluteConcept may consume this output but is not required to.

Integration notes
- Types referenced from `logic/src/absolute/concept/*` (Idea, AspectTruth, BhumiStage).
- Intended as a deterministic test stub and a clear place to evolve noumenal synthesis rules.
- Keep implementations conservative initially; extend provenance and rule details incrementally.

Testing
- Unit tests should assert deterministic outputs for fixed inputs and verify provenance fields are populated.
- Integration tests (optional) can feed AbsoluteBeingOutput into AbsoluteConcept.process to validate downstream composition.

Evolution
- Later enhancements: richer evidence fusion, domain priors (Samyama bhumi), ruleId provenance, and lightweight policy wiring (no persistence).
- ADR 0006 captures the contract and rationale.

Example (conceptual)
```ts
import { createSeed, generateFromSeed } from '@organon/logic/src/absolute/being';

const seed = createSeed('seed:alice', 'absolute.Being', { note: 'bootstrap' });
const out = generateFromSeed({ seed, evidenceSubjective: [{ id: 'm1', note: 'sensed', weight: 0.8 }] });
console.log(out.provenance); // { algorithm: 'absolute-being.v1', version:'v1', ... }
```

If you want, I can now:
- Add a deterministic unit test for generateFromSeed, or
- Wire the optional bootstrap into AbsoluteConcept.process
