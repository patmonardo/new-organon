# Miscellaneous Algorithms - Architectural Review

## Overview

This mirrors the Procedure-First Controller Pattern checklist used elsewhere. Each algorithm should present:

- **Procedure Layer**: facade validates inputs, creates runtimes, wires progress/termination, and invokes storage.
- **Storage Runtime**: controller owns graph access and progress, drives the algorithm, and calls computation with pre-materialized data.
- **Computation Runtime**: pure state, graph-free runtime producing results.

## Current Status Summary

- **Total Miscellaneous Algorithms**: 5
- ✅ **Fully Architecturally Sound**: 5 (index_inverse, scale_properties, collapse_path, to_undirected, indirect_exposure)
- ⚠️ **Partially Compliant**: 0
- ❌ **Non-Compliant**: 0

**Reviewed**: 5/5 algorithms

---

## scale_properties - ✅ Architecturally Sound

### Layered Architecture
- **Procedure Layer** ([gds/src/procedures/miscellaneous/scale_properties.rs](gds/src/procedures/miscellaneous/scale_properties.rs#L1-L180)): validates node_properties/scaler/concurrency, builds `ScalePropertiesConfig`, and delegates to storage+computation; exposes stream/stats; mutate/write unimplemented by design.
- **Storage Runtime** ([gds/src/algo/scale_properties/storage.rs](gds/src/algo/scale_properties/storage.rs#L1-L337)): controller builds per-property scalers (scalar/array), validates dimensions, and hands a plan to computation.
- **Computation Runtime** ([gds/src/algo/scale_properties/computation.rs](gds/src/algo/scale_properties/computation.rs#L1-L103)): executes the scaling loop and aggregates statistics from the prepared plan.

### Pattern Compliance
- Procedure now delegates to storage/computation with multi-property configs and scaler variants. Mutate/write remain intentionally unimplemented; progress/termination are not required by the current algorithm signature.

---

## index_inverse - ✅ Architecturally Sound

### Layered Architecture
- **Procedure Layer** ([gds/src/procedures/miscellaneous/index_inverse.rs](gds/src/procedures/miscellaneous/index_inverse.rs#L1-L60)): configures mutate graph name/concurrency and delegates to storage with computation runtime.
- **Storage Runtime** ([gds/src/algo/index_inverse/storage.rs](gds/src/algo/index_inverse/storage.rs#L1-L92)): projects per-relationship-type graphs, builds topologies with incoming adjacency via computation, and rebuilds the store with inverse indices (no GraphStore helper shortcuts).
- **Computation Runtime** ([gds/src/algo/index_inverse/computation.rs](gds/src/algo/index_inverse/computation.rs#L1-L54)): constructs outgoing/incoming adjacency for a relationship graph.

### Pattern Compliance
- Procedure invokes storage with computation runtime; storage owns graph mutation; computation builds adjacency from projected graphs. Relationship-type filtering is supported via config.

---

## collapse_path - ✅ Architecturally Sound

### Layered Architecture
- **Procedure Layer** ([gds/src/procedures/miscellaneous/collapse_path.rs](gds/src/procedures/miscellaneous/collapse_path.rs#L1-L100)): validates path templates, configures mutate graph/type/self-loop policy, and delegates to storage+computation.
- **Storage Runtime** ([gds/src/algo/walking/storage.rs](gds/src/algo/walking/storage.rs#L1-L220)): builds per-type graph layers, runs computation, and materializes a new graph store with the collapsed relationship type added to schema/topology.
- **Computation Runtime** ([gds/src/algo/walking/computation.rs](gds/src/algo/walking/computation.rs#L1-L80)): pure traversal that follows templates and emits unique collapsed edges.

### Pattern Compliance
- Procedure now calls the walking storage/computation runtimes; storage owns schema/topology mutation; computation is graph-free.
- Progress/termination wiring is still minimal (no tracker), matching current algorithm surface; extend if/when progress signals are added to the spec.

---

## to_undirected - ✅ Architecturally Sound

### Layered Architecture
- **Procedure Layer** ([gds/src/procedures/miscellaneous/to_undirected.rs](gds/src/procedures/miscellaneous/to_undirected.rs#L1-L133)): validates relationship type, configures mutate graph/type/concurrency, and delegates to storage + computation.
- **Storage Runtime** ([gds/src/algo/undirected/storage.rs](gds/src/algo/undirected/storage.rs#L1-L116)): builds a graph view for the selected relationship type, invokes computation, materializes a new store via `with_added_relationship_type`, and returns counts/edges.
- **Computation Runtime** ([gds/src/algo/undirected/computation.rs](gds/src/algo/undirected/computation.rs#L1-L47)): symmetrizes the relationship type by emitting unique directed edges (both directions) for the undirected view.

### Pattern Compliance
- Procedure now drives the algo runtimes; storage owns graph access + mutation; computation stays graph-free. Progress/termination hooks remain minimal pending future surface changes.

---

## indirect_exposure - ✅ Architecturally Sound

### Layered Architecture
- **Procedure Layer** ([gds/src/procedures/miscellaneous/indirect_exposure.rs](gds/src/procedures/miscellaneous/indirect_exposure.rs#L1-L86)): validates sanctioned property, weight property, iterations, concurrency; builds storage runtime and returns Pregel result (stats only, no mutation).
- **Storage Runtime** ([gds/src/algo/indirect_exposure/storage.rs](gds/src/algo/indirect_exposure/storage.rs#L1-L146)): projects the graph (optionally weighted), precomputes sanctioned mask and total transfers, wires Pregel, and materializes exposures/roots/parents/hops.
- **Computation Runtime** ([gds/src/algo/indirect_exposure/computation.rs](gds/src/algo/indirect_exposure/computation.rs#L1-L157)): Pregel kernels (init/compute/schema) with max reducer and sender tracking.

### Pattern Compliance
- Procedure drives storage/computation; storage owns graph access and Pregel wiring; computation is pure Pregel state. Progress/termination use Pregel runtime defaults; extend if surface grows.

---

## Scope Conclusion

- **Next Steps (priority)**: (a) extend progress/termination wiring across miscellaneous algos if/when surfaces require it; (b) decide on mutate/write support for scale_properties if needed.
- Re-run this review after surface changes.
