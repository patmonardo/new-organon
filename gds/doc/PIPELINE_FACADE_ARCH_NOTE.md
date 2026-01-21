# Pipeline Facade Architectural Note (Procedures → Projection/Eval)

## Purpose

This note captures the minimal architecture for how the procedures/pipelines facade should call into the projection/eval/pipeline system. It is intentionally short and implementation-facing.

## Layering Rules

- **Procedures layer owns orchestration**: request parsing, validation, and catalog lookups live in `gds/src/procedures/pipelines`.
- **Pipeline logic lives in projection/eval**: pipeline state, validation, and execution are in `gds/src/projection/eval/pipeline`.
- **No new code in `src/core`** for this translation.
- **Applications talk to procedures**; procedures drive pipelines.

## Facade → Pipeline Mapping

### Create

- **Link Prediction**
  - Facade creates `LinkPredictionTrainingPipeline` and stores in `PipelineCatalog`.
- **Node Classification**
  - Facade creates `NodeClassificationTrainingPipeline` and stores in `PipelineCatalog`.
- **Node Regression**
  - Facade creates `NodeRegressionTrainingPipeline` and stores in `PipelineCatalog`.

### Configure (Split / Auto‑tuning)

- Facade parses config and updates the pipeline in `PipelineCatalog`:
  - `LinkPredictionSplitConfig` / `AutoTuningConfig` for Link Prediction.
  - `NodePropertyPredictionSplitConfig` / `AutoTuningConfig` for node pipelines.

### Add Feature / Add Node Property

- **Link Prediction**
  - `add_feature` uses `LinkFeatureStepFactory` and updates the `LinkPredictionTrainingPipeline`.
  - `add_node_property` adds `NodePropertyStep` (validated by `ExecutableNodePropertyStep`).
- **Node Classification / Regression**
  - `add_node_property` adds `NodePropertyStep` to the node pipeline.
  - `select_features` replaces `NodeFeatureStep` list for node pipelines.

### Train

- **Link Prediction**
  - Use the link training executor (`link_pipeline/train/...`) and render results via `LinkPredictionTrainResult`.
- **Node Classification**
  - Use `NodeClassificationTrainAlgorithm` / factory + pipeline trainer; convert to model via `node_classification_to_model_converter`.
- **Node Regression**
  - Mirror node classification with regression equivalents.

### Predict (Stream / Mutate / Write)

- **Node Pipelines**
  - Use `node_property_predict_pipeline` with `predict_pipeline_executor`.
  - Stream/mutate/write result builders remain pipeline‑specific.
- **Link Prediction**
  - Use link prediction predict executor (link pipeline predict module) for stream/mutate.

### Estimate

- Estimates mirror the corresponding execute paths, using pipeline estimators:
  - Link prediction estimator for LP.
  - Node classification/regression estimators for node pipelines.

## Summary

The procedures facade is the stable public API. It should remain thin and delegate to the projection/eval pipeline system for all pipeline state and execution. This ensures consistent behavior across Link Prediction, Node Classification, and Node Regression while keeping orchestration in procedures and execution in projection/eval.
