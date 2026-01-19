# Eval Pipeline/Procedure 3C Pass (Prep)

Scope: translation parity review for the Eval/Apply executors.

## Central “Form Processor” files (Rust)

- Procedure executor: gds/src/projection/eval/procedure/executor.rs
- Algorithm spec contract: gds/src/projection/eval/procedure/algorithm_spec.rs
- Pipeline executor: gds/src/projection/eval/pipeline/pipeline_executor.rs
- Node property steps: gds/src/projection/eval/pipeline/node_property_step_executor.rs

## Java sources (reference)

- pipeline/src/main/java/org/neo4j/gds/ml/pipeline/PipelineExecutor.java
- executor/src/main/java/org/neo4j/gds/executor/ProcedureExecutor.java
- executor/src/main/java/org/neo4j/gds/executor/AlgorithmSpec.java

## 3C Checklist (Correctness / Completeness / Compatibility)

### Procedure Executor (Eval/Procedure)

- [ ] Flow parity: preprocess → parse → validate before load → load → validate after load → execute → consume
- [ ] Empty graph short-circuit behavior matches Java (result + flags)
- [ ] Timing + logging parity (stages and totals)
- [ ] Validation configuration wiring (pre/post load)
- [ ] Error surfaces: config, validation, context, algorithm, consumer
- [ ] ExecutionMode handling matches result mapping

#### Method-level parity map (ProcedureExecutor)

| Java                                          | Rust                                              | Parity Notes                                                                           | 3C Action                                        |
| --------------------------------------------- | ------------------------------------------------- | -------------------------------------------------------------------------------------- | ------------------------------------------------ |
| `compute(String, Map)`                        | `ProcedureExecutor::compute()`                    | Template flow preserved; includes preprocess, parse, validate, load, execute, consume. | Confirm error mapping vs Java exceptions.        |
| `preProcessConfig(...)`                       | `AlgorithmSpec::preprocess_config()`              | Optional pre-parse hook matches Java.                                                  | Check pipeline/model enhancement parity.         |
| `configParser().processInput(...)`            | `AlgorithmSpec::parse_config()`                   | Direct parse in Rust; Java has parser factories.                                       | Align validation error messages.                 |
| `validator.validateConfigsBeforeLoad(...)`    | `ValidationConfiguration::validate_before_load()` | Present.                                                                               | Ensure identical constraints.                    |
| `graphCreation.graphStore()`                  | `ExecutionContext::load_graph()`                  | Present.                                                                               | Verify graph name handling + catalog errors.     |
| `validator.validateConfigWithGraphStore(...)` | `ValidationConfiguration::validate_after_load()`  | Present.                                                                               | Confirm graph schema parity checks.              |
| `algo.compute()`                              | `AlgorithmSpec::execute()`                        | Present.                                                                               | Ensure timing + termination flags if applicable. |
| `computationResultConsumer.consume(...)`      | `AlgorithmSpec::consume_result()`                 | Present.                                                                               | Validate ExecutionMode output mapping.           |

#### Invariants to validate (Procedure)

- Empty graph handling: Java returns “empty result” flags; Rust marks graph empty on `ComputationResult`. Confirm any missing flags or metadata.
- Timing breakdown: preprocess + load + compute + consume. Ensure logging parity and metric keys.
- Error surfaces: verify conversion of `ContextError`, `ValidationError`, `AlgorithmError`, `ConsumerError` into `ExecutorError` matches Java semantics.

### Pipeline Executor (Eval/Pipeline)

- [ ] Dataset split creation parity (TRAIN/TEST/TEST_COMPLEMENT/FEATURE_INPUT)
- [ ] Pre-validation of pipeline against featureInput graph filter
- [ ] Node property step context validation + execution ordering
- [ ] Feature property validation timing (post steps)
- [ ] Cleanup semantics (intermediate properties + additional cleanup)
- [ ] GraphStore mutation visibility (Arc + executor borrow pattern)
- [ ] Concurrency handling parity in step executor

#### Method-level parity map (PipelineExecutor)

| Java                                               | Rust                                                                  | Parity Notes             | 3C Action                                                 |
| -------------------------------------------------- | --------------------------------------------------------------------- | ------------------------ | --------------------------------------------------------- |
| `generateDatasetSplitGraphFilters()`               | `PipelineExecutor::generate_dataset_split_graph_filters()`            | Present.                 | Confirm split semantics per pipeline kind (node vs link). |
| `splitDatasets()`                                  | `PipelineExecutor::split_datasets()`                                  | Present.                 | Ensure split properties match Java naming.                |
| `compute()`                                        | `PipelineExecutor::compute()`                                         | Template flow preserved. | Confirm cleanup ordering and error precedence.            |
| `execute(Map<DatasetSplits, PipelineGraphFilter>)` | `PipelineExecutor::execute(...)`                                      | Present.                 | Ensure proper graph filter routing to train/test.         |
| `getAvailableRelTypesForNodePropertySteps()`       | `PipelineExecutor::get_available_rel_types_for_node_property_steps()` | Present.                 | Confirm relationship filtering parity.                    |
| `additionalGraphStoreCleanup(...)`                 | `PipelineExecutor::additional_graph_store_cleanup(...)`               | Present (default noop).  | Confirm extra cleanup hooks in Rust.                      |

#### Invariants to validate (Pipeline)

- Feature input filter is the union of train/test/context (Java semantics).
- Node property steps execute after dataset split creation but before feature property validation.
- Cleanup occurs even when execution errors; error precedence matches Java (cleanup error overrides successful result).
- Step executor concurrency and mutability across `Arc<GraphStore>`.

#### Step 1: Error message mapping (Java → Rust)

| Java source (callsite)                                                     | Java exception/message                                                                                                           | Rust error variant                                                                                | Notes                                                          |
| -------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| `PipelineExecutor.compute()` -> `dataSplitGraphFilters.get(FEATURE_INPUT)` | Java would NPE if missing                                                                                                        | `PipelineExecutorError::MissingDatasetSplit("FEATURE_INPUT")`                                     | Rust makes this explicit.                                      |
| `Pipeline.validateBeforeExecution(...)`                                    | `IllegalArgumentException`: “Node properties ... do not exist in the graph or part of the pipeline”                              | `PipelineExecutorError::PipelineValidationFailed(PipelineValidationError::MissingNodeProperties)` | Preserve message parity in `PipelineValidationError::Display`. |
| `NodePropertyStepExecutor.validNodePropertyStepsContextConfigs(...)`       | `IllegalArgumentException` from `ElementTypeValidator.validate/validateTypes` using labels like “contextNodeLabels for step `X`” | `PipelineExecutorError::StepValidationFailed(...)`                                                | Align message formatting with Java’s label strings.            |
| `PipelineExecutor.splitDatasets()`                                         | Implementation-specific exceptions                                                                                               | `PipelineExecutorError::DatasetSplitFailed(String)`                                               | Standardize message text in Rust implementations.              |
| `NodePropertyStepExecutor.executeNodePropertySteps(...)`                   | Exceptions thrown by step execution                                                                                              | `PipelineExecutorError::StepExecutionFailed(...)`                                                 | Keep step name in error context.                               |
| `Pipeline.validateFeatureProperties(...)`                                  | `IllegalArgumentException`: “Node properties ... do not exist in the graph or part of the pipeline”                              | `PipelineExecutorError::FeatureValidationFailed(PipelineValidationError::MissingNodeProperties)`  | Same message as pre-validation, but post-steps.                |
| `PipelineExecutor.execute(...)`                                            | Implementation-specific exceptions                                                                                               | `PipelineExecutorError::ExecutionFailed(String)`                                                  | Ensure algorithm name/context included.                        |
| `cleanupIntermediateProperties(...)` / `additionalGraphStoreCleanup(...)`  | Runtime exceptions if any                                                                                                        | `PipelineExecutorError::CleanupFailed(...)`                                                       | Rust uses explicit cleanup error reporting.                    |

## Noted patterns

- Template method flow in both executors.
- “Step executor” is a separate unit to isolate validation/execution/cleanup.
- Pipeline executor depends on dataset splits and feature input filter.

## Concrete next actions (3C pass)

1. Map `PipelineExecutor` Java error messages to Rust `PipelineExecutorError` variants. ✅
2. Validate dataset split filters for link-prediction vs node-prediction pipelines. ✅
3. Compare `NodePropertyStepExecutor` execution order and property cleanup semantics. ✅
4. Align validation coverage (pre/post load) with `AlgorithmSpec` expectations. ⏳

## Next pass

1. Confirm dataset split filters against Java `PipelineExecutor.generateDatasetSplitGraphFilters()`. ✅
2. Compare step execution ordering vs Java `NodePropertyStepExecutor`. ✅
3. Align error types and messages for compatibility. ✅

---

# Forward path (module-level 3C passes)

This expands the plan into three focused passes, aligned to the natural module groups.

## Pass A: Pipeline\* modules

**Scope (Rust)**

- gds/src/projection/eval/pipeline/pipeline_executor.rs
- gds/src/projection/eval/pipeline/pipeline_graph_filter.rs
- gds/src/projection/eval/pipeline/pipeline_catalog.rs
- gds/src/projection/eval/pipeline/pipeline_companion.rs
- gds/src/projection/eval/pipeline/predict_pipeline_executor.rs

**3C checks**

- Dataset split graph filters match Java (node vs link pipelines)
- Pipeline catalog lookup + naming parity
- Graph filter semantics (labels/rel types) match Java
- Predict executor parity (input validation + cleanup)

**Artifacts**

- Update error mapping tables per executor (train/predict)
- Add tests for split-filter construction and missing split errors

**Status**: ✅ Complete

## Pass B: NodePropertyStep\* modules

**Scope (Rust)**

- gds/src/projection/eval/pipeline/node_property_step_executor.rs
- gds/src/projection/eval/pipeline/node_property_step.rs
- gds/src/projection/eval/pipeline/node_property_step_factory.rs
- gds/src/projection/eval/pipeline/node_property_step_context_config.rs

**3C checks**

- Context config validation parity (labels + relationship types)
- Feature input labels/types resolution parity
- Cleanup semantics for mutate properties
- Error phrasing parity for context validation + execution failures

**Artifacts**

- Message parity tests for context config errors
- Execution-order tests (step sequencing + cleanup always runs)

**Status**: ✅ Complete

## Pass C: Training / Prediction modules

**Scope (Rust)**

- gds/src/projection/eval/pipeline/training_pipeline.rs
- gds/src/projection/eval/pipeline/predict_pipeline_executor.rs
- gds/src/projection/eval/pipeline/train/\*
- gds/src/projection/eval/pipeline/link_pipeline/\*
- gds/src/projection/eval/pipeline/node_pipeline/\*

**3C checks**

- Train config parsing + defaults parity
- Split config parity (train/test/feature input)
- Feature extraction parity (link vs node)
- Model result conversion parity
- Metrics + result schema parity

**Artifacts**

- Per-pipeline parity checklist (link vs node)
- Targeted tests for split configs and feature extraction ordering

**Status**: ✅ Core coverage complete (node/link deep dive pending)

## Suggested execution order

1. Complete Pass A (core pipeline executor semantics)
2. Complete Pass B (step validation/execution contract)
3. Complete Pass C (train/predict + pipeline variants)

## Descent plan (Node & Link Pipelines)

### Node pipelines

- Node training config parsing + validation
- Node prediction split config parity
- Feature pipeline serialization keys (nodePropertySteps / featureProperties)
- Model result conversion + metrics schema

### Link pipelines

- Link prediction split config graph validation (reserved rel types, negative sampling)
- Link feature extraction ordering (hadamard/cosine/l2)
- Train result + metrics schema alignment

### Common

- Ensure ProcedureExecutor integration path is available for both (no Stub dependency)
- Verify user-facing error phrasing

## Node Pipeline Plan (detailed)

**Goal:** produce a concrete 3C sub-plan for node-oriented pipelines that documents checks, artifacts, and example scenarios.

**Scope (Rust)**

- gds/src/projection/eval/pipeline/node_pipeline.rs
- gds/src/projection/eval/pipeline/node_pipeline_companion.rs
- gds/src/projection/eval/pipeline/node_property_step_executor.rs

**3C checks**

- Training config parsing: defaults, optional fields, and legacy keys parity with Java.
- Split semantics: ensure `FEATURE_INPUT` is union of train/test/context and node-label filtering matches Java.
- Node property steps: ordering, context label resolution, and relationship-type gating.
- Feature validation: post-step validation timing and missing-property errors.
- Cleanup semantics: intermediate property naming, overwrite vs append behavior, and guaranteed cleanup on error.

**Artifacts**

- Example configs (train/predict) for node pipelines.
- Unit tests for `generate_dataset_split_graph_filters()` focused on node-label combinations.
- Message-parity tests for node step context validation errors.

**Example next steps**

- Add `examples/node_pipeline_example.md` with a minimal graph + pipeline JSON.
- Implement and run tests for split filter generation.

## Link Pipeline Plan (detailed)

**Goal:** produce a concrete 3C sub-plan for link-oriented pipelines covering negative sampling, rel-type handling, and feature extraction parity.

**Scope (Rust)**

- gds/src/projection/eval/pipeline/link_pipeline.rs
- gds/src/projection/eval/pipeline/link_pipeline_companion.rs
- gds/src/projection/eval/pipeline/node_property_step_executor.rs (shared)

**3C checks**

- Split semantics: link-specific TRAIN/TEST semantics, reserved rel-types handling, and negative sampling parity with Java.
- Edge feature extraction ordering: ensure pairwise operators (hadamard/cosine/l2) are applied in the same order as Java.
- Relationship-type filtering: validate that available rel types reported to steps matches Java behavior.
- Cleanup semantics: ensure intermediate properties from link steps are removed and visibility rules mirror Java.

**Artifacts**

- Example configs (link-train/link-predict) including negative-sampling parameters.
- Tests for negative sampling behavior and rel-type filtering.
- A short RFC snippet documenting differences (if any) versus Java.

**Example next steps**

- Add `examples/link_pipeline_example.md` demonstrating negative sampling and a small rel-type graph.
- Add tests validating hadamard/cosine/l2 extraction ordering and outputs.

---

I've added the Node and Link pipeline plans and example next steps above; let me know if you want me to create the example files and tests now.
