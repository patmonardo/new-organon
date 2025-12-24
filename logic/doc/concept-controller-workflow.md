# Concept → Controller → Workflow (AI workflows)

## Why this exists

We need a stable architecture that makes the “Agent’s return to Logic” explicit and reviewable.

- The Agent lives *in Logic* as **Concept** first.
- It becomes executable as **Workflow** only after Concept stabilizes.
- **Controllers** are intermediates that translate Concept/Workflow intentions into concrete IO (kernel runs, DB queries, UI actions) and translate results back into discursive artifacts.

This is intentionally *not* “BI workflows.” These are **AI workflows**: self-revising, traceable, dialectically grounded procedures for inquiry and action.

## Core layering (contract, not implementation)

### 1) Logic Stream (Concept)

**Role:** phenomenological / discursive stabilization.

- Input: context, constraints, traces, artifacts.
- Output: a `Concept` that can be named, referenced, and re-entered.

Concept is where *meaning* coheres: Morph-patterns, Judgment cues, contradiction surfaces, Objectivity/Apearance mapping, etc.

### 2) Controllers (Intermediate)

**Role:** orchestration boundary.

Controllers do not “think.” They:

- validate and normalize inputs,
- call services (kernel compute, Neo4j, file system, model APIs),
- manage retries/timeouts,
- package outputs into stable artifact shapes.

Controllers are the *interface* between discursive Logic and non-discursive substrates.

### 3) Workflow (AI workflow)

**Role:** executable time-structure of inquiry.

- Input: a stabilized `Concept` (plus runtime constraints)
- Output: an executed `WorkflowRun` with trace/proof and produced artifacts.

AI workflow characteristics (vs BI workflows):

- **Dialectical:** includes contradiction surfacing/resolution, not just steps.
- **Reflexive:** includes judgment/syllogism artifacts (or placeholders) as first-class.
- **Traceable:** each run yields a proof/trace sufficient for review and re-entry.
- **Adaptive:** steps can be revised based on results, but revisions are recorded.

## Useful analogy: Compute from Storage

This architecture mirrors “invoke compute from storage”:

- Storage should not *compute*; it exposes read/write surfaces.
- Compute should not *store as a DB*; it exposes run surfaces.
- Controllers mediate between them.
- Logic/Agent is the discursive layer that decides *what* to do and *why*.

## Minimal interfaces (TypeScript-level sketch)

These are intentionally structural (protocol-first).

```ts
export type Concept = {
  id?: string;
  name: string;
  morph: { patterns: string[]; steps?: unknown[] };
  phenomenology?: unknown; // contradictions/foundation/judgment seeds
  objectivity?: unknown;
};

export type Workflow = {
  id?: string;
  concept: Concept;
  plan: unknown; // step graph; task/workflow IR
};

export type WorkflowRun = {
  workflow: Workflow;
  ok: boolean;
  trace: unknown;
  artifacts: Record<string, unknown>;
};

export interface Controller {
  runWorkflow(workflow: Workflow): Promise<WorkflowRun>;
}
```

## Implication for implementation work

- When we add kernel/protocol calls, we treat them as **controller-invoked** operations.
- When we add dialectical artifacts (Judgment/Syllogism/Objectivity), we treat them as **Logic-owned** results (even if stubs today).
- When we add planning/execution, we treat it as **Workflow-owned** (Task/TAW layer), driven by Concept.
