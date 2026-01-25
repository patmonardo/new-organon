use super::node_regression_pipeline_train_config::NodeRegressionPipelineTrainConfig;
use super::node_regression_train_result::NodeRegressionTrainResult;
use super::node_regression_training_pipeline::NodeRegressionTrainingPipeline;
use crate::collections::HugeDoubleArray;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::model::ModelCatalog;
use crate::core::utils::progress::{LeafTask, ProgressTracker, Task, TaskProgressTracker};
use crate::mem::{MemoryEstimation, MemoryEstimations};
use crate::ml::metrics::regression::RegressionMetric;
use crate::ml::metrics::{Metric, ModelCandidateStats};
use crate::ml::models::linear_regression::LinearRegressionTrainConfig;
use crate::ml::models::{Features, RegressionTrainerFactory, Regressor};
use crate::ml::node_prediction::NodeSplitter;
use crate::ml::splitting::TrainingExamplesSplit;
use crate::ml::training::statistics::TrainingStatistics;
use crate::procedures::AlgorithmsProcedureFacade;
use crate::projection::eval::pipeline::node_pipeline::node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;
use crate::projection::eval::pipeline::node_pipeline::node_property_training_pipeline::NodePropertyTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::NodeFeatureProducer;
use crate::projection::eval::pipeline::PipelineTrainer;
use crate::types::graph::Graph;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::prelude::GraphStore;
use std::collections::HashMap;
use std::sync::Arc;

/// Core training algorithm for node regression.
///
/// This implements the training loop:
/// 1. Extract target values from the target property
/// 2. Split data into train/test/validation sets
/// 3. Train a regression model
/// 4. Evaluate on train/test sets
/// 5. Retrain on full training set
pub struct NodeRegressionTrain {
    pipeline: NodeRegressionTrainingPipeline,
    train_config: NodeRegressionPipelineTrainConfig,
    targets: HugeDoubleArray,
    node_graph: Arc<dyn Graph>,
    node_feature_producer: NodeFeatureProducer<NodeRegressionPipelineTrainConfig>,
    progress_tracker: Box<dyn ProgressTracker>,
    termination_flag: TerminationFlag,
}

impl NodeRegressionTrain {
    /// Estimate memory requirements for training.
    pub fn estimate(
        _pipeline: &NodeRegressionTrainingPipeline,
        _configuration: &NodeRegressionPipelineTrainConfig,
        _model_catalog: &impl ModelCatalog,
        _algorithms_procedure_facade: &AlgorithmsProcedureFacade,
    ) -> Box<dyn MemoryEstimation> {
        // Note: Implement once memory estimation infrastructure is translated.
        MemoryEstimations::empty()
    }

    /// Create progress task for training.
    pub fn progress_task(_pipeline: &NodeRegressionTrainingPipeline, _node_count: u64) -> Task {
        // Note: Implement once the Tasks API is translated.
        Task::new("Node Regression Train Pipeline".to_string(), vec![])
    }

    /// Create a new NodeRegressionTrain instance.
    pub fn create(
        graph_store: Arc<DefaultGraphStore>,
        pipeline: NodeRegressionTrainingPipeline,
        config: NodeRegressionPipelineTrainConfig,
        node_feature_producer: NodeFeatureProducer<NodeRegressionPipelineTrainConfig>,
        progress_tracker: Box<dyn ProgressTracker>,
    ) -> Self {
        let node_graph = graph_store.get_graph();
        pipeline
            .split_config()
            .validate_min_num_nodes_in_split_sets(node_graph.node_count())
            .expect("Invalid split configuration for node count");

        let target_node_property = node_graph
            .node_properties(config.target_property())
            .expect("Missing target node property for regression");

        let mut targets = HugeDoubleArray::new(node_graph.node_count());
        for i in 0..node_graph.node_count() {
            let value = target_node_property.double_value(i as u64).unwrap_or(0.0);
            if value.is_nan() {
                panic!(
                    "Node with id {} has `{}` target property value `NaN`",
                    node_graph.to_original_node_id(i as i64).unwrap_or(i as i64),
                    config.target_property()
                );
            }
            if value.is_infinite() {
                panic!(
                    "Node with id {} has infinite `{}` target property value",
                    node_graph.to_original_node_id(i as i64).unwrap_or(i as i64),
                    config.target_property()
                );
            }
            targets.set(i, value);
        }

        let termination_flag = TerminationFlag::running_true();

        Self {
            pipeline,
            train_config: config,
            targets,
            node_graph,
            node_feature_producer,
            progress_tracker,
            termination_flag,
        }
    }

    /// Set termination flag for early stopping.
    pub fn set_termination_flag(&mut self, termination_flag: TerminationFlag) {
        self.termination_flag = termination_flag;
    }

    /// Run the training algorithm.
    pub fn run(
        &mut self,
    ) -> Result<NodeRegressionTrainResult, Box<dyn std::error::Error + Send + Sync>> {
        self.progress_tracker.begin_subtask();

        let split_config = self.pipeline.split_config();
        let node_count = self.node_graph.node_count();

        let node_splitter = NodeSplitter::new(
            Concurrency::available_cores(),
            node_count,
            Arc::new({
                let graph = Arc::clone(&self.node_graph);
                move |id| graph.to_original_node_id(id as i64).unwrap_or(id as i64)
            }),
            Arc::new({
                let graph = Arc::clone(&self.node_graph);
                move |id| {
                    graph
                        .to_mapped_node_id(id)
                        .expect("Mapped node id not found") as usize
                }
            }),
        );

        let node_splits = node_splitter.split(
            split_config.test_fraction(),
            split_config.validation_folds(),
            self.train_config.random_seed(),
            self.progress_tracker.as_mut(),
        );

        let metrics = resolve_metrics(&self.train_config);
        let metric_boxes: Vec<Box<dyn Metric>> = metrics
            .iter()
            .copied()
            .map(|m| Box::new(m) as Box<dyn Metric>)
            .collect();

        let mut training_statistics = TrainingStatistics::new(&metric_boxes);
        training_statistics.add_candidate_stats(ModelCandidateStats::new(
            serde_json::json!({}),
            HashMap::new(),
            HashMap::new(),
        ));

        let features = self
            .node_feature_producer
            .procedure_features(&self.pipeline)
            .map_err(|e| format!("Feature production failed: {e}"))?;

        let regressor = self.train_simple_model(node_splits.outer_split(), &features)?;
        self.evaluate_model(
            node_splits.outer_split(),
            &features,
            &regressor,
            &metrics,
            &mut training_statistics,
        );

        let retrained = self.retrain_best_model(node_splits.all_training_examples(), &features)?;

        self.progress_tracker.end_subtask();

        Ok(NodeRegressionTrainResult::new(
            retrained,
            training_statistics,
        ))
    }
}

impl PipelineTrainer for NodeRegressionTrain {
    type Result = NodeRegressionTrainResult;

    fn run(&mut self) -> Result<Self::Result, Box<dyn std::error::Error + Send + Sync>> {
        self.run()
    }

    fn is_terminated(&self) -> bool {
        !self.termination_flag.running()
    }
}

impl NodeRegressionTrain {
    fn train_simple_model(
        &self,
        split: &TrainingExamplesSplit,
        features: &Box<dyn Features>,
    ) -> Result<Box<dyn Regressor>, Box<dyn std::error::Error + Send + Sync>> {
        let train_set = to_u64_arc(split.train_set());
        let trainer_config = LinearRegressionTrainConfig::default();
        let progress =
            TaskProgressTracker::new(LeafTask::new("Train regression model".to_string(), 0));
        let trainer = RegressionTrainerFactory::create(
            &trainer_config,
            &self.termination_flag,
            progress,
            &Concurrency::available_cores(),
            self.train_config.random_seed(),
        );

        Ok(trainer.train(features.as_ref(), &self.targets, &train_set))
    }

    fn retrain_best_model(
        &self,
        all_training_examples: &Arc<Vec<i64>>,
        features: &Box<dyn Features>,
    ) -> Result<Box<dyn Regressor>, Box<dyn std::error::Error + Send + Sync>> {
        let train_set = to_u64_arc(all_training_examples.clone());
        let trainer_config = LinearRegressionTrainConfig::default();
        let progress =
            TaskProgressTracker::new(LeafTask::new("Retrain regression model".to_string(), 0));
        let trainer = RegressionTrainerFactory::create(
            &trainer_config,
            &self.termination_flag,
            progress,
            &Concurrency::available_cores(),
            self.train_config.random_seed(),
        );

        Ok(trainer.train(features.as_ref(), &self.targets, &train_set))
    }

    fn evaluate_model(
        &self,
        split: &TrainingExamplesSplit,
        features: &Box<dyn Features>,
        regressor: &Box<dyn Regressor>,
        metrics: &[RegressionMetric],
        training_statistics: &mut TrainingStatistics,
    ) {
        let outer_train_scores = evaluate_metrics(
            &split.train_set(),
            features,
            regressor.as_ref(),
            &self.targets,
            metrics,
        );

        for (metric, score) in outer_train_scores {
            training_statistics.add_outer_train_score(metric, score);
        }

        let test_scores = evaluate_metrics(
            &split.test_set(),
            features,
            regressor.as_ref(),
            &self.targets,
            metrics,
        );

        for (metric, score) in test_scores {
            training_statistics.add_test_score(metric, score);
        }
    }
}

fn resolve_metrics(config: &NodeRegressionPipelineTrainConfig) -> Vec<RegressionMetric> {
    if config.metrics().is_empty() {
        vec![RegressionMetric::MeanSquaredError]
    } else {
        config.metrics().to_vec()
    }
}

fn to_u64_arc(values: Arc<Vec<i64>>) -> Arc<Vec<u64>> {
    Arc::new(values.iter().map(|v| *v as u64).collect())
}

fn evaluate_metrics(
    eval_ids: &Arc<Vec<i64>>,
    features: &Box<dyn Features>,
    regressor: &dyn Regressor,
    targets: &HugeDoubleArray,
    metrics: &[RegressionMetric],
) -> HashMap<String, f64> {
    let mut predictions: Vec<f64> = Vec::with_capacity(eval_ids.len());
    let mut actuals: Vec<f64> = Vec::with_capacity(eval_ids.len());

    for node_id in eval_ids.iter() {
        let idx = *node_id as usize;
        let prediction = regressor.predict(features.get(idx));
        predictions.push(prediction);
        actuals.push(targets.get(idx));
    }

    let actuals = HugeDoubleArray::from_vec(actuals);
    let predictions = HugeDoubleArray::from_vec(predictions);

    metrics
        .iter()
        .map(|metric| {
            (
                metric.name().to_string(),
                metric.compute(&actuals, &predictions),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::model::EmptyModelCatalog;
    use crate::core::utils::progress::NoopProgressTracker;
    use crate::types::random::RandomGraphConfig;

    #[test]
    #[ignore]
    fn test_create_train_algorithm() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));
        let pipeline = NodeRegressionTrainingPipeline::new();
        let train_config = NodeRegressionPipelineTrainConfig::default();
        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), train_config.clone());
        let progress_tracker = Box::new(NoopProgressTracker);

        let _trainer = NodeRegressionTrain::create(
            graph_store,
            pipeline,
            train_config,
            node_feature_producer,
            progress_tracker,
        );
    }

    #[test]
    fn test_progress_task() {
        let pipeline = NodeRegressionTrainingPipeline::new();
        let node_count = 1000;

        let _task = NodeRegressionTrain::progress_task(&pipeline, node_count);
    }

    #[test]
    fn test_estimate() {
        let pipeline = NodeRegressionTrainingPipeline::new();
        let config = NodeRegressionPipelineTrainConfig::default();
        let model_catalog = EmptyModelCatalog;
        let graph_store = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig::default())
                .expect("random graph generation"),
        );
        let algorithms_facade = AlgorithmsProcedureFacade::from_store(graph_store);

        let _est =
            NodeRegressionTrain::estimate(&pipeline, &config, &model_catalog, &algorithms_facade);
    }

    #[test]
    #[should_panic(expected = "Missing target node property for regression")]
    fn test_run() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));
        let pipeline = NodeRegressionTrainingPipeline::new();
        let config = NodeRegressionPipelineTrainConfig::default();
        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), config.clone());
        let progress_tracker = Box::new(NoopProgressTracker);

        let mut trainer = NodeRegressionTrain::create(
            graph_store,
            pipeline,
            config,
            node_feature_producer,
            progress_tracker,
        );

        let _result = trainer.run();
    }
}
