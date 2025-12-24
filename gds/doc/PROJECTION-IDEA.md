# Projection: The Concealing and Revealing Power

> "Projection is not just data loading. It is the architectural act of mediating between raw Reality and determinate Form."

This document serves as the unified architectural definition for the `gds/src/projection` system. It consolidates the conceptual framing ("Conceal/Reveal"), the runtime architecture ("The Three ISAs"), and the engineering boundaries ("Factory vs. Eval").

## 1. The Core Concept: Two-Sided Projection

Projection in this system is strictly defined as a **two-stage** operation:

1.  **Factory (Concealing Power)**: The movement from **Source → Image**.

    - _Action_: It **conceals** the contingent details of the source (file formats, physical storage, raw IDs) to produce a stable, abstract **Image** (`GraphStore`).
    - _Why_: To operate on data, we must first abstract away its accidental properties (e.g., whether it came from Arrow, CSV, or a Database).

2.  **Eval (Revealing Power)**: The movement from **Image → Result**.
    - _Action_: It **reveals** determinate structures, patterns, or computations inherent in the Image.
    - _Why_: Evaluation is not "magic"; it is an explication of what is implicitly contained in the Image (Projection).

### The Invariants

- **Image Invariance**: The `GraphStore` is the stable boundary. Eval never touches Source; Factory never computes analytics.
- **Source Opacity**: Eval must remain agnostic to where data came from.
- **Determinacy**: All revealing must be an explainable transformation of the Image.

---

## 2. Stage 1: Factory (Conceal)

**Location**: `gds/src/projection/factory/`

The Factory subsystem owns the **ingestion** of reality into the system. Its goal is to produce a `GraphStore`.

### Responsibilities

- **Native Ingestion**: Reading Arrow tables, Parquet files, or other sources.
- **Topology Construction**: Building node/edge lists from raw tables.
- **ID Mapping**: Translating external "Source IDs" to internal efficient integers.
- **Property Materialization**: selecting a `Collections` backend (Vec, Huge, Arrow) to store property data.

### Current Architecture & Boundary

- **Public Contract**: `GraphStoreFactory` / `GraphStoreFactoryTyped`.
- **Arrow Subsystem**: `gds/src/projection/factory/arrow/**` contains the pipeline (scanners, importers).
  - _Note_: Currently couples to `Collections` by choosing concrete backends (e.g., `VecLong`). The long-term goal is to make backend selection swappable via a `CollectionsFactory` injection.

---

## 3. Stage 2: Eval (Reveal) - The Three ISAs

**Location**: `gds/src/projection/eval/`

Once an Image exists, we "reveal" truths about it using three distinct Instruction Set Architectures (ISAs).

### I. Procedure ISA (Computation / Assertion)

- **Focus**: Discrete algorithms on the graph topology.
- **Mode**: Immediate / Universal.
- **Mechanism**: `ProcedureExecutor` running `AlgorithmSpec`.
- **Examples**: PageRank, WCC, Shortest Path.
- **Output**: `f64` streams, simple stats.

### II. ML ISA (Pipeline / Problematic)

- **Focus**: Statistical learning and continuous feature extraction.
- **Mode**: Mediate / Particular.
- **Mechanism**: `PipelineExecutor` running `Pipeline` traits.
- **Examples**: Node Classification, Link Prediction, GNN training.
- **Output**: Tensor streams, model weights.

### III. Form ISA (The Union / Apodictic)

- **Focus**: The transcendental union of Procedure and ML.
- **Mode**: Singular / Result-producing.
- **Mechanism**: `FormProcessor` / `TriadicCycle`.
- **Function**: Form projects logic _through_ the other two ISAs. It treats Procedure (Thesis) and ML (Antithesis) as moments in a larger **Triadic Cycle** to produce a unified Synthesis.
- **Output**: A derived `GraphStore` (the "Result").

---

## 4. The RealityFabric

Underpinning both Factory and Eval is the "Fat Pipe" to reality, known as the **ResultFabric**. It connects the abstract projection machinery to physical constraints.

**The Five-Fold Fabric:**

1.  **Storage**: Persistence surfaces (Backends).
2.  **Compute**: CPU/GPU allocation surfaces.
3.  **Control**: Identity, tenancy, policy.
4.  **Time**: Budgets, leases, deadlines.
5.  **Witness**: Trace, audit, and proof sinks.

_Code Anchors_: `FormStoreSurface`, `ComputeSurface`, `StorageMeta`.

---

## 5. Engineering Status & Future Direction

### Integration Points (Factory ↔ Collections)

Currently, `Projection/Factory` manually constructs specific `Collections` (like `Vec`-backed properties). The refactoring roadmap involves:

- Allowing the Factory to accept a generic `CollectionsFactory`.
- Moving "Storage Backend" choices out of ingestion logic and into configuration/injection.

### Agent-Driven Future

- Future **Agents** will drive the Factory to "scan reality" and produce Images on demand.
- Future **CoreGraph** will sit above `GraphStoreFactory`, orchestrating multi-source ingestion policies.

### Summary

The GDS Projection system is a **machine for turning Reality into Truth**:

1.  **Factory** conceals Reality to make it manageable (Image).
2.  **Eval** reveals Truth from that Image through Computation, Learning, and Form.
