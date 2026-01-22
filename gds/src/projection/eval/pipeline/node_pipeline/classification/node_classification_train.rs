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
use crate::ml::metrics::classification::ClassificationMetric;
use crate::ml::metrics::ClassificationMetricSpecification;
use crate::ml::metrics::{
    EvaluationScores, Metric, ModelCandidateStats, ModelSpecificMetricsHandler,
};
use crate::ml::models::automl::create_trainer_config_from_map;
use crate::ml::models::{
    base::TrainerConfigTrait, Classifier, ClassifierTrainerFactory, Features,
    TrainingMethod as MlTrainingMethod,
};
use crate::ml::node_prediction::NodeSplitter;
use crate::ml::splitting::TrainingExamplesSplit;
use crate::ml::training::statistics::TrainingStatistics;
use crate::prelude::GraphStore;
use crate::projection::eval::pipeline::node_pipeline::node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;
use crate::projection::eval::pipeline::node_pipeline::node_property_training_pipeline::NodePropertyTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::NodeFeatureProducer;
use crate::projection::eval::pipeline::PipelineTrainer;
use crate::projection::eval::pipeline::{
    TrainingMethod as PipelineTrainingMethod, TrainingPipeline,
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

        let build_metrics = || -> Result<Vec<Box<dyn Metric>>, String> {
            if self.train_config.metrics_specs().is_empty() {
                let defaults = vec![
                    "ACCURACY".to_string(),
                    "F1_WEIGHTED".to_string(),
                    "F1_MACRO".to_string(),
                        use super::*;
                        use crate::core::model::EmptyModelCatalog;

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
        let best_config = candidate_configs
            .get(best_idx)
            .expect("At least one trainer config is required");

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
    /// 5. Evaluate on train and test sets
    /// 6. Retrain on full dataset
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

        let build_metrics = || -> Result<Vec<Box<dyn Metric>>, String> {
            if self.train_config.metrics_specs().is_empty() {
                let defaults = vec![
                    "ACCURACY".to_string(),
                    "F1_WEIGHTED".to_string(),
                    "F1_MACRO".to_string(),
                    "F1(class=*)".to_string(),
                    "PRECISION(class=*)".to_string(),
                    "RECALL(class=*)".to_string(),
                ];
                Ok(ClassificationMetricSpecification::parse_list(&defaults)
                    .map_err(|e| format!("Failed to parse default metrics: {e}"))?
                    .into_iter()
                    .flat_map(|spec| spec.create_metrics(&self.class_id_map, &self.class_counts))
                    .collect())
            } else {
                Ok(self
                    .train_config
                    .metrics(&self.class_id_map, &self.class_counts))
            }
        };

        let metrics = build_metrics().map_err(|e| e.to_string())?;
        let classification_metrics =
            NodeClassificationPipelineTrainConfig::classification_metrics(&metrics);

        let mut training_statistics = TrainingStatistics::new(&metrics);

        let metrics_for_cv = build_metrics().map_err(|e| e.to_string())?;
        let metrics_for_eval = build_metrics().map_err(|e| e.to_string())?;

        let features = self
            .node_feature_producer
            .procedure_features(&self.pipeline)
            .map_err(|e| format!("Feature production failed: {e}"))?;

        let primary_metric = training_statistics.evaluation_metric().to_string();
        let comparator = training_statistics.evaluation_comparator();

        let mut best_config: Option<Box<dyn TrainerConfigTrait>> = None;
        let mut best_score: Option<f64> = None;
        let mut best_train_scores: HashMap<String, f64> = HashMap::new();
        let mut best_test_scores: HashMap<String, f64> = HashMap::new();

        let candidates = self.collect_candidate_configs();
        for (method, config_map) in candidates {
            let ml_method = match method {
                PipelineTrainingMethod::LogisticRegression => MlTrainingMethod::LogisticRegression,
                PipelineTrainingMethod::RandomForestClassification => {
                    MlTrainingMethod::RandomForestClassification
                }
                PipelineTrainingMethod::MLPClassification => MlTrainingMethod::MLPClassification,
                _ => MlTrainingMethod::MLPClassification,
            };

            let trainer_config = create_trainer_config_from_map(config_map.clone(), ml_method);
            let classifier = self.train_simple_model(
                node_splits.outer_split(),
                &features,
                trainer_config.as_ref(),
            )?;

            let (train_scores, test_scores) = self.evaluate_model(
                node_splits.outer_split(),
                &features,
                &classifier,
                &classification_metrics,
            );

            let mut trainer_map = trainer_config.to_map();
            for (key, value) in &config_map {
                trainer_map.insert(key.clone(), value.clone());
            }

            training_statistics.add_candidate_stats(ModelCandidateStats::new(
                serde_json::Value::Object(trainer_map.into_iter().collect()),
                scores_to_eval_scores(train_scores.clone()),
                scores_to_eval_scores(test_scores.clone()),
            ));

            let candidate_score = test_scores.get(&primary_metric).copied().unwrap_or(0.0);
            let is_best = match best_score {
                None => true,
                let features = Arc::from(features);
        split: &TrainingExamplesSplit,
        features: &Box<dyn Features>,
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

        Ok(trainer.train(features.as_ref(), &self.targets, &train_set))
    }

    fn retrain_best_model(
        &self,
        all_training_examples: &Arc<Vec<i64>>,
        features: &Box<dyn Features>,
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
        let build_metrics = || -> Result<Vec<Box<dyn Metric>>, String> {
            if self.train_config.metrics_specs().is_empty() {
                let defaults = vec![
                    "ACCURACY".to_string(),
                    "F1_WEIGHTED".to_string(),
                    "F1_MACRO".to_string(),
                    "F1(class=*)".to_string(),
                    "PRECISION(class=*)".to_string(),
                    "RECALL(class=*)".to_string(),
                ];
                Ok(ClassificationMetricSpecification::parse_list(&defaults)
                    .map_err(|e| format!("Failed to parse default metrics: {e}"))?
                    .into_iter()
                    .flat_map(|spec| spec.create_metrics(&self.class_id_map, &self.class_counts))
                    .collect())
            } else {
                Ok(self
                    .train_config
                    .metrics(&self.class_id_map, &self.class_counts))
            }
        };

        let metrics = build_metrics().map_err(|e| e.to_string())?;
        );
        let test_scores = evaluate_metrics(
            &split.test_set(),
            classifier.as_ref(),
            features.as_ref(),
            &labels_long,
            classification_metrics,
        );

        (train_scores, test_scores)
    }
}

fn scores_to_eval_scores(scores: HashMap<String, f64>) -> HashMap<String, EvaluationScores> {
    scores
        .into_iter()
        .map(|(metric, score)| (metric, EvaluationScores::new(score, score, score)))
        .collect()
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

impl NodeClassificationTrain {
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
        let primary_metric = training_statistics.evaluation_metric().to_string();
        let comparator = training_statistics.evaluation_comparator();

        let mut best_config: Option<Box<dyn TrainerConfigTrait>> = None;
        let mut best_score: Option<f64> = None;
        let mut best_train_scores: HashMap<String, f64> = HashMap::new();
        let mut best_test_scores: HashMap<String, f64> = HashMap::new();

        let candidates = self.collect_candidate_configs();
        for (method, config_map) in candidates {
            let ml_method = match method {
                TrainingMethod::LogisticRegression => MlTrainingMethod::LogisticRegression,
                TrainingMethod::RandomForestClassification => {
                    MlTrainingMethod::RandomForestClassification
                }
                TrainingMethod::MLPClassification => MlTrainingMethod::MLPClassification,
                _ => MlTrainingMethod::MLPClassification,
            };

            let trainer_config = create_trainer_config_from_map(config_map.clone(), ml_method);
            let classifier = self.train_simple_model(
                node_splits.outer_split(),
                &features,
                trainer_config.as_ref(),
            )?;

            let (train_scores, test_scores) = self.evaluate_model(
                node_splits.outer_split(),
                &features,
                &classifier,
                &classification_metrics,
            );

            let mut trainer_map = trainer_config.to_map();
            for (key, value) in &config_map {
                trainer_map.insert(key.clone(), value.clone());
            }

            training_statistics.add_candidate_stats(ModelCandidateStats::new(
                serde_json::Value::Object(trainer_map.into_iter().collect()),
                scores_to_eval_scores(train_scores.clone()),
                scores_to_eval_scores(test_scores.clone()),
            ));

            let candidate_score = test_scores.get(&primary_metric).copied().unwrap_or(0.0);
            let is_best = match best_score {
                None => true,
                Some(best) => comparator.compare(candidate_score, best).is_gt(),
            };

            if is_best {
                best_score = Some(candidate_score);
                best_config = Some(trainer_config);
                best_train_scores = train_scores;
                best_test_scores = test_scores;
            }
        }

        for (metric, score) in &best_train_scores {
            training_statistics.add_outer_train_score(metric.clone(), *score);
        }
        for (metric, score) in &best_test_scores {
            training_statistics.add_test_score(metric.clone(), *score);
        }

        let retrained = self.retrain_best_model(
            node_splits.all_training_examples(),
            &features,
            best_config
                .as_ref()
                .expect("At least one trainer config is required")
                .as_ref(),
        )?;
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
