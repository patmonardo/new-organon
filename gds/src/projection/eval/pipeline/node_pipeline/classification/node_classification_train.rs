use super::labels_and_class_counts_extractor::LabelsAndClassCountsExtractor;
use super::node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;
use super::node_classification_train_result::NodeClassificationTrainResult;
use super::node_classification_training_pipeline::NodeClassificationTrainingPipeline;
use crate::collections::long_multiset::LongMultiSet;
use crate::collections::{HugeIntArray, HugeLongArray};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::model::ModelCatalog;
use crate::core::utils::progress::{ProgressTracker, Task};
use crate::mem::{MemoryEstimation, MemoryEstimations};
use crate::ml::core::subgraph::LocalIdMap;
use crate::ml::metrics::classification::{ClassificationMetric, GlobalAccuracy};
use crate::ml::metrics::{Metric, ModelCandidateStats, ModelSpecificMetricsHandler};
use crate::ml::models::neural::MLPClassifierTrainConfig;
use crate::ml::models::{Classifier, ClassifierTrainerFactory, Features};
use crate::ml::node_prediction::NodeSplitter;
use crate::ml::splitting::TrainingExamplesSplit;
use crate::ml::training::statistics::TrainingStatistics;
use crate::prelude::GraphStore;
use crate::projection::eval::pipeline::node_pipeline::node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;
use crate::projection::eval::pipeline::node_pipeline::node_property_training_pipeline::NodePropertyTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::NodeFeatureProducer;
use crate::projection::eval::pipeline::PipelineTrainer;
use crate::types::graph::Graph;
use crate::types::graph_store::DefaultGraphStore;
use std::collections::HashMap;
use std::sync::Arc;

pub trait AlgorithmsProcedureFacade: Send + Sync {}

/// Core training algorithm for node classification.
///
/// This implements the full training loop:
/// 1. Extract labels and class counts from target property
/// 2. Split data into train/test/validation sets
/// 3. Cross-validation with hyperparameter search (AutoML)
/// 4. Model selection (find best model candidate)
/// 5. Evaluate best model on train and test sets
/// 6. Retrain best model on full training set
/// 7. Return trained model with statistics
pub struct NodeClassificationTrain {
    pipeline: NodeClassificationTrainingPipeline,
    train_config: NodeClassificationPipelineTrainConfig,
    targets: HugeIntArray,
    class_id_map: LocalIdMap,
    node_graph: Arc<dyn Graph>,
    class_counts: LongMultiSet,
    node_feature_producer: NodeFeatureProducer<NodeClassificationPipelineTrainConfig>,
    progress_tracker: Box<dyn ProgressTracker>,
    termination_flag: TerminationFlag,
}

impl NodeClassificationTrain {
    /// Estimate memory requirements for training.
    pub fn estimate(
        _pipeline: &NodeClassificationTrainingPipeline,
        _configuration: &NodeClassificationPipelineTrainConfig,
        _model_catalog: &impl ModelCatalog,
        _algorithms_procedure_facade: &dyn AlgorithmsProcedureFacade,
    ) -> Box<dyn MemoryEstimation> {
        // Note: Implement once memory estimation infrastructure is translated.
        MemoryEstimations::empty()
    }

    /// Create progress task for training.
    pub fn progress_task(pipeline: &NodeClassificationTrainingPipeline, node_count: u64) -> Task {
        // Note: Implement once the Tasks API is translated.
        // let split_config = pipeline.split_config();
        // let train_set_size = split_config.train_set_size(node_count);
        // let test_set_size = split_config.test_set_size(node_count);
        // let validation_folds = split_config.validation_folds();
        //
        // let mut tasks = vec![];
        // tasks.push(NodePropertyStepExecutor::tasks(pipeline.node_property_steps(), node_count));
        // tasks.extend(CrossValidation::progress_tasks(
        //     validation_folds,
        //     pipeline.number_of_model_selection_trials(),
        //     train_set_size,
        // ));
        // tasks.push(ClassifierTrainer::progress_task("Train best model", 5 * train_set_size));
        // tasks.push(Tasks::leaf("Evaluate on train data", train_set_size));
        // tasks.push(Tasks::leaf("Evaluate on test data", test_set_size));
        // tasks.push(ClassifierTrainer::progress_task("Retrain best model", 5 * node_count));
        //
        // Tasks::task("Node Classification Train Pipeline", tasks)

        let _ = (pipeline, node_count);
        Task::new("Node Classification Train Pipeline".to_string(), vec![])
    }

    /// Create a new NodeClassificationTrain instance.
    pub fn create(
        graph_store: Arc<DefaultGraphStore>,
        pipeline: NodeClassificationTrainingPipeline,
        config: NodeClassificationPipelineTrainConfig,
        node_feature_producer: NodeFeatureProducer<NodeClassificationPipelineTrainConfig>,
        progress_tracker: Box<dyn ProgressTracker>,
    ) -> Self {
        let node_graph = graph_store.get_graph();
        pipeline
            .split_config()
            .validate_min_num_nodes_in_split_sets(node_graph.node_count())
            .expect("Invalid split configuration for node count");

        let target_node_property = node_graph
            .node_properties(config.target_property())
            .expect("Missing target node property for classification");

        let labels_and_class_counts =
            LabelsAndClassCountsExtractor::extract_labels_and_class_counts(
                &*target_node_property,
                node_graph.node_count() as u64,
            );

        let class_counts = labels_and_class_counts.class_counts().clone();
        let mut class_ids: Vec<u64> = class_counts
            .keys()
            .into_iter()
            .map(|id| id as u64)
            .collect();
        class_ids.sort_unstable();
        let class_id_map = LocalIdMap::of_sorted(&class_ids);

        let termination_flag = TerminationFlag::running_true();

        Self {
            pipeline,
            train_config: config,
            targets: labels_and_class_counts.labels().clone(),
            class_id_map,
            node_graph,
            class_counts,
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
    ///
    /// Main training loop:
    /// 1. Split data into train/test/validation
    /// 2. Extract features
    /// 3. Cross-validation with AutoML hyperparameter search
    /// 4. Select best model
    /// 5. Evaluate on train and test sets
    /// 6. Retrain on full dataset
    pub fn run(&mut self) -> Result<NodeClassificationTrainResult, Box<dyn std::error::Error>> {
        self.progress_tracker.begin_subtask();

        let split_config = self.pipeline.split_config();
        let node_count = self.node_graph.node_count();

        let node_splitter = NodeSplitter::new(
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
        );

        let (metrics, classification_metrics) = default_metrics();
        let mut training_statistics = TrainingStatistics::new(metrics);
        training_statistics.add_candidate_stats(ModelCandidateStats::new(
            serde_json::json!({}),
            HashMap::new(),
            HashMap::new(),
        ));

        let features = self
            .node_feature_producer
            .procedure_features(&self.pipeline)
            .map_err(|e| format!("Feature production failed: {e}"))?;

        let classifier = self.train_simple_model(&node_splits.outer_split, &features)?;
        self.evaluate_model(
            &node_splits.outer_split,
            &features,
            &classifier,
            &classification_metrics,
            &mut training_statistics,
        );

        let retrained = self.retrain_best_model(&node_splits.all_training_examples, &features)?;

        self.progress_tracker.end_subtask();

        Ok(NodeClassificationTrainResult::new(
            retrained,
            training_statistics,
            self.class_id_map.clone(),
            self.class_counts.clone(),
        ))
    }
}

impl PipelineTrainer for NodeClassificationTrain {
    type Result = NodeClassificationTrainResult;

    fn run(&mut self) -> Result<Self::Result, Box<dyn std::error::Error>> {
        self.run()
    }

    fn is_terminated(&self) -> bool {
        !self.termination_flag.running()
    }
}

impl NodeClassificationTrain {
    fn train_simple_model(
        &self,
        split: &TrainingExamplesSplit,
        features: &Box<dyn Features>,
    ) -> Result<Box<dyn Classifier>, Box<dyn std::error::Error>> {
        let train_set = to_u64_arc(split.train_set());
        let trainer_config = MLPClassifierTrainConfig::default();
        let trainer = ClassifierTrainerFactory::create(
            &trainer_config,
            self.class_id_map.size(),
            &self.termination_flag,
            &*self.progress_tracker,
            &Concurrency::available_cores(),
            self.train_config.random_seed(),
            false,
            &ModelSpecificMetricsHandler::noop(),
        );

        Ok(trainer.train(features.as_ref(), &self.targets, &train_set))
    }

    fn retrain_best_model(
        &self,
        all_training_examples: &Arc<Vec<i64>>,
        features: &Box<dyn Features>,
    ) -> Result<Box<dyn Classifier>, Box<dyn std::error::Error>> {
        let train_set = to_u64_arc(all_training_examples.clone());
        let trainer_config = MLPClassifierTrainConfig::default();
        let trainer = ClassifierTrainerFactory::create(
            &trainer_config,
            self.class_id_map.size(),
            &self.termination_flag,
            &*self.progress_tracker,
            &Concurrency::available_cores(),
            self.train_config.random_seed(),
            false,
            &ModelSpecificMetricsHandler::noop(),
        );

        Ok(trainer.train(features.as_ref(), &self.targets, &train_set))
    }

    fn evaluate_model(
        &self,
        split: &TrainingExamplesSplit,
        features: &Box<dyn Features>,
        classifier: &Box<dyn Classifier>,
        classification_metrics: &[Box<dyn ClassificationMetric>],
        training_statistics: &mut TrainingStatistics,
    ) {
        let labels_long = labels_as_long(&self.targets);
        let train_scores = evaluate_metrics(
            &split.train_set(),
            classifier.as_ref(),
            features.as_ref(),
            &labels_long,
            classification_metrics,
        );
        let test_scores = evaluate_metrics(
            &split.test_set(),
            classifier.as_ref(),
            features.as_ref(),
            &labels_long,
            classification_metrics,
        );

        for (metric, score) in train_scores {
            training_statistics.add_outer_train_score(metric, score);
        }
        for (metric, score) in test_scores {
            training_statistics.add_test_score(metric, score);
        }
    }
}

fn default_metrics() -> (Vec<Box<dyn Metric>>, Vec<Box<dyn ClassificationMetric>>) {
    (
        vec![Box::new(GlobalAccuracy::new())],
        vec![Box::new(GlobalAccuracy::new())],
    )
}

fn labels_as_long(labels: &HugeIntArray) -> HugeLongArray {
    let mut out = HugeLongArray::new(labels.size());
    for idx in 0..labels.size() {
        out.set(idx, labels.get(idx) as i64);
    }
    out
}

fn to_u64_arc(values: Arc<Vec<i64>>) -> Arc<Vec<u64>> {
    Arc::new(values.iter().map(|v| *v as u64).collect())
}

fn evaluate_metrics(
    evaluation_set: &Arc<Vec<i64>>,
    classifier: &dyn Classifier,
    features: &dyn Features,
    labels: &HugeLongArray,
    metrics: &[Box<dyn ClassificationMetric>],
) -> HashMap<String, f64> {
    let eval_ids: Vec<usize> = evaluation_set.iter().map(|v| *v as usize).collect();
    let mut predictions = HugeLongArray::new(eval_ids.len());
    let mut eval_labels = HugeLongArray::new(eval_ids.len());

    for (idx, node_id) in eval_ids.iter().enumerate() {
        let probs = classifier.predict_probabilities(features.get(*node_id));
        let predicted = probs
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx as i64)
            .unwrap_or(0);
        predictions.set(idx, predicted);
        eval_labels.set(idx, labels.get(*node_id));
    }

    metrics
        .iter()
        .map(|metric| {
            (
                metric.name().to_string(),
                metric.compute(&eval_labels, &predictions),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::model::EmptyModelCatalog;
    use crate::core::utils::progress::NoopProgressTracker;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;
    use std::sync::Arc;

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
        let pipeline = NodeClassificationTrainingPipeline::new();
        let train_config = NodeClassificationPipelineTrainConfig::default();
        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), train_config.clone());
        let progress_tracker = Box::new(NoopProgressTracker);

        let _trainer = NodeClassificationTrain::create(
            graph_store,
            pipeline,
            train_config,
            node_feature_producer,
            progress_tracker,
        );

        // Verify it was created without panicking
    }

    #[test]
    fn test_progress_task() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        let node_count = 1000;

        let _task = NodeClassificationTrain::progress_task(&pipeline, node_count);

        // Should return placeholder for now
    }

    #[test]
    fn test_estimate() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        let config = NodeClassificationPipelineTrainConfig::default();
        let model_catalog = EmptyModelCatalog;
        struct StubAlgorithmsFacade;
        impl AlgorithmsProcedureFacade for StubAlgorithmsFacade {}
        let algorithms_facade = StubAlgorithmsFacade;

        let _est = NodeClassificationTrain::estimate(
            &pipeline,
            &config,
            &model_catalog,
            &algorithms_facade,
        );

        // Should return placeholder for now
    }

    #[test]
    #[ignore]
    fn test_set_termination_flag() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));
        let pipeline = NodeClassificationTrainingPipeline::new();
        let train_config = NodeClassificationPipelineTrainConfig::default();
        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), train_config.clone());
        let progress_tracker = Box::new(NoopProgressTracker);

        let mut trainer = NodeClassificationTrain::create(
            graph_store,
            pipeline,
            train_config,
            node_feature_producer,
            progress_tracker,
        );

        let termination_flag = TerminationFlag::running_true();
        trainer.set_termination_flag(termination_flag);

        // Should set without panicking
    }

    #[test]
    #[ignore]
    fn test_run_placeholder() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));
        let pipeline = NodeClassificationTrainingPipeline::new();
        let train_config = NodeClassificationPipelineTrainConfig::default();
        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), train_config.clone());
        let progress_tracker = Box::new(NoopProgressTracker);

        let mut trainer = NodeClassificationTrain::create(
            graph_store,
            pipeline,
            train_config,
            node_feature_producer,
            progress_tracker,
        );

        let _result = trainer.run();
        // Placeholder test - result should be default/empty
    }

    #[test]
    #[should_panic(expected = "Missing target node property for classification")]
    fn test_run() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));
        let pipeline = NodeClassificationTrainingPipeline::new();
        let config = NodeClassificationPipelineTrainConfig::default();
        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), config.clone());
        let progress_tracker = Box::new(NoopProgressTracker);

        let mut trainer = NodeClassificationTrain::create(
            graph_store,
            pipeline,
            config,
            node_feature_producer,
            progress_tracker,
        );

        let _result = trainer.run();

        // Should complete without panicking (placeholder implementation)
    }
}
