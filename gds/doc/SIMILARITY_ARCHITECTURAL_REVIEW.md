# Similarity Algorithms - Architectural Review

## Overview

This document reviews the similarity procedures under `gds/src/procedures/similarity` for compliance with the procedure-first controller pattern:

- **Storage Runtime (controller)**: orchestrates graph access and algorithm loop
- **Computation Runtime (state)**: pure state management, no graph access
- **Procedure Layer**: creates both runtimes and calls `storage.compute_{algo}(&mut computation, ...)`

## Current Status Summary

**Total Similarity Procedures**: 4
- ✅ **Architecturally Sound** (4): node similarity (NodeSimilarityBuilder), filtered node similarity, knn, filtered knn
- ❌ **Non-Compliant** (0)

**Reviewed**: 4/4 procedures (100% complete)
**Remaining Issues**: none in the read-only (stream/stats) paths

## 3C Review (Correctness, Completeness, Compatibility)

- **Correctness**: NodeSimilarity now applies `top_n` globally (after per-source `top_k`) and supports weighted similarity end-to-end via relationship-property selectors and `cursor.property()` reads. FilteredNodeSimilarity delegates to the same storage/computation pipeline with explicit source/target filtering. KNN/FilteredKNN paths are aligned with their specs; no new correctness gaps spotted here.
- **Completeness**: Mutate/write remain stubs across similarity procedures. There is no similarity-graph output mode (read-only stream/stats only) and no component/degree filtering knobs analogous to the Java reference.
- **Compatibility (Java GDS)**: Rust now matches key Node Similarity behaviors (metric variants + weighted vectors + topK/topN selection). Remaining parity gaps are primarily around additional filtering options (e.g. component/degree) and graph output modes.

## Node Similarity (`NodeSimilarityBuilder`) - ✅ Architecturally Sound

**Layering**
- Procedure: [gds/src/procedures/similarity/node_similarity.rs](gds/src/procedures/similarity/node_similarity.rs) builds `NodeSimilarityConfig`, instantiates `NodeSimilarityStorageRuntime` and `NodeSimilarityComputationRuntime`, and invokes `storage.compute(...)`.
- Storage Runtime: orchestrates projection via `get_graph_with_types_and_orientation`, owns neighbor access, drives computation.
- Computation Runtime: pure state; no graph access.

**Notes**
- Stream/stats only; mutate/write intentionally return execution errors.

## Filtered Node Similarity (`FilteredNodeSimilarityBuilder`) - ✅ Architecturally Sound

**Layering**
- Procedure: [gds/src/procedures/similarity/filtered_node_similarity.rs](gds/src/procedures/similarity/filtered_node_similarity.rs) validates label filters, projects the graph, and delegates to the filtered-node-similarity helper.
- Helper: [gds/src/algo/similarity/filtered_node_similarity/mod.rs](gds/src/algo/similarity/filtered_node_similarity/mod.rs) builds the source list + target mask, then invokes `NodeSimilarityStorageRuntime::compute_with_filters(...)`.
- Storage Runtime: graph-access controller (vector materialization, candidate enumeration, concurrency control).
- Computation Runtime: graph-free scoring using the configured metric (weighted or unweighted).

**Notes**
- Mutate/write remain stubs returning execution errors.

## kNN (`KnnBuilder`) - ✅ Architecturally Sound

**Layering**
- Procedure: [gds/src/procedures/similarity/knn.rs](gds/src/procedures/similarity/knn.rs) builds `KnnConfig`, creates `KnnStorageRuntime` and `KnnComputationRuntime`, and delegates to `storage.compute_single` or `compute_multi`.
- Storage Runtime: controller for neighbor/materialization and loop orchestration.
- Computation Runtime: pure state; no graph access.

**Notes**
- Multi-property mode ensures the primary property participates; tested.

**3C Review (KNN-specific)**
- **Correctness**: Rust kNN now follows an NN-Descent style loop aligned with Java GDS: initial neighbor sampling → split old/new → join neighbors (+ random joins) for a bounded number of iterations. Similarities are computed via `SimilarityComputer::safe_similarity`, filtered by `similarity_cutoff`, and each node retains a top-$k$ neighbor list.
- **Correctness (concurrency semantics)**: `KnnConfig.concurrency` is now enforced by the controller (`KnnStorageRuntime`) by installing a per-algorithm Rayon thread pool for the parallel section.
- **Completeness**: The config surface now includes the main NN-Descent knobs (`max_iterations`, `sampled_k`, `initial_sampler`, `random_seed`, `perturbation_rate`, `random_joins`, `update_threshold`). Procedure `stats()` now exposes key convergence metadata (`ranIterations`, `didConverge`, `nodePairsConsidered`). What’s still missing versus Java is parity-level task partitioning semantics (e.g. min batch sizing) and neighbor-consumer instrumentation.
- **Compatibility (Java GDS)**: Directionally aligned: Rust now implements the same stage structure and the same high-level controls. Remaining gaps are primarily around matching Java’s exact sampler behavior/parameters (especially `RANDOMWALK` details), and returning Java-style result metadata (`ranIterations`, `didConverge`, `nodePairsConsidered`, `nodesCompared`).

## Filtered kNN (`FilteredKnnBuilder`) - ✅ Architecturally Sound

**Layering**
- Procedure: [gds/src/procedures/similarity/filtered_knn.rs](gds/src/procedures/similarity/filtered_knn.rs) builds `FilteredKnnConfig`, creates `FilteredKnnStorageRuntime` and `FilteredKnnComputationRuntime`, and dispatches to single vs multi-property paths.
- Storage Runtime: controller for projections and loop orchestration.
- Computation Runtime: pure state; no graph access.

## Suggested Follow-ups

1) **Mutate/write design**: if needed, add mutate/write flows that produce similarity relationships or node properties via storage runtime outputs, following the controller pattern.
2) **Tests**: add focused coverage for end-to-end weighted behavior using real relationship properties (not just metric unit tests).
3) **Docs**: surface defaults (metric, k, cutoff, concurrency) and current read-only status in user-facing docs.
