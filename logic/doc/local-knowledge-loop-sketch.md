# Local Knowledge Loop (Sketch)

Goal: a **fully local** “Knowledge Agent” loop that returns to Logic with *traceable* artifacts.

This is a sketch meant to be realistic with current repo surfaces:

- GDSL already defines boundary calls for:
  - `gds.graph_store.put` (stash a snapshot into kernel catalog)
  - `gds.form_eval.evaluate` (run Form ISA moments)
- Logic already has minimal helpers to:
  - seed a container program (`essence → shine → reflection`)
  - carry/merge a phenomenology facet (contradictions → foundation → judgment)
  - seed a syllogism input from morph patterns (Active Ground)
- Discursive “engines” are currently stubs (deterministic ports), which is fine.

## Loop overview

1) **Acquire** (TS / Logic): query Neo4j and normalize to a snapshot.
2) **Stash** (TS→Rust / kernel): `graph_store.put` registers a graph in the kernel catalog.
3) **Project** (kernel): run `form_eval.evaluate` on the stashed graph with `morph.patterns`.
4) **Return** (TS / Logic): interpret `proof` + context phenomenology and emit:
   - a `Concept` update (or seed)
   - a `JudgmentArtifact` (stub today; later GNN/LLM/hybrid)
   - a `SyllogismArtifact` (stub today; later Truth-of-Ground engine)
5) **Record** (TS / Task): wrap the run into an `AI WorkflowRun` trace.

The core architectural point: the Agent “returns to Logic” as **Concept first**, and only becomes executable as **Workflow** after Concept stabilizes.

## Step 1 — Acquire (Neo4j → snapshot)

Neo4j stays TS-side. Kernel never sees Cypher.

Minimal snapshot shape (current `gds.graph_store.put` contract):

```ts
export type GraphSnapshot = {
  nodes: number[]; // Neo4j internal ids or stable numeric ids
  relationships: Array<{ type: string; source: number; target: number }>;
};
```

Sketch:

```ts
async function snapshotFromNeo4j(/* cypher + params */): Promise<GraphSnapshot> {
  // 1) run cypher (Logic repository)
  // 2) extract node ids
  // 3) extract rel triples (type, sourceId, targetId)
  // 4) return { nodes, relationships }
  throw new Error('not implemented');
}
```

## Step 2 — Stash (snapshot → kernel catalog)

Via `KernelPort` using `GdsTsjsonKernelPort` (real NAPI) or a fake invoker for tests.

```ts
import type { KernelRunRequest } from '@organon/gdsl';

const req: KernelRunRequest = {
  model: { id: 'gds.graph_store.put' },
  input: {
    facade: 'graph_store',
    op: 'put',
    user: { username: 'root', isAdmin: true },
    databaseId: 'db1',
    graphName: 'neo4j:project:2025-12-21',
    snapshot,
  },
};

const res = await kernel.run(req);
if (!res.ok) throw new Error(res.error?.message);
```

This yields a kernel response (counts, etc.) and makes the graph available to later kernel calls.

## Step 3 — Project (Form Eval triad)

Current kernel supports `form_eval.evaluate` and the **canonical triad**:

- `essence` (proof marker: essentiality)
- `shine` (positedness marker; can carry anchors)
- `reflection` (citta/reflective marker)

Call:

```ts
const program = {
  morph: { patterns: ['essence', 'shine', 'reflection'] },
  // optional: carry anchors in program.shape.validation_rules
  shape: { validation_rules: { hegel: 'Essence→Shine' } },
};

const formRes = await kernel.run({
  model: { id: 'gds.form_eval.evaluate' },
  input: {
    facade: 'form_eval',
    op: 'evaluate',
    user: { username: 'root', isAdmin: true },
    databaseId: 'db1',
    graphName: 'neo4j:project:2025-12-21',
    program,
    artifacts: {},
  },
});
```

The returned `proof` is the kernel-side “witness” that Logic can re-enter.

## Step 4 — Discursive return (Judgment + Syllogism)

Today we can keep this deterministic (no LLM) using the stub ports in GDSL:

- `StubJudgmentKernelPort` produces a `JudgmentArtifact` primarily from `foundation.thesis`.
- `StubSyllogismKernelPort` produces a `SyllogismArtifact` from `morphPatterns` + optional judgment.

Sketch:

```ts
import { StubJudgmentKernelPort, StubSyllogismKernelPort } from '@organon/gdsl';
import { seedSyllogismInputFromProgram } from '@organon/logic/src/relative/form/dialectic/active-ground';

const judge = new StubJudgmentKernelPort();
const syllogize = new StubSyllogismKernelPort();

const judgmentResult = await judge.judge({
  moment: 'reflection',
  phenomenology: {
    foundation: {
      moment: 'infinite',
      thesis: 'Contradiction-free foundation is presented',
      resolves: ['c1'],
    },
  },
  proof: (formRes.ok ? (formRes.output as any)?.proof : undefined),
});

const syllogismInput = seedSyllogismInputFromProgram({
  program,
  judgment: judgmentResult.ok ? judgmentResult.artifact : undefined,
  proof: (formRes.ok ? (formRes.output as any)?.proof : undefined),
});

const syllogismResult = await syllogize.sylogize(syllogismInput);
```

This is the minimal “Knowledge Agent” signature: the return to Logic is **artifact-bearing**, not just “task completed.”

## Step 5 — Wrap as AI WorkflowRun

Even before a full TAW engine, we can treat the loop as a WorkflowRun record:

- inputs: query, snapshot stats, program
- outputs: kernel proof, judgment, syllogism
- trace: deterministic event list

## What’s missing for the *ML/GNN* upgrade

To replace the stubs with local GDS ML/GNN results:

1) Add a TS-JSON facade for running procedures (a minimal `run` or `algorithm` surface) against a stashed graph.
2) Choose one deterministic kernel result that can be interpreted as “grounds” for judgment:
   - similarity neighborhoods (KNN)
   - community partitions
   - contradiction-as-cut candidates
   - embedding clusters
3) Map that result into the phenomenology facet:
   - contradictions discovered
   - foundation candidates
   - judgment grounds

Then the Agent becomes a Knowledge Agent without calling a remote LLM.
