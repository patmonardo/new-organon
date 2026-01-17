# Node Pipeline 3C Pass (Plan)

Scope: 3C (Correctness / Completeness / Compatibility) upgrade for the **existing node pipeline** implementation, aligned to Java GDS sources.

## Primary Rust scope (existing)

- gds/src/projection/eval/pipeline/node_pipeline/mod.rs
- gds/src/projection/eval/pipeline/node_pipeline/node_property_training_pipeline.rs
- gds/src/projection/eval/pipeline/node_pipeline/node_property_predict_pipeline.rs
- gds/src/projection/eval/pipeline/node_pipeline/node_property_prediction_split_config.rs
- gds/src/projection/eval/pipeline/node_pipeline/node_property_pipeline_base_train_config.rs
- gds/src/projection/eval/pipeline/node_pipeline/node_feature_step.rs
- gds/src/projection/eval/pipeline/node_pipeline/node_feature_producer.rs
- gds/src/projection/eval/pipeline/node_pipeline/classification/\*
- gds/src/projection/eval/pipeline/node_pipeline/regression/\*

## Java sources (reference)

- gds/src/projection/pipeline/nodePipeline/NodeFeatureProducer.java
- gds/src/projection/pipeline/nodePipeline/NodeFeatureStep.java
- gds/src/projection/pipeline/nodePipeline/NodePropertyPipelineBaseTrainConfig.java
- gds/src/projection/pipeline/nodePipeline/NodePropertyPredictionSplitConfig.java
- gds/src/projection/pipeline/nodePipeline/NodePropertyPredictPipeline.java
- gds/src/projection/pipeline/nodePipeline/NodePropertyTrainingPipeline.java
- gds/src/projection/pipeline/nodePipeline/classification/\*
- gds/src/projection/pipeline/nodePipeline/regression/\*

## 3C checklist

### Correctness

- [ ] Train config parsing defaults parity (base + classification + regression).
- [ ] Prediction split config parity (train/test/feature input filtering).
- [ ] Feature step ordering parity with Java pipeline definitions.
- [ ] Feature production parity (node property derivations, required inputs).
- [ ] Result conversion parity (model info + metrics payload).

### Completeness

- [ ] All Java node pipeline entrypoints have Rust equivalents.
- [ ] Node classification + regression variants present and wired.
- [ ] Pipeline executor integration paths exist for train/predict.
- [ ] Error surfaces mapped (config, validation, execution, cleanup).

### Compatibility

- [ ] Error message text aligns with Java (user-facing).
- [ ] Output schema compatibility (result fields + defaults).
- [ ] Feature pipeline serialization keys parity.

## Parity map (node pipeline entrypoints)

| Java entrypoint                       | Rust entrypoint                               | Parity notes                                   | 3C action                                     |
| ------------------------------------- | --------------------------------------------- | ---------------------------------------------- | --------------------------------------------- |
| `NodePropertyTrainingPipeline`        | `node_property_training_pipeline.rs`          | Train flow parity (split → steps → train).     | Verify dataset split + step ordering.         |
| `NodePropertyPredictPipeline`         | `node_property_predict_pipeline.rs`           | Predict flow parity (split → steps → predict). | Verify feature input filter + output mapping. |
| `NodePropertyPredictionSplitConfig`   | `node_property_prediction_split_config.rs`    | Config defaults parity.                        | Align defaults + validation errors.           |
| `NodePropertyPipelineBaseTrainConfig` | `node_property_pipeline_base_train_config.rs` | Base train config parity.                      | Validate defaults + required fields.          |
| `NodeFeatureStep`                     | `node_feature_step.rs`                        | Step definition parity.                        | Check serialization keys + ordering.          |
| `NodeFeatureProducer`                 | `node_feature_producer.rs`                    | Feature extraction parity.                     | Validate property naming + cleanup.           |
| `classification/*`                    | `classification/*`                            | Classification variants parity.                | Align training result conversion.             |
| `regression/*`                        | `regression/*`                                | Regression variants parity.                    | Align training result conversion.             |

## Tests to add

- Split config defaults for node prediction (feature input = union of train/test/context).
- Feature step order tests (pipeline steps execute in correct sequence).
- Error message parity for missing node properties.
- Classification vs regression result schema parity tests.

## Examples (to add)

- examples/node_pipeline_basic.md — minimal node pipeline config + small graph.
- examples/node_pipeline_classification.md — classification training/prediction config.
- examples/node_pipeline_regression.md — regression training/prediction config.

## Next actions

1. Validate config defaults and error messages against Java sources.
2. Add tests for split config and step ordering.
3. Draft the example markdowns listed above.
