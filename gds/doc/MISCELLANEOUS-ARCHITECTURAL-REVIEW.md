# Miscellaneous Algorithms - Architectural Review

## Overview

This mirrors the Procedure-First Controller Pattern checklist used elsewhere. Each algorithm should present:

- **Procedure Layer**: facade validates inputs, creates runtimes, wires progress/termination, and invokes storage.
- **Storage Runtime**: controller owns graph access and progress, drives the algorithm, and calls computation with pre-materialized data.
- **Computation Runtime**: pure state, graph-free runtime producing results.

## Current Status Summary

- **Total Miscellaneous Algorithms**: 5
- ✅ **Fully Architecturally Sound**: 1 (index_inverse)
- ⚠️ **Partially Compliant**: 3 (scale_properties, collapse_path, to_undirected)
- ❌ **Non-Compliant**: 1 (indirect_exposure – procedure missing)

**Reviewed**: 5/5 algorithms

---

## scale_properties - ⚠️ Partially Compliant

### Layered Architecture
- **Procedure Layer** ([gds/src/procedures/miscellaneous/scale_properties.rs](gds/src/procedures/miscellaneous/scale_properties.rs#L1-L186)): validates source property and concurrency; performs min-max scaling inline using `MinMaxScaler`; exposes stream/stats; mutate/write unimplemented.
- **Storage Runtime** ([gds/src/algo/scale_properties/storage.rs](gds/src/algo/scale_properties/storage.rs#L1-L399)): controller builds per-property scalers (scalar/array), validates dimensions, and produces scaled rows + stats.
- **Computation Runtime** ([gds/src/algo/scale_properties/computation.rs](gds/src/algo/scale_properties/computation.rs#L1-L12)): placeholder to satisfy controller pattern.

### Pattern Compliance
- Procedure currently bypasses the storage/computation runtimes and performs only min-max scaling of a single property. Mutate/write paths are stubbed. No progress/termination wiring. Configuration surface diverges from the algorithm module (multi-property + variant selection).

### Actions Needed
1. Wire the procedure to `ScalePropertiesStorageRuntime` + `ScalePropertiesComputationRuntime`, honoring multi-property configs and scaler variants.
2. Implement mutate/write modes or explicitly document them as unsupported.
3. Add progress/termination wiring consistent with other procedures.

---

## index_inverse - ✅ Architecturally Sound

### Layered Architecture
- **Procedure Layer** ([gds/src/procedures/miscellaneous/index_inverse.rs](gds/src/procedures/miscellaneous/index_inverse.rs#L1-L60)): configures mutate graph name/concurrency and delegates to storage with computation runtime.
- **Storage Runtime** ([gds/src/algo/index_inverse/storage.rs](gds/src/algo/index_inverse/storage.rs#L1-L68)): builds inverse indices via graph-store helper and returns new store.
- **Computation Runtime** ([gds/src/algo/index_inverse/computation.rs](gds/src/algo/index_inverse/computation.rs#L1-L12)): placeholder to keep controller pattern consistent.

### Pattern Compliance
- Procedure invokes storage with computation runtime; storage owns graph mutation; computation is graph-free. Relationship-type filtering remains TODO but does not affect architecture.

### Notes
- Add optional relationship-type filtering if parity with Java is required.

---

## collapse_path - ⚠️ Partially Compliant

### Layered Architecture
- **Procedure Layer** ([gds/src/procedures/miscellaneous/collapse_path.rs](gds/src/procedures/miscellaneous/collapse_path.rs#L1-L37)): validates optional `max_hops` and calls graph-store helper `collapse_paths_degree2`.
- **Storage/Computation**: Uses `DefaultGraphStore::collapse_paths_degree2` directly; the translated algo module exists (CollapsePath storage/computation runtimes), but the procedure does not call them.

### Pattern Compliance
- Controller pattern is bypassed; no progress/termination wiring; relies on store helper instead of the translated storage/computation runtimes.

### Actions Needed
1. Rewire procedure to `CollapsePathStorageRuntime` + computation runtime with proper config.
2. Add progress/termination wiring and error surfacing through `AlgorithmError`.

---

## to_undirected - ⚠️ Partially Compliant

### Layered Architecture
- **Procedure Layer** ([gds/src/procedures/miscellaneous/to_undirected.rs](gds/src/procedures/miscellaneous/to_undirected.rs#L1-L41)): calls `DefaultGraphStore::to_undirected` and provides a simple stats path.
- **Storage/Computation**: Translated algo module exists (`ToUndirectedStorageRuntime` + computation), but the procedure bypasses it.

### Pattern Compliance
- Bypasses controller/computation; no progress/termination wiring; uses store helper instead of algo runtimes.

### Actions Needed
1. Rewire to `ToUndirectedStorageRuntime` + computation runtime.
2. Add progress/termination wiring and align config surface with the algo spec.

---

## indirect_exposure - ❌ Non-Compliant

### Layered Architecture
- **Algorithm** exists (Pregel-based) under [gds/src/algo/indirect_exposure/*](gds/src/algo/indirect_exposure/mod.rs#L1-L8), but there is **no procedure layer** in miscellaneous.

### Pattern Compliance
- Missing procedure facade; storage/computation exist, but no controller entrypoint in the procedures layer.

### Actions Needed
1. Add a miscellaneous procedure facade that validates config, wires progress/termination, and calls `IndirectExposureStorageRuntime` + `IndirectExposureComputationRuntime`.
2. Expose stream/stats/mutate/write shapes consistent with other Pregel algos, or document supported modes explicitly.

---

## Scope Conclusion

- **Next Steps (priority)**: (a) add an indirect_exposure procedure facade; (b) rewire scale_properties to its storage/computation runtimes; (c) rewire collapse_path and to_undirected to their algo runtimes with progress/termination; (d) decide on mutate/write support for scale_properties.
- Once rewired, re-run this review to confirm controller-pattern compliance across all miscellaneous algorithms.
