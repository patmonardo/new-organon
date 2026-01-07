# GraphStore Catalog - Architectural Review

## Overview

This document reviews the **GraphStore Catalog Applications** under:

- `gds/src/applications/graph_store_catalog/*`

The GraphStore Catalog is treated as the **Storage ISA** and (today) is the most important “Applications Form” surface.

The immediate goal for review/upgrade is **3C compliance**:

- **Correctness**: behavior and invariants are right; no panics on user input
- **Completeness**: the surface is coherent and end-to-end usable (for the supported operations)
- **Compatibility**: aligns with Java GDS semantics and with our repo’s procedure-first guardrails

This review is intentionally scoped to **Applications → Procedures control logic** and the **GraphStore Catalog forms** (not yet “true Applications Components/Forms”).

---

## Current Surface

### Top-level Applications entrypoint

- `ApplicationsFacade` wires sub-facades and is the integration handle.
  - File: [gds/src/applications/graph_store_catalog/facade/applications_facade.rs](../src/applications/graph_store_catalog/facade/applications_facade.rs)
  - Current stance: GraphStore Catalog is the primary concrete facade; other facades are placeholders.

### Primary facade contract (TS consumer surface)

- `GraphCatalogApplications` is the main trait interface and is explicitly described as the TS-facing contract.
  - File: [gds/src/applications/graph_store_catalog/facade/graph_catalog_applications.rs](../src/applications/graph_store_catalog/facade/graph_catalog_applications.rs)
  - Notes:
    - Many methods return `Result<_, String>` (stringly typed errors).
    - The surface includes: list/drop/stream/write/export/project/generate/sampling/mutation.

### Default implementation (orchestrator)

- `DefaultGraphCatalogApplications` is a concrete orchestrator that composes per-op application structs.
  - File: [gds/src/applications/graph_store_catalog/facade/default_graph_catalog_applications.rs](../src/applications/graph_store_catalog/facade/default_graph_catalog_applications.rs)
  - Notes:
    - This is the “wiring hub” analogous to Java’s `DefaultGraphCatalogApplications`.
    - It owns factories for task/progress registries and user logging registries.

---

## Architectural Snapshot

The GraphStore Catalog Apps already follow a useful decomposition:

- `applications/*`: one application per operation (drop/list/stream/project/export/etc.)
- `configs/*`: request configs (often JSON-facing)
- `services/*`: cross-cutting utilities (validation, listing, preconditions, progress)
- `loaders/*`: catalog/store loading and per-user catalog services
- `results/*`: typed results (and sometimes JSON rendering)
- `facade/*`: the stable contract surface

This is directionally similar to the **Algorithms Applications + Machinery** split, but currently GraphStore Catalog does not have a single obvious “Machinery-style” orchestration template.

---

## 3C Review (Correctness, Completeness, Compatibility)

### Correctness

Primary correctness goals for the Storage ISA:

1. **Never panic on user-driven inputs**
   - Any request decoding, config validation, and precondition failures should return structured errors.
   - Example current sharp edge:
     - `GraphListingService::default()` panics (acceptable internally, but must never be reachable from a form/dispatch path).
       - File: [gds/src/applications/graph_store_catalog/services/graph_listing_service.rs](../src/applications/graph_store_catalog/services/graph_listing_service.rs)

2. **Catalog invariants are maintained**
   - Operations that mutate the graph store must reinsert the updated store back into the catalog scope.
   - Example: `drop_node_properties` fetches store → computes modified store → sets it back into the catalog.
     - File: [gds/src/applications/graph_store_catalog/facade/default_graph_catalog_applications.rs](../src/applications/graph_store_catalog/facade/default_graph_catalog_applications.rs)

3. **Validation is stable and predictable**
   - Config schemas (especially `from_json`) should reject malformed input with consistent error codes.
   - Current tests sometimes assert by matching an error variant and `panic!` otherwise; that’s fine for tests but hints that the public error contract is not yet formalized.

### Completeness

Completeness here means: **given the supported substrate** (catalog + in-memory stores + available loaders), the forms should provide an end-to-end usable workflow:

- list → inspect memory usage → project/generate → stream → write/export → drop

Current completeness gaps to watch for (based on existing inventory work and file structure):

- **Error model**: `Result<_, String>` blocks typed errors and consistent JSON error payloads.
- **Progress tracking parity**: there are task registry factories, but not all applications necessarily report progress consistently.
- **Memory estimation/guardrails**: there is `MemoryUsageValidator`, but it’s not yet obvious that every mutating/projection/export op uses it uniformly.

### Compatibility (Java GDS + repo guardrails)

Compatibility targets:

1. **Java GDS API parity** for GraphStore Catalog semantics
   - method names and behaviors match the Java `applications.graphstorecatalog` package conceptually.

2. **Repo guardrails**
   - Applications should call only the appropriate higher-level “facade” surfaces.
   - If/when GraphStore Catalog operations invoke graph algorithms, they must do so via `procedures::*` (never `algo::*`).

---

## Focus Areas for Tomorrow (What to Review / Upgrade)

This is the “worklist” for getting GraphStore Catalog forms closer to 3C-compliant.

### 1) Formalize the error contract (stop returning `String`)

- Define a typed Applications error shape for GraphStore Catalog ops (at minimum: `code`, `message`, and structured `details`).
- Ensure the trait surface (`GraphCatalogApplications`) returns this typed error (or a shared `facade::Error`).
- Replace any panic paths in request handling with this error.

### 2) Validate every form/config path

- Audit `configs/*` for:
  - required fields present
  - empty lists rejected where illegal
  - mutually exclusive options enforced
  - stable parameter naming (Java parity + JSON parity)

### 3) Catalog mutation is always transactional

- For every op that mutates the graph store, verify:
  - it fetches the current store from the catalog
  - it returns a modified store (or executes via a catalog service abstraction)
  - it re-inserts the updated store into the catalog, in the correct user/database scope

### 4) Progress + memory guardrails are uniform

- Identify a small set of “standard steps” for each op type:
  - resolve graph
  - validate config
  - estimate memory
  - execute
  - render result
  - persist/export

…and ensure every application reports progress in those steps.

### 5) Define the GraphStore Catalog “Machinery-equivalent” (optional)

If we want a parallel to AlgorithmMachinery:

- a template runner for GraphStore Catalog operations that standardizes:
  - request-scoped dependency construction
  - progress tracker creation
  - memory policy enforcement
  - consistent logging and error conversion

This may be a future refactor; the immediate goal is 3C compliance, not re-architecture.

---

## Pointers to Existing Notes

- Inventory note that already references some GraphStore Catalog paths:
  - [gds/doc/APPLICATIONS_CONTROL_LOGIC_INVENTORY.md](APPLICATIONS_CONTROL_LOGIC_INVENTORY.md)

- Broader architecture notes including an Applications Form example for GraphStore Catalog:
  - [gds/doc/ARCHITECTURE.md](ARCHITECTURE.md)

