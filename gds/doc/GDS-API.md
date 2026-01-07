# GDS API Overview

This note summarizes the current Rust-side API surfaces for GDS. It is intentionally brief and reflects what exists today.

## Layers
- **Applications + Machinery**: request orchestration; lives under `gds/src/applications/algorithms/**`. Uses `AlgorithmProcessingTemplate` and `ProgressTrackerCreator` to wrap compute with tracker lifecycle, memory guards, and rendering.
- **Procedures (facades)**: canonical algorithm entrypoints for callers; live under `gds/src/procedures/**`. Provide fluent builders (e.g., `Graph::astar()`, `Graph::pagerank()`) that validate inputs, build storage/computation runtimes, and run the algorithm.
- **Algorithms (internal)**: `gds/src/algo/**` contains storage/computation runtimes. Applications should not call these directly; procedures are the public surface.

## Primary caller surface
- `Graph` facade in `gds/src/procedures/graph.rs` is the user-facing handle. It exposes methods that return fluent builders for pathfinding, centrality, community, embeddings, and utilities.
- Builders typically support modes (stream/stats/mutate/write) via methods like `.stream()`, `.stats()`, and `.mutate(...)`.

## Progress + memory contract (short)
- Applications own the outer tracker via `ProgressTrackerCreator` (machinery). Procedures use injected trackers for domain progress and may create per-run trackers for multi-target loops.
- Memory estimates are provided by procedures via `estimate_memory()` and enforced by machinery guards when invoked through templates.

## Usage shape (conceptual)
1) Application handler parses request → chooses mode → calls the appropriate procedure builder (via `Graph`).
2) Machinery builds the envelope (task tree, tracker, guard) and invokes a compute closure.
3) Procedure validates config, constructs storage+computation runtimes, runs the algorithm, reports progress, and returns results for rendering.

## Source references
- Machinery templates: `gds/src/applications/algorithms/machinery/algorithm_processing_template.rs`
- Progress tracker creator: `gds/src/applications/algorithms/machinery/progress_tracker_creator.rs`
- Procedure graph facade: `gds/src/procedures/graph.rs`
- Example pathfinding builder: `gds/src/procedures/pathfinding/astar.rs`
- Control-logic review (applications): `gds/doc/ALGORITHMS-MACHINERY-CONTROL-LOGIC-REVIEW.md`
- Progress contract (pathfinding): `gds/doc/PATHFINDING-PROGRESS-CONTRACT.md`
