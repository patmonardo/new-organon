# ML Eval Deep Dive (Projection/Eval — ML ISA)

This document is a technical tour of the **ML Eval system** in the `gds` crate.

It is written to help you *read the code as a system*, and to clarify what is **stable today** vs what is **transitional** while descriptor tables migrate toward a proc-macro “Reality” injector.

## What is “ML Eval” in this repo?

**Projection/Eval** is the **Revealing power**: it takes an Image (`GraphStore`) and reveals derived structure.

Within Eval, the **ML ISA** is the subsystem responsible for:

- building and validating ML pipelines
- executing *node property steps* (graph algorithms used as feature generators)
- executing *feature steps* (feature extraction / assembly)
- training/evaluating models (where implemented)

## Where the ML ISA lives

The ML ISA root module is:

- `gds/src/projection/eval/ml/mod.rs`

**As of 2025-12**, the compiling, actively-maintained ML runtime is the Java GDS pipeline translation:

- `gds/src/projection/eval/ml/pipeline/**`

If you are looking for “the ML engine”, start there.

## Two ML layers (important)

### 1) “Pipeline as Runtime” (active today)

The `ml/pipeline/**` tree is a direct translation of Neo4j GDS’s ML pipeline subsystem.

Key idea: a pipeline is a **sequence of steps** that transforms a graph Image into features, splits, and (eventually) trained models.

Core traits/types:

- `Pipeline` — describes a pipeline’s *node property steps* + *feature steps*.
- `ExecutableNodePropertyStep` — an executable graph-algorithm step that mutates properties into the graph.
- `FeatureStep` — a feature extraction step that consumes properties.
- `PipelineExecutor` — template-method runtime orchestration.
- `NodePropertyStepExecutor` — executes the node property steps in order.

### 2) “Descriptor tables as Island of Truth” (emerging)

There are older stubs that assume a descriptor table layer (e.g., `PipelineDescriptor`, `StepDescriptor`).

Those are not currently wired into the module tree, because the plan is:

- Reality holds **Pure A Priori Descriptor Tables** (the “Island of Truth”)
- proc-macros inject those tables into the Projector
- the Projector becomes an **emerging feature of Reality**

So: **today** you learn ML Eval through `ml/pipeline/**`. **Tomorrow** we drive it through injected descriptor tables.

## The execution picture (Factory → Eval → ML)

ML Eval runs on an **Image** (`GraphStore`). In broad strokes:

1. you already have a `GraphStore` (from Projection/Factory, or constructed in tests)
2. you choose/construct a pipeline
3. you run the pipeline executor against the store

Inside the pipeline execution, the key reveal is:

- graph algorithms become **feature producers** (node properties)
- features become ML inputs

## The pipeline contract: `Pipeline`

See:

- `gds/src/projection/eval/ml/pipeline/pipeline_trait.rs`

A `Pipeline` has:

- `node_property_steps()` → `ExecutableNodePropertyStep`s
- `feature_steps()` → `FeatureStep`s

Validation logic is split:

- `validate_before_execution(graph_store, node_labels)`
  - checks that feature step input properties exist, *or* will be created by node property steps
- `validate_feature_properties(graph_store, node_labels)`
  - intended to run after node property steps

Note: `feature_properties_missing_from_graph()` is now implemented via `GraphStore::node_property_keys_for_labels()`.

## The node property step: `ExecutableNodePropertyStep`

See:

- `gds/src/projection/eval/ml/pipeline/executable_node_property_step.rs`

This is the “graph algorithm bridge” for ML pipelines.

Its central method:

- `execute(graph_store: &mut DefaultGraphStore, node_labels, relationship_types, concurrency)`

So the ML pipeline can mutate the image with computed properties.

## Orchestration: `PipelineExecutor` (template method)

See:

- `gds/src/projection/eval/ml/pipeline/pipeline_executor.rs`

This trait encodes the standardized pipeline lifecycle:

- generate split filters
- split datasets
- execute node property steps
- execute pipeline-specific logic

The design is intentionally “template method”:

- you get a fixed lifecycle
- you override the few pipeline-specific operations

That’s the *engineering meaning* of “ISA”: a stable instruction format + stable execution rhythm.

## What to read first (recommended route)

1. `Pipeline` trait: `pipeline_trait.rs`
2. `ExecutableNodePropertyStep` trait: `executable_node_property_step.rs`
3. `NodePropertyStep` + factory: `node_property_step.rs`, `node_property_step_factory.rs`
4. `NodePropertyStepExecutor`: `node_property_step_executor.rs`
5. `PipelineExecutor`: `pipeline_executor.rs`
6. Then the concrete node/link pipeline implementations under:
   - `node_pipeline/**`
   - `link_pipeline/**`

## Where progress/trace belongs

ML is an evaluation system; progress is a **temporal tool**.

The natural hook points are:

- per node-property step begin/end
- per dataset split phase
- per training trial

(We now have a small generic trace surface at `gds/src/projection/trace.rs` that can be adopted when you’re ready.)

## Next step

If you tell me which pipeline you want to understand first (node classification vs link prediction), I’ll:

- annotate the exact execution path through the relevant executor
- and draft the minimal Form ISA interface that can *compose* ML + Procedure as Thesis/Antithesis, without dragging in the future Reality descriptor tables yet.
