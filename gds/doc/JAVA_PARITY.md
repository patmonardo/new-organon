# Java Parity Map: Applications Algorithms Machinery (Rust)

This folder is a **Rust parity scaffold** for the Java package:
`org.neo4j.gds.applications.algorithms.machinery`.

The goal is to keep the *control mechanism* stable and step-oriented.

## File Map (Rust)

### Progress / lifecycle
- `algorithm_machinery.rs`
  - Java: `AlgorithmMachinery`
  - Status: implemented (requested concurrency + begin/end/failure + optional release)

- `progress_tracker_creator.rs`
  - Java: `ProgressTrackerCreator`
  - Status: implemented (JobId + TaskRegistryFactory + TerminationFlag)

- `request_scoped_dependencies.rs`
  - Java: request scoped deps holder
  - Status: minimal implemented (job_id, task_registry_factory, termination_flag)

### Memory guard
- `memory_guard.rs`
  - Java: `MemoryGuard` + `DefaultMemoryGuard`
  - Status: implemented (DISABLED guard + default guard asserts/reserves via `MemoryTracker`; skips when estimation not implemented)

### Labels
- `label.rs`
  - Java: `Label`, `StandardLabel`, `AlgorithmLabel`
  - Status: implemented (typed labels to avoid string plumbing; `AlgorithmLabel::from(Algorithm)` mapping)

### Write (procedure-specific)
- `write_context.rs`
  - Java: `WriteContext`
  - Status: scaffolded (typed container + provider traits; exporter APIs mocked)
- `write_to_database.rs`
  - Java: `WriteToDatabase`
  - Status: scaffolded (logs + preserves call-shape; best-effort writes to in-memory GraphStore via GraphStoreService)

### Processing template
- `algorithm_processing_template.rs`
  - Java: `AlgorithmProcessingTemplate`
  - Status: implemented (tracker creation + compute + optional side effects + render, all inside machinery lifecycle)

- `algorithm_processing_template_convenience.rs`
  - Java: "Template Convenience" / mode assembly helper
  - Status: implemented (explicit entry points + generic `process_with_model`)

- `algorithm_processing_timings.rs`
  - Java: timings model
  - Status: implemented (pre-processing / compute / side-effect)

### Rendering
- `result_renderer.rs`
  - Java: renderer interface(s)
  - Status: implemented (`ResultRenderer`)

- `result_builder.rs`
  - Java: result builder interface(s)
  - Status: implemented
    - stats: `StatsResultBuilder`
    - stream: `StreamResultBuilder`
    - mutate: `MutateResultBuilder`
    - write: `WriteResultBuilder` (mock semantics; correct control-shape)

- `renderers.rs`
  - Java: concrete renderer models for stats/stream/mutate/write
  - Status: implemented
    - stats: `StatsResultRenderer`
    - stream: `StreamResultRenderer`
    - mutate: `MutateResultRenderer`
    - write: `WriteResultRenderer` (mock semantics; correct control-shape)

- `render_model.rs`
  - Java: per-mode "render model" objects (stats/stream/mutate/write)
  - Status: implemented
    - `StatsRenderModel`, `StreamRenderModel`, `MutateRenderModel`, `WriteRenderModel`

### Side effects / steps
- `side_effect.rs`
  - Java: side effect abstraction
  - Status: implemented (`SideEffect` + `SideEffectExecutor`)

- `steps.rs`
  - Java: step interfaces
  - Status: implemented (`MutateStep`, `WriteStep`)

- `side_effects.rs`
  - Java: mutate/write side-effect wrappers
  - Status: implemented
    - closure-backed: `MutateSideEffect`, `WriteSideEffect`
    - step-backed: `MutateStepSideEffect`, `WriteStepSideEffect`

### Services (Java files that live in the same folder)
- `computation_service.rs`
  - Java: `ComputationService`
  - Status: implemented (guard-first compute; metrics simplified; returns guard failure as error)

- `graph_store_service.rs`
  - Java: `GraphStoreService`
  - Status: implemented (adds node properties into graph store; filtered-graph translation not implemented)

## What’s Still Missing (Not Yet Translated)

These are the “likely Java pieces” that are *not* present yet in this folder:

- A richer **Write Context** model
  - Java has DB-specific write plumbing.
  - Rust status: intentionally mocked (we only preserve control-flow shape).

- Any domain-specific result schemas (algorithm-specific)
  - Those belong outside this folder; this folder is control-plane.

## Next Translation Target

Implement `AlgorithmProcessingTemplateConvenience` in this folder to provide:
- `process_stats(...)`
- `process_stream(...)`
- `process_mutate(...)`
- `process_write(...)` (mock write side-effects, but shape correct)
