use super::labels_and_class_counts_extractor::LabelsAndClassCountsExtractor;
use super::node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;
use super::node_classification_train_result::NodeClassificationTrainResult;
use super::node_classification_training_pipeline::NodeClassificationTrainingPipeline;
use crate::collections::long_multiset::LongMultiSet;
use crate::collections::{HugeIntArray, HugeLongArray};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::model::ModelCatalog;
use crate::core::utils::progress::{NoopProgressTracker, ProgressTracker, Task};
use crate::mem::{MemoryEstimation, MemoryEstimations};
use crate::ml::core::subgraph::LocalIdMap;
use crate::ml::metrics::classification::ClassificationMetric;
use crate::ml::metrics::ClassificationMetricSpecification;
use crate::ml::metrics::{Metric, ModelSpecificMetricsHandler};
use crate::ml::models::automl::create_trainer_config_from_map;
use crate::ml::models::{
    base::TrainerConfigTrait, Classifier, ClassifierTrainerFactory, Features,
    TrainingMethod as MlTrainingMethod,
};
use crate::ml::node_prediction::NodeSplitter;
use crate::ml::splitting::TrainingExamplesSplit;
use crate::ml::training::{CrossValidation, TrainingStatistics};
use crate::procedures::AlgorithmsProcedureFacade;
use crate::projection::eval::pipeline::node_pipeline::node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;
use crate::projection::eval::pipeline::node_pipeline::node_property_training_pipeline::NodePropertyTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::NodeFeatureProducer;
use crate::projection::eval::pipeline::PipelineTrainer;
use crate::projection::eval::pipeline::{
    TrainingMethod as PipelineTrainingMethod, TrainingPipeline,
};
use crate::types::graph::Graph;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::prelude::GraphStore;
use parking_lot::RwLock;
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

/// Core training algorithm for node classification.
///
/// This implements the training loop:
/// 1. Extract target labels from the target property
/// 2. Split data into train/test/validation sets
/// 3. Select model with cross-validation
/// 4. Evaluate on train/test sets
/// 5. Retrain on full training set
pub struct NodeClassificationTrain {
    pipeline: NodeClassificationTrainingPipeline,
    train_config: NodeClassificationPipelineTrainConfig,
    targets: HugeIntArray,
    class_id_map: LocalIdMap,
    class_counts: LongMultiSet,
    node_graph: Arc<dyn Graph>,
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
        _algorithms_procedure_facade: &AlgorithmsProcedureFacade,
    ) -> Box<dyn MemoryEstimation> {
        // Note: Implement once memory estimation infrastructure is translated.
        MemoryEstimations::empty()
    }

    /// Create progress task for training.
    pub fn progress_task(_pipeline: &NodeClassificationTrainingPipeline, _node_count: u64) -> Task {
        // Note: Implement once the Tasks API is translated.
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

        let labels_and_counts = LabelsAndClassCountsExtractor::extract_labels_and_class_counts(
            target_node_property.as_ref(),
            node_graph.node_count() as u64,
        );

        let class_counts = labels_and_counts.class_counts().clone();
        let targets = labels_and_counts.labels().clone();

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
            targets,
            class_id_map,
            class_counts,
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
    ) -> Result<NodeClassificationTrainResult, Box<dyn std::error::Error + Send + Sync>> {
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

        let metrics = self.build_metrics()?;
        let classification_metrics =
            NodeClassificationPipelineTrainConfig::classification_metrics(&metrics);
        let mut training_statistics = TrainingStatistics::new(&metrics);

        let metrics_for_cv = self.build_metrics()?;
        let metrics_for_cv_eval = Arc::new(self.build_metrics()?);

        let features = self
            .node_feature_producer
            .procedure_features(&self.pipeline)
            .map_err(|e| format!("Feature production failed: {e}"))?;
        let features: Arc<dyn Features> = Arc::from(features);

        let labels_long = Arc::new(labels_as_long(&self.targets));
        let labels_vec = Arc::new(labels_long.iter().collect::<Vec<_>>());
        let distinct_targets: BTreeSet<i64> =
            (0..self.class_id_map.size()).map(|v| v as i64).collect();

        let candidates = self.collect_candidate_configs();
        let candidate_configs_for_cv: Vec<Box<dyn TrainerConfigTrait>> = candidates
            .iter()
            .map(|(method, config_map)| {
                create_trainer_config_from_map(config_map.clone(), map_training_method(*method))
            })
            .collect();

        let cv_termination_flag = Arc::new(RwLock::new(false));
        let random_seed = self.train_config.random_seed();
        let termination_flag = self.termination_flag.clone();
        let targets = Arc::new(self.targets.clone());
        let class_count = self.class_id_map.size();

        let cv = CrossValidation::new(
            cv_termination_flag,
            metrics_for_cv,
            split_config.validation_folds(),
            random_seed,
            Box::new({
                let features = Arc::clone(&features);
                let targets = Arc::clone(&targets);
                move |train_set, trainer_config, metrics_handler, _name| {
                    let train_set = to_u64_arc(train_set);
                    let progress_tracker = NoopProgressTracker;
                    let trainer = ClassifierTrainerFactory::create(
                        trainer_config,
                        class_count,
                        &termination_flag,
                        &progress_tracker,
                        &Concurrency::available_cores(),
                        random_seed,
                        false,
                        metrics_handler,
                    );

                    trainer.train(features.as_ref(), &targets, &train_set)
                }
            }),
            Box::new({
                let features = Arc::clone(&features);
                let labels_long = Arc::clone(&labels_long);
                let metrics_for_cv_eval = Arc::clone(&metrics_for_cv_eval);
                move |eval_set, model, consumer| {
                    let classification_metrics =
                        NodeClassificationPipelineTrainConfig::classification_metrics(
                            metrics_for_cv_eval.as_ref(),
                        );
                    let scores = evaluate_metrics(
                        &eval_set,
                        model.as_ref(),
                        features.as_ref(),
                        labels_long.as_ref(),
                        &classification_metrics,
                    );

                    for metric in classification_metrics {
                        let value = scores.get(metric.name()).copied().unwrap_or(0.0);
                        consumer.consume(metric as &dyn Metric, value);
                    }
                }
            }),
        );

        let outer_train = node_splits.outer_split().train_set();
        cv.select_model(
            outer_train,
            {
                let labels_vec = Arc::clone(&labels_vec);
                move |node_id| labels_vec[node_id as usize]
            },
            distinct_targets,
            &mut training_statistics,
            candidate_configs_for_cv.into_iter(),
        );

        let best_idx = training_statistics.best_trial_idx();
        let (best_method, best_map) = candidates
            .get(best_idx)
            .expect("At least one trainer config is required");
        let best_config =
            create_trainer_config_from_map(best_map.clone(), map_training_method(*best_method));

        let classifier = self.train_simple_model(
            node_splits.outer_split(),
            features.as_ref(),
            best_config.as_ref(),
        )?;
        let (train_scores, test_scores) = self.evaluate_model(
            node_splits.outer_split(),
            features.as_ref(),
            &classifier,
            &classification_metrics,
        );

        for (metric, score) in &train_scores {
            training_statistics.add_outer_train_score(metric.clone(), *score);
        }
        for (metric, score) in &test_scores {
            training_statistics.add_test_score(metric.clone(), *score);
        }

        let retrained = self.retrain_best_model(
            node_splits.all_training_examples(),
            features.as_ref(),
            best_config.as_ref(),
        )?;

        self.progress_tracker.end_subtask();

        Ok(NodeClassificationTrainResult::new(
            retrained,
            training_statistics,
            self.class_id_map.clone(),
            self.class_counts.clone(),
        ))
    }

    fn build_metrics(&self) -> Result<Vec<Box<dyn Metric>>, String> {
        if self.train_config.metrics_specs().is_empty() {
            let defaults = vec![
                "ACCURACY".to_string(),
                "F1_WEIGHTED".to_string(),
                "F1_MACRO".to_string(),
                "F1(class=*)".to_string(),
                "PRECISION(class=*)".to_string(),
                "RECALL(class=*)".to_string(),
            ];

            let specs = ClassificationMetricSpecification::parse_list(&defaults)?;
            Ok(specs
                .iter()
                .flat_map(|spec| spec.create_metrics(&self.class_id_map, &self.class_counts))
                .collect())
        } else {
            Ok(self
                .train_config
                .metrics(&self.class_id_map, &self.class_counts))
        }
    }
}

impl PipelineTrainer for NodeClassificationTrain {
    type Result = NodeClassificationTrainResult;

    fn run(&mut self) -> Result<Self::Result, Box<dyn std::error::Error + Send + Sync>> {
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
        features: &dyn Features,
        trainer_config: &dyn TrainerConfigTrait,
    ) -> Result<Box<dyn Classifier>, Box<dyn std::error::Error + Send + Sync>> {
        let train_set = to_u64_arc(split.train_set());
        let trainer = ClassifierTrainerFactory::create(
            trainer_config,
            self.class_id_map.size(),
            &self.termination_flag,
            &*self.progress_tracker,
            &Concurrency::available_cores(),
            self.train_config.random_seed(),
            false,
            &ModelSpecificMetricsHandler::noop(),
        );

        Ok(trainer.train(features, &self.targets, &train_set))
    }

    fn retrain_best_model(
        &self,
        all_training_examples: &Arc<Vec<i64>>,
        features: &dyn Features,
        trainer_config: &dyn TrainerConfigTrait,
    ) -> Result<Box<dyn Classifier>, Box<dyn std::error::Error + Send + Sync>> {
        let train_set = to_u64_arc(all_training_examples.clone());
        let trainer = ClassifierTrainerFactory::create(
            trainer_config,
            self.class_id_map.size(),
            &self.termination_flag,
            &*self.progress_tracker,
            &Concurrency::available_cores(),
            self.train_config.random_seed(),
            false,
            &ModelSpecificMetricsHandler::noop(),
        );

        Ok(trainer.train(features, &self.targets, &train_set))
    }

    fn evaluate_model(
        &self,
        split: &TrainingExamplesSplit,
        features: &dyn Features,
        classifier: &Box<dyn Classifier>,
        classification_metrics: &[&dyn ClassificationMetric],
    ) -> (HashMap<String, f64>, HashMap<String, f64>) {
        let labels_long = labels_as_long(&self.targets);
        let train_scores = evaluate_metrics(
            &split.train_set(),
            classifier.as_ref(),
            features,
            &labels_long,
            classification_metrics,
        );
        let test_scores = evaluate_metrics(
            &split.test_set(),
            classifier.as_ref(),
            features,
            &labels_long,
            classification_metrics,
        );

        (train_scores, test_scores)
    }

    fn collect_candidate_configs(
        &self,
    ) -> Vec<(PipelineTrainingMethod, HashMap<String, serde_json::Value>)> {
        let mut candidates: Vec<(PipelineTrainingMethod, HashMap<String, serde_json::Value>)> =
            Vec::new();

        for (method, configs) in self.pipeline.training_parameter_space() {
            if configs.is_empty() {
                candidates.push((*method, HashMap::<String, serde_json::Value>::new()));
                continue;
            }

            for cfg in configs {
                candidates.push((*method, cfg.to_map()));
            }
        }

        if candidates.is_empty() {
            candidates.push((
                PipelineTrainingMethod::MLPClassification,
                HashMap::<String, serde_json::Value>::new(),
            ));
        }

        let max_trials = self.pipeline.auto_tuning_config().max_trials();
        if max_trials > 0 && candidates.len() > max_trials {
            candidates.truncate(max_trials);
        }

        candidates
    }
}

fn map_training_method(method: PipelineTrainingMethod) -> MlTrainingMethod {
    match method {
        PipelineTrainingMethod::LogisticRegression => MlTrainingMethod::LogisticRegression,
        PipelineTrainingMethod::RandomForestClassification => {
            MlTrainingMethod::RandomForestClassification
        }
        PipelineTrainingMethod::MLPClassification => MlTrainingMethod::MLPClassification,
        other => panic!("Unsupported training method for classification: {other:?}"),
    }
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
    metrics: &[&dyn ClassificationMetric],
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
    }

    #[test]
    fn test_progress_task() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        let node_count = 1000;

        let _task = NodeClassificationTrain::progress_task(&pipeline, node_count);
    }

    #[test]
    fn test_estimate() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        let config = NodeClassificationPipelineTrainConfig::default();
        let model_catalog = EmptyModelCatalog;
        let graph_store = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig::default())
                .expect("random graph generation"),
        );
        let algorithms_facade = AlgorithmsProcedureFacade::from_store(graph_store);

        let _est = NodeClassificationTrain::estimate(
            &pipeline,
            &config,
            &model_catalog,
            &algorithms_facade,
        );
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
    }
}
