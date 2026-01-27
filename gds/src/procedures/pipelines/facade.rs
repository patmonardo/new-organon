//! Trait surface for the Pipelines Procedure Facade.
//!
//! This is intentionally "ceremonial" and Java-shaped.

use std::sync::Arc;
use std::sync::OnceLock;

use serde_json::Value;

use crate::mem::MemoryEstimationResult;
use crate::projection::eval::pipeline::link_pipeline::{
    LinkFeatureStepFactory, LinkPredictionSplitConfig, LinkPredictionTrainingPipeline,
};
use crate::projection::eval::pipeline::node_pipeline::{
    NodeClassificationTrainingPipeline, NodeFeatureStep, NodePropertyPredictionSplitConfig,
    NodePropertyTrainingPipeline, NodeRegressionTrainingPipeline,
};
use crate::projection::eval::pipeline::AutoTuningConfig;
use crate::projection::eval::pipeline::ExecutableNodePropertyStep;
use crate::projection::eval::pipeline::FeatureStep;
use crate::projection::eval::pipeline::NodePropertyStep;
use crate::projection::eval::pipeline::Pipeline;
use crate::projection::eval::pipeline::PipelineCatalog;
use crate::projection::eval::pipeline::PipelineCatalogEntry;
use crate::projection::eval::pipeline::TrainingMethod;
use crate::projection::eval::pipeline::TrainingPipeline;
use crate::types::user::User;

use super::types::*;

pub type RawConfig = AnyMap;

pub trait LinkPredictionFacade {
    fn add_feature(
        &self,
        pipeline_name: &str,
        feature_type: &str,
        raw_configuration: RawConfig,
    ) -> Vec<PipelineInfoResult>;

    fn add_logistic_regression(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<PipelineInfoResult>;

    fn add_mlp(&self, pipeline_name: &str, configuration: RawConfig) -> Vec<PipelineInfoResult>;

    fn add_node_property(
        &self,
        pipeline_name: &str,
        task_name: &str,
        procedure_config: RawConfig,
    ) -> Vec<PipelineInfoResult>;

    fn add_random_forest(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<PipelineInfoResult>;

    fn configure_auto_tuning(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<PipelineInfoResult>;

    fn configure_split(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<PipelineInfoResult>;

    fn create_pipeline(&self, pipeline_name: &str) -> Vec<PipelineInfoResult>;

    fn mutate(&self, graph_name: &str, configuration: RawConfig) -> Vec<MutateResult>;

    fn mutate_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;

    fn stream(&self, graph_name: &str, configuration: RawConfig) -> Vec<StreamResult>;

    fn stream_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;

    fn train(&self, graph_name: &str, configuration: RawConfig) -> Vec<LinkPredictionTrainResult>;

    fn train_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;
}

pub trait NodeClassificationFacade {
    fn add_logistic_regression(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn add_mlp(&self, pipeline_name: &str, configuration: RawConfig)
        -> Vec<NodePipelineInfoResult>;

    fn add_node_property(
        &self,
        pipeline_name: &str,
        task_name: &str,
        procedure_config: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn add_random_forest(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn configure_auto_tuning(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn configure_split(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn create_pipeline(&self, pipeline_name: &str) -> Vec<NodePipelineInfoResult>;

    fn mutate(&self, graph_name: &str, configuration: RawConfig) -> Vec<PredictMutateResult>;

    fn mutate_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;

    fn select_features(
        &self,
        pipeline_name: &str,
        node_feature_steps: Value,
    ) -> Vec<NodePipelineInfoResult>;

    fn stream(
        &self,
        graph_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodeClassificationStreamResult>;

    fn stream_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;

    fn train(
        &self,
        graph_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodeClassificationPipelineTrainResult>;

    fn train_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;

    fn write(&self, graph_name: &str, configuration: RawConfig) -> Vec<WriteResult>;

    fn write_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;
}

pub trait NodeRegressionFacade {
    fn add_logistic_regression(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn add_node_property(
        &self,
        pipeline_name: &str,
        task_name: &str,
        procedure_config: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn add_random_forest(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn configure_auto_tuning(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn configure_split(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn create_pipeline(&self, pipeline_name: &str) -> Vec<NodePipelineInfoResult>;

    fn mutate(&self, graph_name: &str, configuration: RawConfig) -> Vec<PredictMutateResult>;

    fn stream(&self, graph_name: &str, configuration: RawConfig)
        -> Vec<NodeRegressionStreamResult>;

    fn select_features(
        &self,
        pipeline_name: &str,
        feature_properties: Value,
    ) -> Vec<NodePipelineInfoResult>;

    fn train(
        &self,
        graph_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodeRegressionPipelineTrainResult>;
}

pub trait PipelinesProcedureFacade {
    const NO_VALUE: &'static str = "__NO_VALUE";

    fn drop(&self, pipeline_name: &str, fail_if_missing: bool) -> Vec<PipelineCatalogResult>;

    fn exists(&self, pipeline_name: &str) -> Vec<PipelineExistsResult>;

    fn list(&self, pipeline_name: &str) -> Vec<PipelineCatalogResult>;

    fn link_prediction(&self) -> &dyn LinkPredictionFacade;

    fn node_classification(&self) -> &dyn NodeClassificationFacade;

    fn node_regression(&self) -> &dyn NodeRegressionFacade;
}

// ---------------------------------------------------------------------------
// Local implementation (minimal: catalog operations are real; subfacades stub)
// ---------------------------------------------------------------------------

/// Request scoped dependencies for pipeline procedures.
pub struct RequestScopedDependencies {
    pub user: User,
}

impl RequestScopedDependencies {
    pub fn new(user: User) -> Self {
        Self { user }
    }
}

pub struct LocalPipelinesProcedureFacade {
    request_scoped_dependencies: RequestScopedDependencies,
    pipeline_catalog: Arc<PipelineCatalog>,
    link_prediction: LocalLinkPredictionFacade,
    node_classification: LocalNodeClassificationFacade,
    node_regression: LocalNodeRegressionFacade,
}

fn shared_in_memory_pipeline_catalog() -> Arc<PipelineCatalog> {
    static CATALOG: OnceLock<Arc<PipelineCatalog>> = OnceLock::new();
    Arc::clone(CATALOG.get_or_init(|| Arc::new(PipelineCatalog::new())))
}

impl Default for LocalPipelinesProcedureFacade {
    fn default() -> Self {
        Self::new(
            RequestScopedDependencies::new(User::from("anonymous")),
            shared_in_memory_pipeline_catalog(),
        )
    }
}

impl LocalPipelinesProcedureFacade {
    pub fn new(
        request_scoped_dependencies: RequestScopedDependencies,
        pipeline_catalog: Arc<PipelineCatalog>,
    ) -> Self {
        let username = request_scoped_dependencies.user.username().to_string();
        Self {
            request_scoped_dependencies,
            pipeline_catalog: Arc::clone(&pipeline_catalog),
            link_prediction: LocalLinkPredictionFacade::new(
                username.clone(),
                Arc::clone(&pipeline_catalog),
            ),
            node_classification: LocalNodeClassificationFacade::new(
                username.clone(),
                Arc::clone(&pipeline_catalog),
            ),
            node_regression: LocalNodeRegressionFacade::new(username, pipeline_catalog),
        }
    }

    fn username(&self) -> &str {
        self.request_scoped_dependencies.user.username()
    }

    fn entry_to_catalog_result(entry: &PipelineCatalogEntry) -> PipelineCatalogResult {
        PipelineCatalogResult {
            pipeline_info: AnyMap::new(),
            pipeline_name: entry.pipeline_name().to_string(),
            pipeline_type: entry.pipeline_type().to_string(),
            creation_time: chrono::Utc::now()
                .with_timezone(&chrono::FixedOffset::east_opt(0).expect("UTC offset")),
        }
    }
}

impl PipelinesProcedureFacade for LocalPipelinesProcedureFacade {
    fn drop(&self, pipeline_name: &str, fail_if_missing: bool) -> Vec<PipelineCatalogResult> {
        match PipelineCatalog::drop(
            self.pipeline_catalog.as_ref(),
            self.username(),
            pipeline_name,
        ) {
            Ok(entry) => vec![Self::entry_to_catalog_result(&entry)],
            Err(err) => {
                if fail_if_missing {
                    panic!("{err}");
                }
                vec![]
            }
        }
    }

    fn exists(&self, pipeline_name: &str) -> Vec<PipelineExistsResult> {
        if !self.pipeline_catalog.exists(self.username(), pipeline_name) {
            return vec![PipelineExistsResult::empty(pipeline_name)];
        }

        let pipeline_type = self
            .pipeline_catalog
            .get(self.username(), pipeline_name)
            .map(|entry| entry.pipeline_type().to_string())
            .unwrap_or_else(|_| "n/a".to_string());

        vec![PipelineExistsResult {
            pipeline_name: pipeline_name.to_string(),
            pipeline_type,
            exists: true,
        }]
    }

    fn list(&self, pipeline_name: &str) -> Vec<PipelineCatalogResult> {
        if pipeline_name == Self::NO_VALUE {
            let mut entries = self.pipeline_catalog.get_all_pipelines(self.username());
            entries.sort_by(|a, b| a.pipeline_name().cmp(b.pipeline_name()));
            return entries.iter().map(Self::entry_to_catalog_result).collect();
        }

        self.pipeline_catalog
            .get(self.username(), pipeline_name)
            .map(|entry| vec![Self::entry_to_catalog_result(&entry)])
            .unwrap_or_default()
    }

    fn link_prediction(&self) -> &dyn LinkPredictionFacade {
        &self.link_prediction
    }

    fn node_classification(&self) -> &dyn NodeClassificationFacade {
        &self.node_classification
    }

    fn node_regression(&self) -> &dyn NodeRegressionFacade {
        &self.node_regression
    }
}

pub struct LocalLinkPredictionFacade {
    username: String,
    pipeline_catalog: Arc<PipelineCatalog>,
}

impl LocalLinkPredictionFacade {
    pub fn new(username: String, pipeline_catalog: Arc<PipelineCatalog>) -> Self {
        Self {
            username,
            pipeline_catalog,
        }
    }

    fn pipeline_info(&self, pipeline_name: &str) -> Vec<PipelineInfoResult> {
        let pipeline = self
            .pipeline_catalog
            .get_typed::<LinkPredictionTrainingPipeline>(&self.username, pipeline_name)
            .unwrap_or_else(|e| panic!("{e}"));

        vec![Self::pipeline_info_from_pipeline(
            pipeline_name,
            pipeline.as_ref(),
        )]
    }

    fn pipeline_info_from_pipeline(
        pipeline_name: &str,
        pipeline: &LinkPredictionTrainingPipeline,
    ) -> PipelineInfoResult {
        let node_property_steps: AnyMapList = pipeline
            .node_property_steps()
            .iter()
            .map(|step| step.to_map())
            .collect();

        let feature_steps: AnyMapList = pipeline
            .feature_steps()
            .iter()
            .map(|step| step.to_map())
            .collect();

        let split_config: AnyMap = pipeline.split_config().to_map();
        let auto_tuning_config: AnyMap = pipeline.auto_tuning_config().to_map();

        let mut parameter_space = serde_json::Map::new();
        for (method, configs) in pipeline.training_parameter_space() {
            let entries: Vec<Value> = configs
                .iter()
                .map(|cfg| {
                    Value::Object(
                        cfg.to_map()
                            .into_iter()
                            .collect::<serde_json::Map<String, Value>>(),
                    )
                })
                .collect();
            parameter_space.insert(method.to_string(), Value::Array(entries));
        }

        PipelineInfoResult {
            name: pipeline_name.to_string(),
            node_property_steps,
            feature_steps,
            split_config,
            auto_tuning_config,
            parameter_space: Value::Object(parameter_space),
        }
    }

    fn parse_split_config(configuration: &RawConfig) -> LinkPredictionSplitConfig {
        let mut builder = LinkPredictionSplitConfig::builder();

        if let Some(v) = configuration
            .get("validationFolds")
            .and_then(|v| v.as_u64())
        {
            builder = builder.validation_folds(v as u32);
        }
        if let Some(v) = configuration.get("testFraction").and_then(|v| v.as_f64()) {
            builder = builder.test_fraction(v);
        }
        if let Some(v) = configuration.get("trainFraction").and_then(|v| v.as_f64()) {
            builder = builder.train_fraction(v);
        }
        if let Some(v) = configuration
            .get("negativeSamplingRatio")
            .and_then(|v| v.as_f64())
        {
            builder = builder.negative_sampling_ratio(v);
        }
        if let Some(v) = configuration
            .get("negativeRelationshipType")
            .and_then(|v| v.as_str())
        {
            builder = builder.negative_relationship_type(v.to_string());
        }

        builder.build().unwrap_or_else(|e| panic!("{e}"))
    }

    fn parse_auto_tuning_config(configuration: &RawConfig) -> AutoTuningConfig {
        let max_trials = configuration
            .get("maxTrials")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(AutoTuningConfig::MAX_TRIALS);

        AutoTuningConfig::new(max_trials).unwrap_or_else(|e| panic!("{e}"))
    }

    fn with_pipeline_update(
        &self,
        pipeline_name: &str,
        update: impl FnOnce(&mut LinkPredictionTrainingPipeline),
    ) -> Vec<PipelineInfoResult> {
        let existing = self
            .pipeline_catalog
            .get_typed::<LinkPredictionTrainingPipeline>(&self.username, pipeline_name)
            .unwrap_or_else(|e| panic!("{e}"));

        let mut next = (*existing).clone();
        update(&mut next);

        self.pipeline_catalog
            .replace(&self.username, pipeline_name, Arc::new(next))
            .unwrap_or_else(|e| panic!("{e}"));

        self.pipeline_info(pipeline_name)
    }
}

impl LinkPredictionFacade for LocalLinkPredictionFacade {
    fn add_feature(
        &self,
        pipeline_name: &str,
        feature_type: &str,
        raw_configuration: RawConfig,
    ) -> Vec<PipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            let node_props_value = raw_configuration.get("nodeProperties").unwrap_or_else(|| {
                panic!("add_feature expects configuration key `nodeProperties`")
            });

            let step = LinkFeatureStepFactory::create_from_config(feature_type, node_props_value)
                .unwrap_or_else(|e| panic!("{e}"));

            pipeline.add_feature_step(step);
        })
    }

    fn add_logistic_regression(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<PipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            pipeline
                .training_parameter_space_mut()
                .entry(TrainingMethod::LogisticRegression)
                .or_default();
        })
    }

    fn add_mlp(&self, pipeline_name: &str, _configuration: RawConfig) -> Vec<PipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            pipeline
                .training_parameter_space_mut()
                .entry(TrainingMethod::MLPClassification)
                .or_default();
        })
    }

    fn add_node_property(
        &self,
        pipeline_name: &str,
        task_name: &str,
        procedure_config: RawConfig,
    ) -> Vec<PipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            let step = NodePropertyStep::new(task_name.to_string(), procedure_config);
            let step_box: Box<dyn ExecutableNodePropertyStep> = Box::new(step);

            pipeline
                .validate_unique_mutate_property(step_box.as_ref())
                .unwrap_or_else(|e| panic!("{e}"));

            pipeline.add_node_property_step(step_box);
        })
    }

    fn add_random_forest(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<PipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            pipeline
                .training_parameter_space_mut()
                .entry(TrainingMethod::RandomForestClassification)
                .or_default();
        })
    }

    fn configure_auto_tuning(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<PipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            let cfg = Self::parse_auto_tuning_config(&configuration);
            pipeline.set_auto_tuning_config(cfg);
        })
    }

    fn configure_split(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<PipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            let cfg = Self::parse_split_config(&configuration);
            pipeline.set_split_config(cfg);
        })
    }

    fn create_pipeline(&self, pipeline_name: &str) -> Vec<PipelineInfoResult> {
        let pipeline = Arc::new(LinkPredictionTrainingPipeline::new());
        self.pipeline_catalog
            .set(&self.username, pipeline_name, Arc::clone(&pipeline))
            .unwrap_or_else(|e| panic!("{e}"));

        vec![Self::pipeline_info_from_pipeline(
            pipeline_name,
            pipeline.as_ref(),
        )]
    }

    fn mutate(&self, _graph_name: &str, _configuration: RawConfig) -> Vec<MutateResult> {
        vec![]
    }

    fn mutate_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        vec![]
    }

    fn stream(&self, _graph_name: &str, _configuration: RawConfig) -> Vec<StreamResult> {
        vec![]
    }

    fn stream_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        vec![]
    }

    fn train(
        &self,
        _graph_name: &str,
        _configuration: RawConfig,
    ) -> Vec<LinkPredictionTrainResult> {
        vec![]
    }

    fn train_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        vec![]
    }
}

pub struct LocalNodeClassificationFacade {
    username: String,
    pipeline_catalog: Arc<PipelineCatalog>,
}

impl LocalNodeClassificationFacade {
    pub fn new(username: String, pipeline_catalog: Arc<PipelineCatalog>) -> Self {
        Self {
            username,
            pipeline_catalog,
        }
    }

    fn pipeline_info(&self, pipeline_name: &str) -> Vec<NodePipelineInfoResult> {
        let pipeline = self
            .pipeline_catalog
            .get_typed::<NodeClassificationTrainingPipeline>(&self.username, pipeline_name)
            .unwrap_or_else(|e| panic!("{e}"));

        vec![Self::node_pipeline_info_from_pipeline(
            pipeline_name,
            pipeline.as_ref(),
        )]
    }

    fn node_pipeline_info_from_pipeline(
        pipeline_name: &str,
        pipeline: &NodeClassificationTrainingPipeline,
    ) -> NodePipelineInfoResult {
        let node_property_steps: AnyMapList = pipeline
            .node_property_steps()
            .iter()
            .map(|step| step.to_map())
            .collect();

        let feature_properties = pipeline.feature_properties();

        let split_config: AnyMap = pipeline
            .split_config()
            .to_map()
            .into_iter()
            .map(|(k, v)| (k, Value::String(v)))
            .collect();

        let auto_tuning_config: AnyMap = pipeline.auto_tuning_config().to_map();

        let parameter_space = pipeline.parameter_space_to_map();

        NodePipelineInfoResult {
            name: pipeline_name.to_string(),
            node_property_steps,
            feature_properties,
            split_config,
            auto_tuning_config,
            parameter_space,
        }
    }

    fn parse_split_config(configuration: &RawConfig) -> NodePropertyPredictionSplitConfig {
        let test_fraction = configuration
            .get("testFraction")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.3);

        let validation_folds = configuration
            .get("validationFolds")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(3);

        NodePropertyPredictionSplitConfig::new(test_fraction, validation_folds)
            .unwrap_or_else(|e| panic!("{e}"))
    }

    fn parse_auto_tuning_config(configuration: &RawConfig) -> AutoTuningConfig {
        let max_trials = configuration
            .get("maxTrials")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(AutoTuningConfig::MAX_TRIALS);

        AutoTuningConfig::new(max_trials).unwrap_or_else(|e| panic!("{e}"))
    }

    fn with_pipeline_update(
        &self,
        pipeline_name: &str,
        update: impl FnOnce(&mut NodeClassificationTrainingPipeline),
    ) -> Vec<NodePipelineInfoResult> {
        let existing = self
            .pipeline_catalog
            .get_typed::<NodeClassificationTrainingPipeline>(&self.username, pipeline_name)
            .unwrap_or_else(|e| panic!("{e}"));

        let mut next = (*existing).clone();
        update(&mut next);

        self.pipeline_catalog
            .replace(&self.username, pipeline_name, Arc::new(next))
            .unwrap_or_else(|e| panic!("{e}"));

        self.pipeline_info(pipeline_name)
    }
}

impl NodeClassificationFacade for LocalNodeClassificationFacade {
    fn add_logistic_regression(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            pipeline
                .training_parameter_space_mut()
                .entry(TrainingMethod::LogisticRegression)
                .or_default();
        })
    }

    fn add_mlp(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            pipeline
                .training_parameter_space_mut()
                .entry(TrainingMethod::MLPClassification)
                .or_default();
        })
    }

    fn add_node_property(
        &self,
        pipeline_name: &str,
        task_name: &str,
        procedure_config: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            let step = NodePropertyStep::new(task_name.to_string(), procedure_config);
            let step_box: Box<dyn ExecutableNodePropertyStep> = Box::new(step);

            pipeline
                .validate_unique_mutate_property(step_box.as_ref())
                .unwrap_or_else(|e| panic!("{e}"));

            pipeline.add_node_property_step(step_box);
        })
    }

    fn add_random_forest(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            pipeline
                .training_parameter_space_mut()
                .entry(TrainingMethod::RandomForestClassification)
                .or_default();
        })
    }

    fn configure_auto_tuning(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            let cfg = Self::parse_auto_tuning_config(&configuration);
            pipeline.set_auto_tuning_config(cfg);
        })
    }

    fn configure_split(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            let cfg = Self::parse_split_config(&configuration);
            pipeline.set_split_config(cfg);
        })
    }

    fn create_pipeline(&self, pipeline_name: &str) -> Vec<NodePipelineInfoResult> {
        let pipeline = Arc::new(NodeClassificationTrainingPipeline::new());
        self.pipeline_catalog
            .set(&self.username, pipeline_name, Arc::clone(&pipeline))
            .unwrap_or_else(|e| panic!("{e}"));

        vec![Self::node_pipeline_info_from_pipeline(
            pipeline_name,
            pipeline.as_ref(),
        )]
    }

    fn mutate(&self, _graph_name: &str, _configuration: RawConfig) -> Vec<PredictMutateResult> {
        vec![]
    }

    fn mutate_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        vec![]
    }

    fn select_features(
        &self,
        pipeline_name: &str,
        node_feature_steps: Value,
    ) -> Vec<NodePipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            let steps = match node_feature_steps {
                Value::Array(values) => values,
                _ => panic!("select_features expects a JSON array"),
            };

            let mut next_steps = Vec::with_capacity(steps.len());
            for v in steps {
                let prop = v
                    .as_str()
                    .unwrap_or_else(|| panic!("feature entry must be a string"));
                next_steps.push(NodeFeatureStep::of(prop));
            }

            // Replace features (mirrors Java 'selectFeatures' semantics)
            pipeline.set_feature_steps(next_steps);
        })
    }

    fn stream(
        &self,
        _graph_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodeClassificationStreamResult> {
        vec![]
    }

    fn stream_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        vec![]
    }

    fn train(
        &self,
        _graph_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodeClassificationPipelineTrainResult> {
        vec![]
    }

    fn train_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        vec![]
    }

    fn write(&self, _graph_name: &str, _configuration: RawConfig) -> Vec<WriteResult> {
        vec![]
    }

    fn write_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        vec![]
    }
}

// Regression facade mirrors node classification behavior.
pub struct LocalNodeRegressionFacade {
    username: String,
    pipeline_catalog: Arc<PipelineCatalog>,
}

impl LocalNodeRegressionFacade {
    pub fn new(username: String, pipeline_catalog: Arc<PipelineCatalog>) -> Self {
        Self {
            username,
            pipeline_catalog,
        }
    }

    fn pipeline_info(&self, pipeline_name: &str) -> Vec<NodePipelineInfoResult> {
        let pipeline = self
            .pipeline_catalog
            .get_typed::<NodeRegressionTrainingPipeline>(&self.username, pipeline_name)
            .unwrap_or_else(|e| panic!("{e}"));

        vec![Self::node_pipeline_info_from_pipeline(
            pipeline_name,
            pipeline.as_ref(),
        )]
    }

    fn node_pipeline_info_from_pipeline(
        pipeline_name: &str,
        pipeline: &NodeRegressionTrainingPipeline,
    ) -> NodePipelineInfoResult {
        let node_property_steps: AnyMapList = pipeline
            .node_property_steps()
            .iter()
            .map(|step| step.to_map())
            .collect();

        let feature_properties = pipeline.feature_properties();

        let split_config: AnyMap = pipeline
            .split_config()
            .to_map()
            .into_iter()
            .map(|(k, v)| (k, Value::String(v)))
            .collect();

        let auto_tuning_config: AnyMap = pipeline.auto_tuning_config().to_map();

        let parameter_space = pipeline.parameter_space_to_map();

        NodePipelineInfoResult {
            name: pipeline_name.to_string(),
            node_property_steps,
            feature_properties,
            split_config,
            auto_tuning_config,
            parameter_space,
        }
    }

    fn parse_split_config(configuration: &RawConfig) -> NodePropertyPredictionSplitConfig {
        let test_fraction = configuration
            .get("testFraction")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.3);

        let validation_folds = configuration
            .get("validationFolds")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(3);

        NodePropertyPredictionSplitConfig::new(test_fraction, validation_folds)
            .unwrap_or_else(|e| panic!("{e}"))
    }

    fn parse_auto_tuning_config(configuration: &RawConfig) -> AutoTuningConfig {
        let max_trials = configuration
            .get("maxTrials")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(AutoTuningConfig::MAX_TRIALS);

        AutoTuningConfig::new(max_trials).unwrap_or_else(|e| panic!("{e}"))
    }

    fn with_pipeline_update(
        &self,
        pipeline_name: &str,
        update: impl FnOnce(&mut NodeRegressionTrainingPipeline),
    ) -> Vec<NodePipelineInfoResult> {
        let existing = self
            .pipeline_catalog
            .get_typed::<NodeRegressionTrainingPipeline>(&self.username, pipeline_name)
            .unwrap_or_else(|e| panic!("{e}"));

        let mut next = (*existing).clone();
        update(&mut next);

        self.pipeline_catalog
            .replace(&self.username, pipeline_name, Arc::new(next))
            .unwrap_or_else(|e| panic!("{e}"));

        self.pipeline_info(pipeline_name)
    }
}

impl NodeRegressionFacade for LocalNodeRegressionFacade {
    fn add_logistic_regression(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            pipeline
                .training_parameter_space_mut()
                .entry(TrainingMethod::LinearRegression)
                .or_default();
        })
    }

    fn add_node_property(
        &self,
        pipeline_name: &str,
        task_name: &str,
        procedure_config: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            let step = NodePropertyStep::new(task_name.to_string(), procedure_config);
            let step_box: Box<dyn ExecutableNodePropertyStep> = Box::new(step);

            pipeline
                .validate_unique_mutate_property(step_box.as_ref())
                .unwrap_or_else(|e| panic!("{e}"));

            pipeline.add_node_property_step(step_box);
        })
    }

    fn add_random_forest(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            pipeline
                .training_parameter_space_mut()
                .entry(TrainingMethod::RandomForestRegression)
                .or_default();
        })
    }

    fn configure_auto_tuning(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            let cfg = Self::parse_auto_tuning_config(&configuration);
            pipeline.set_auto_tuning_config(cfg);
        })
    }

    fn configure_split(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        self.with_pipeline_update(pipeline_name, |pipeline| {
            let cfg = Self::parse_split_config(&configuration);
            pipeline.set_split_config(cfg);
        })
    }

    fn create_pipeline(&self, pipeline_name: &str) -> Vec<NodePipelineInfoResult> {
        let pipeline = Arc::new(NodeRegressionTrainingPipeline::new());
        self.pipeline_catalog
            .set(&self.username, pipeline_name, Arc::clone(&pipeline))
            .unwrap_or_else(|e| panic!("{e}"));

        vec![Self::node_pipeline_info_from_pipeline(
            pipeline_name,
            pipeline.as_ref(),
        )]
    }

    fn mutate(&self, _graph_name: &str, _configuration: RawConfig) -> Vec<PredictMutateResult> {
        vec![]
    }

    fn stream(
        &self,
        _graph_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodeRegressionStreamResult> {
        vec![]
    }

    fn select_features(
        &self,
        pipeline_name: &str,
        feature_properties: Value,
    ) -> Vec<NodePipelineInfoResult> {
        let existing = self
            .pipeline_catalog
            .get_typed::<NodeRegressionTrainingPipeline>(&self.username, pipeline_name)
            .unwrap_or_else(|e| panic!("{e}"));

        let steps = match feature_properties {
            Value::Array(values) => values,
            _ => panic!("select_features expects a JSON array"),
        };

        let mut next_steps = Vec::with_capacity(steps.len());
        for v in steps {
            let prop = v
                .as_str()
                .unwrap_or_else(|| panic!("feature entry must be a string"));
            next_steps.push(NodeFeatureStep::of(prop));
        }

        let mut next = NodeRegressionTrainingPipeline::new();
        for step in existing.node_property_steps() {
            next.add_node_property_step(step.clone());
        }
        for step in next_steps {
            next.add_feature_step(step);
        }
        for (method, configs) in existing.training_parameter_space() {
            next.training_parameter_space_mut()
                .insert(*method, configs.iter().cloned().collect());
        }
        next.set_auto_tuning_config(existing.auto_tuning_config().clone());
        next.set_split_config(existing.split_config().clone());

        let next = Arc::new(next);
        self.pipeline_catalog
            .replace(&self.username, pipeline_name, Arc::clone(&next))
            .unwrap_or_else(|e| panic!("{e}"));

        vec![Self::node_pipeline_info_from_pipeline(
            pipeline_name,
            next.as_ref(),
        )]
    }

    fn train(
        &self,
        _graph_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodeRegressionPipelineTrainResult> {
        vec![]
    }
}

#[cfg(test)]
mod node_classification_facade_tests {
    use super::*;

    #[test]
    fn node_classification_create_and_configure_pipeline() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        let created = facade.node_classification().create_pipeline("p1");
        assert_eq!(created.len(), 1);
        assert_eq!(created[0].name, "p1");
        assert!(created[0].node_property_steps.is_empty());
        assert!(created[0].feature_properties.is_empty());

        let configured = facade.node_classification().configure_split(
            "p1",
            AnyMap::from([
                ("testFraction".to_string(), Value::from(0.2)),
                ("validationFolds".to_string(), Value::from(5)),
            ]),
        );
        assert_eq!(
            configured[0].split_config.get("testFraction"),
            Some(&Value::String("0.2".to_string()))
        );
        assert_eq!(
            configured[0].split_config.get("validationFolds"),
            Some(&Value::String("5".to_string()))
        );
    }

    #[test]
    fn node_classification_add_node_property_and_select_features() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        facade.node_classification().create_pipeline("p1");

        let updated = facade.node_classification().add_node_property(
            "p1",
            "gds.debug.writeConstantDouble.mutate",
            AnyMap::from([
                (
                    "mutateProperty".to_string(),
                    Value::String("pagerank".to_string()),
                ),
                ("value".to_string(), Value::from(1.0)),
            ]),
        );

        assert_eq!(updated.len(), 1);
        assert_eq!(updated[0].node_property_steps.len(), 1);

        let with_features = facade.node_classification().select_features(
            "p1",
            Value::Array(vec![Value::String("pagerank".to_string())]),
        );
        assert_eq!(
            with_features[0].feature_properties,
            vec!["pagerank".to_string()]
        );
    }
}

#[cfg(test)]
mod link_prediction_facade_tests {
    use super::*;

    #[test]
    fn link_prediction_create_add_feature_and_configure_split() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        let created = facade.link_prediction().create_pipeline("lp1");
        assert_eq!(created.len(), 1);
        assert_eq!(created[0].name, "lp1");
        assert!(created[0].feature_steps.is_empty());

        let with_feature = facade.link_prediction().add_feature(
            "lp1",
            "COSINE",
            AnyMap::from([(
                "nodeProperties".to_string(),
                Value::Array(vec![Value::String("embedding".to_string())]),
            )]),
        );
        assert_eq!(with_feature.len(), 1);
        assert_eq!(with_feature[0].feature_steps.len(), 1);
        assert_eq!(
            with_feature[0].feature_steps[0].get("name"),
            Some(&Value::String("COSINE".to_string()))
        );

        let configured = facade.link_prediction().configure_split(
            "lp1",
            AnyMap::from([
                ("testFraction".to_string(), Value::from(0.2)),
                ("trainFraction".to_string(), Value::from(0.3)),
                ("validationFolds".to_string(), Value::from(5)),
            ]),
        );

        assert_eq!(
            configured[0].split_config.get("testFraction"),
            Some(&Value::from(0.2))
        );
        assert_eq!(
            configured[0].split_config.get("trainFraction"),
            Some(&Value::from(0.3))
        );
        assert_eq!(
            configured[0].split_config.get("validationFolds"),
            Some(&Value::from(5))
        );
    }

    #[test]
    fn link_prediction_add_node_property() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        facade.link_prediction().create_pipeline("lp1");

        let updated = facade.link_prediction().add_node_property(
            "lp1",
            "gds.debug.writeConstantDouble.mutate",
            AnyMap::from([
                (
                    "mutateProperty".to_string(),
                    Value::String("pagerank".to_string()),
                ),
                ("value".to_string(), Value::from(1.0)),
            ]),
        );

        assert_eq!(updated.len(), 1);
        assert_eq!(updated[0].node_property_steps.len(), 1);
    }
}
