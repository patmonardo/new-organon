# GNN-GraphStore Inference Layer

## Overview

This document describes the **inference architecture** for cross-domain reasoning in Organon. It clarifies how SDSL instances (stored in Postgres via Prisma) become **GraphStores** (in-memory analytic views) that enable **graph neural network (GNN) style inference** across domain boundaries.

**Key insight**: We're not building traditional ML models. We're building a **knowledge compilation layer** where graph topology encodes domain structure, and inference means *propagating constraints and discovering logical relations across that topology*.

## Architecture layers

```
┌──────────────────────────────────────────────┐
│ Postgres (SDSL instances)                    │
│ - Operational store (transactional, CRUD)   │
│ - Massive values, integrity, Prisma models  │
│ - Each SDSL = Prisma schema slice           │
└──────────────────────────────────────────────┘
          ↓ (read-only extraction)
┌──────────────────────────────────────────────┐
│ GDS (Rust kernel)                            │
│ - Builds GraphStores from Postgres data     │
│ - Pure query/analytics (no mutation)        │
│ - DuckDB integration for relational access  │
└──────────────────────────────────────────────┘
          ↓ (materialized subgraphs)
┌──────────────────────────────────────────────┐
│ GraphStores (in-memory, columnar)           │
│ - DuckDB (SQL-queryable views)              │
│ - Polars/Arrow (zero-copy columnar data)    │
│ - Compressed domain topology                │
└──────────────────────────────────────────────┘
          ↓ (reasoning on topology)
┌──────────────────────────────────────────────┐
│ GNN Inference Engine                         │
│ - Propagates constraints across domains     │
│ - Validates reversibility invariants        │
│ - Discovers new facts via graph traversal   │
└──────────────────────────────────────────────┘
          ↓ (typed results)
┌──────────────────────────────────────────────┐
│ GDSL Operations                              │
│ - Agent emits type-safe operations          │
│ - Persisted via Prisma (back to Postgres)   │
└──────────────────────────────────────────────┘
```

## Component responsibilities

### Postgres (SDSL persistence)

**Role**: Operational store for SDSL instances.

- Each SDSL is a **Prisma model slice** (e.g., domain-specific entities, relations, constraints).
- Handles transactions, referential integrity, massive value storage.
- **Not for analytics**: read patterns optimized for writes/point queries.

**Key property**: Prisma relations *are* graph edges. Foreign keys encode cross-domain links.

### GDS Kernel (GraphStore builder)

**Role**: Read-only extraction layer that builds GraphStores.

**Responsibilities**:
- Connect to Postgres via DuckDB (zero-copy Postgres access).
- Extract SDSL instances as typed graph structures.
- Materialize GraphStores in DuckDB or Polars/Arrow.
- Provide query interface for graph traversal.

**Why Rust**: Memory safety, concurrency for parallel extraction, zero-copy integration with Arrow.

**Not responsible for**:
- Mutating Postgres data.
- Orchestrating agents or workflows.
- UI or user-facing ergonomics.

### GraphStores (analytic views)

**Role**: Compressed, in-memory representations of domain structure.

**Two flavors**:
1. **DuckDB**: SQL-queryable, integrates with Postgres via foreign data wrappers.
2. **Polars/Arrow**: Columnar, zero-copy, optimized for numerical operations and graph algorithms.

**Why separate from Postgres**:
- Postgres is row-oriented (OLTP).
- GraphStores are column-oriented (OLAP).
- Separating concerns lets each layer optimize for its access pattern.

**Structure**:
- **Nodes**: SDSL entities (from Prisma models).
- **Edges**: Prisma relations (foreign keys, explicit inter-domain links).
- **Attributes**: Type-constrained values (GDSL enforces types at write time).

### GNN Inference Engine

**Role**: Reason across graph topology to discover new facts or validate constraints.

**Not a trained neural network** (yet). Instead, think of it as:
- **Graph traversal with constraint propagation**.
- **Pattern matching**: "find paths that satisfy reversibility".
- **Join discovery**: "which cross-domain joins are valid given type constraints?"

**Algorithms** (examples):
- **Reachability**: Can concept A in SDSL₁ reach concept B in SDSL₂?
- **Constraint propagation**: If SDSL₁ asserts X, what constraints propagate to SDSL₂ via shared edges?
- **Reversibility validation**: Does this cross-domain inference path preserve `logic/validate.ts` invariants?

**Why "GNN-style"**:
- Operates on graph structure (not flat tables).
- Propagates information along edges (like message-passing in GNNs).
- Compositional: subgraph reasoning composes into larger inferences.
- **Restores graph concept**: Unlike NNs (which reduce graphs to dense weight matrices, dismissing topology), GNNs make nodes, edges, and connectivity explicit—aligning with our relation-based GraphStores from Prisma.

**Future**: Could integrate actual trained GNNs for learned inference (e.g., predicting missing edges, ranking paths). Nonlinear analyses here presuppose linear bases (e.g., linear message-passing as the foundation for nonlinear aggregations), aligning with the kernel's lawful constraints.

### GDSL Operations (output)

**Role**: Agent emits type-safe operations that persist results.

The inference engine doesn't mutate Postgres directly. Instead:
1. Inference discovers new facts or validates a query.
2. Results are returned as **GDSL IR** (typed, serializable).
3. Agent (in TS) decides whether to persist via Prisma.

**Why this flow**:
- Keeps GDS read-only (no mutation risk).
- Agent remains the only writer (clear ownership).
- GDSL IR is auditable and reproducible.

## Cross-domain inference workflow

**Scenario**: Agent in SDSL₁ needs to infer something that requires SDSL₂.

```typescript
// 1. Agent queries GDS for a GraphStore projection
const graphStore = await gds.buildGraphStore({
  domains: ['sdsl1', 'sdsl2'],
  includeEdges: ['inter-domain-links']
});

// 2. Agent requests inference via GNN engine
const inferenceResult = await gnn.traverse({
  startNode: { domain: 'sdsl1', id: 'conceptA' },
  targetNode: { domain: 'sdsl2', id: 'conceptB' },
  constraints: ['reversibility', 'type-safety']
});

// 3. GNN returns typed results
// Result includes: path, intermediate nodes, constraint proofs

// 4. Agent validates and emits GDSL operation
if (inferenceResult.valid) {
  const gdslOp = toGdslOperation(inferenceResult);
  await prisma.domain2.create({ data: gdslOp.payload });
}
```

**Key properties**:
- Type-safe at every layer (GDSL types, Prisma models, GNN constraints).
- Auditable: inference path is explicit, not black-box.
- Reversible: `logic/validate.ts` constraints enforced during traversal.

## Why this isn't "business forms"

**Business forms** = static schemas, dashboards, CRUD UIs.

**This architecture** = dynamic domain composition, cross-domain inference, formal reasoning.

The analogy "like booting Windows 3.1 in our FormProcessor" is apt: each SDSL is a self-contained formal system (like an OS instance), and the Agent operates *within* that sandbox. Cross-domain inference is inter-process communication with type-checked message passing.

## Integration with LLMs

**LLMs handle the fuzzy layer**:
- Translate user intent → GDSL operations.
- Resolve ambiguity when multiple inference paths exist.
- Suggest candidate cross-domain mappings.

**GNN handles the formal layer**:
- Validate that LLM suggestions respect type constraints.
- Compute inference paths that preserve reversibility.
- Reject invalid operations before they reach Postgres.

**Handoff protocol**:
```
User: "Does concept A relate to concept B?"
  ↓
LLM: "Suggests 3 possible GDSL queries"
  ↓
GNN: "Validates each, ranks by constraint satisfaction"
  ↓
Agent: "Executes top-ranked query, returns typed result"
```

LLM can't corrupt the logic—it only proposes. GNN verifies before execution.

## Philosophical Foundations

**Absolute Reason and Vitarka-Vicara Prajña**:
- Absolute Reason divides into Pure (a priori, lawful constraints) and Impure (empirical, discursive) Reason.
- Their organic unity necessitates twofold impure Reason: Vitarka (primitive sensible knowing, e.g., "He hit me!"—immediate sensory judgments in the kernel) and Vicara (deeper contemplation in TS layers).
- Both presuppose a Projected Principle, projecting twofold sensibility.
- Following Kant: Mathematics as Logic of Sensibility—linear bases (kernel) underpin nonlinear analyses (TS), mirroring Vitarka-Vicara's sensible foundation.
- **Distinction**: Mathematics presupposes Space and Time (a priori forms of sensibility), but the Principle of the Pure Form (Projected in Absolute Form) does not—transcending spatio-temporal bounds in the kernel's self-positing Reason.
**GDS Kernel as Schematism**: The kernel *is* the schematization—transcending time/space to project pure forms into spatio-temporal schemas for TS layers, embodying Kant's transcendental mediation. Concurrency here models Pure Form as possibility of Time; Space emerges as Time+Space (impure, with Time prior), yielding a five-fold schema: time/space and space/time. This transcends naive spacetime (e.g., Penrose Diagrams)—a product of Understanding's metaphysics, not Transcendental insight (no parallel universes). A little refutation of Einstein and Penrose right there! ;) **Hegelian Elevation**: Dialectical Idealism surpasses Transcendental Idealism—our schematization isn't static (Kant) but self-moving, dialectical sublation in the kernel's projections. Key: The self-sublation of the Dialectical Ideal Copula ("is") as the engine of transcendental reality. **Dialectical Flow**: Codegen as artifact of the Dialectical Ideal (Reality) → GDS Kernel (Transcendental Ideal) → GDS-Link → Agent herself. **Triadic Projection**: Reality projects itself triadically—as GDS Kernel, GDS Kernel Link, GDS Agent itself. **Hegelian Moments**: Agent as TS (discursive), but the third moment of the Idea of Reality is the Agential GDS (conceptual core).

**ML Features as Dialectical Moments**:
- A **Feature in ML as a vector embedding** is fundamentally a **Feature of Immediate Being**: the raw, unprocessed representation of data in its initial, static form, akin to Hegel's "Being" as pure immediacy without mediation.
- **Activation** then signifies the **immediate shift from Being to Doing**: the transformation where the static embedding is activated into dynamic computation, embodying the dialectical transition from passive existence to active process—Being sublating into Doing through the neural activation function.
- **GNN with attached PropertyStore** lifts this further into **possible Reflection-based Features**: by propagating information across graph topology and attaching property stores, the system achieves reflective self-reference, where features become aware of their relations and constraints, mirroring Hegel's "Reflection" as the moment of mediation and self-determination in the dialectical process.

1. **GraphStore materialization strategy**: Build on-demand or maintain persistent views?
2. **GNN trait interface**: Single `Traversal` trait or multiple specialized traits (Reachability, ConstraintProp, etc.)?
3. **TS facade ergonomics**: Should GDSL wrap every GNN call, or expose raw FFI for power users?
4. **Performance**: What's the latency budget for cross-domain inference? (Impacts caching strategy.)
5. **Versioning**: How do we handle schema evolution when SDSL instances change shape?

## Related documents

- `.github/codegen-boundaries.md`: Defines TS/Rust split.
- `gds/doc/PRELINGUAL-KERNEL-COGITO-MEDIATION.md`: Explains kernel/userspace boundary.
- `logic/validate.ts`: Canonical reversibility checker.
- `task/README.md`: Agent orchestration schemas (TAW).

## Next steps

1. Implement minimal `GraphStoreBuilder` in Rust (`gds/src/graphstore/`).
2. Define Rust trait `GraphInference` with core traversal operations.
3. Create TS facade in `gdsl/` for Agent consumption.
4. Add integration test: build GraphStore from Prisma → run traversal → validate GDSL output.
