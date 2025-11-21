# RuleSpec Primer — Morphs as Practical Principles

Status: draft  
Path: logic/src/absolute — purpose: primer for engineers and researchers

This primer expands the RuleSpec story: what a RuleSpec is, how a Morph uses it as a "practical principle", and two runnable examples you can use to exercise the ground stage directly.

High-level mapping (philosophical → code)
- Morph = practical principle (active Ground). A Morph is an engine object that carries a RuleSpec (declarative rule), priority/activation metadata, and runtime hooks.
- RuleSpec = declarative form of a controller: condition(s) → action(s). It is evaluated by the ground engine (applyMorphRule / groundStage) against the assembled World + reflect facets (spectrum/aspects).
- Shape = active Essence (used by reflect to compute facets/spectra).
- Context = active Reflection (runtime environment signals that modify rule application).
- Relation = Appearance / Spectrum-carrying artifact produced by groundStage.

Design goals for RuleSpec
- Declarative and inspectable (JSON/Zod-conformant).
- Deterministic for fixed inputs (no hidden randomness).
- Small, composable actions (createRelation, setProperty, tag).
- Safe: engine enforces invariants; RuleSpec actions should be idempotent.
- Advisory use of spectrum: reflect-derived spectra influence but do not deterministically force creation unless rule condition requires it.

Canonical RuleSpec shape (informal TypeScript)
```ts
// (for reference only; canonical schemas live in logic/src/schema)
type Ref = { type: "entity" | "property" | "shape"; id?: string; match?: Record<string, unknown> };

type Condition =
  | { op: "hasProperty"; target: Ref; key: string; value?: unknown }
  | { op: "spectrumGt"; target: Ref; intensity: number }
  | { op: "matchShape"; target: Ref; shape: string }
  | { op: "always" };

type Action =
  | { type: "createRelation"; relationType: string; source: Ref; target: Ref; meta?: Record<string, unknown> }
  | { type: "createProperty"; target: Ref; propertyKey: string; propertyValue: unknown; meta?: Record }
  | { type: "emitKnowledge"; event: string; payload?: Record<string, unknown> };

type RuleSpec = {
  id?: string;
  kind: "deriveRelation" | "deriveProperty" | "assert";
  when: Condition | Condition[]; // logical AND if array
  actions: Action[];
  priority?: number;
  note?: string;
};
