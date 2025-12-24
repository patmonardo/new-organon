# Form Loci Map (Being → Essence → Concept)

Purpose: pin each locus to a role, boundary, and allowed dependencies before further implementation. Keep schema-first; avoid runtime promises.

## Loci and Roles
- Being: raw graph facts + procedural effects; no dialectic semantics. Source of existence/absence signals.
- Essence (FormProcessor boundary): shape + morph patterns + context envelopes; orchestrates calls, produces ResultStores; no mutation and no inference engine inside.
- Concept (ResultStore): stabilized unity that can hold morphPatterns plus optional judgment, objectivity, syllogism artifacts.
- Syllogism (artifact on Concept): discursive articulation with `mode` in meta.
  - Existence mode: premises/conclusion about instantiation/being in a graph; provenance to procedures/observations/attention scores.
  - Reflection mode: allness, induction, analogy; generalizes or transfers patterns; provenance to reflective procedures.
- Judgment (artifact on Concept): active/positive evidence; scalar node weights (per-node confidence) with basis/scale; provenance to procedures/attention. Keep embeddings/attention separate.
- Procedures (GDS procedural world): own positive/negative assertions (exists/missing), graph mutations, evidence generation; feed Concept/Syllogism/Judgment but remain distinct from FormProcessor boundary.
- ML/GNN (evidence providers): supply attention/weights/scores; never define dialectical categories—only populate provenance/evidence fields.

## Allowed Dependencies
- FormProcessor → may depend on schemas and orchestrate procedures; must not own inference or mutate graphs.
- Concept/Syllogism/Judgment artifacts → may reference procedures, graphs, evidence handles in meta; do not trigger execution.
- Procedures → may write graphs and produce evidence; may output artifacts to attach to Concept; must keep dialectical labels in artifacts, not in procedure plumbing.
- ML/GNN → may be invoked by procedures; outputs flow into artifacts’ meta/evidence; no direct dependency on dialectical types.

## Modes and Meta Contracts
- Syllogism meta: `mode` (existence|reflection|other), `evidence` (handles to traces/scores), `graph/context` identifiers, `procedure` provenance.
- Judgment meta: `basis` (probability|logit|energy), `scale`, `procedure`, `graphSnapshot`; payload: `nodeWeights` map (id → scalar).

## Guardrails (to avoid speculation creep)
- Keep FormProcessor schema-first; add runtime only when a procedure exists.
- No embedding of dialectical semantics inside ML/GNN; they only provide evidence.
- Any new locus must declare: role, inputs, outputs, provenance, and who is allowed to call it.

## Near-term TODO candidates (non-binding)
- Add `mode` and `evidence` fields to Syllogism artifacts (schema-only).
- Add `nodeWeights` (scalar map) + `basis/scale` meta to Judgment artifacts (schema-only).
- Add a FormProcessor call-level `mode` hint (reflection vs existence) to steer artifact emission, still boundary-only.
