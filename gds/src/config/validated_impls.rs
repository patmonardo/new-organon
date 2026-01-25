//! Centralized ValidatedConfig implementations for various types.
//!
//! This keeps `ValidatedConfig` impls in one place to avoid editing many upstream
//! files and to provide consistent error mapping for different local error types.

use crate::config::validation::ConfigError as RootConfigError;

// Import base train config trait so we can call pipeline/target_property accessors
use crate::projection::eval::pipeline::node_pipeline::node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;

macro_rules! map_err_to_root {
    ($e:expr, $name:expr) => {
        match $e {
            Ok(v) => Ok(v),
            Err(err) => Err(RootConfigError::InvalidParameter {
                parameter: $name.to_string(),
                reason: err.to_string(),
            }),
        }
    };
}

// Algorithm specs ------------------------------------------------------------

impl crate::config::ValidatedConfig for crate::algo::astar::AStarConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(
            crate::algo::astar::AStarConfig::validate(self),
            "AStarConfig"
        )
    }
}

impl crate::config::ValidatedConfig for crate::algo::bellman_ford::BellmanFordConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(
            crate::algo::bellman_ford::BellmanFordConfig::validate(self),
            "BellmanFordConfig"
        )
    }
}

impl crate::config::ValidatedConfig for crate::algo::hits::HitsConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(crate::algo::hits::HitsConfig::validate(self), "HitsConfig")
    }
}

impl crate::config::ValidatedConfig for crate::algo::spanning_tree::SpanningTreeConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(
            crate::algo::spanning_tree::SpanningTreeConfig::validate(self),
            "SpanningTreeConfig"
        )
    }
}

impl crate::config::ValidatedConfig for crate::algo::bridges::BridgesConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(
            crate::algo::bridges::BridgesConfig::validate(self),
            "BridgesConfig"
        )
    }
}

impl crate::config::ValidatedConfig for crate::algo::yens::YensConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(crate::algo::yens::YensConfig::validate(self), "YensConfig")
    }
}

impl crate::config::ValidatedConfig for crate::algo::k1coloring::K1ColoringConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(
            crate::algo::k1coloring::K1ColoringConfig::validate(self),
            "K1ColoringConfig"
        )
    }
}

impl crate::config::ValidatedConfig for crate::algo::degree_centrality::DegreeCentralityConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(
            crate::algo::degree_centrality::DegreeCentralityConfig::validate(self),
            "DegreeCentralityConfig"
        )
    }
}

impl crate::config::ValidatedConfig for crate::algo::articulation_points::ArticulationPointsConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(
            crate::algo::articulation_points::ArticulationPointsConfig::validate(self),
            "ArticulationPointsConfig"
        )
    }
}

impl crate::config::ValidatedConfig for crate::algo::dijkstra::DijkstraConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(
            crate::algo::dijkstra::DijkstraConfig::validate(self),
            "DijkstraConfig"
        )
    }
}

impl crate::config::ValidatedConfig for crate::algo::closeness::ClosenessCentralityConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(
            crate::algo::closeness::ClosenessCentralityConfig::validate(self),
            "ClosenessCentralityConfig"
        )
    }
}

impl crate::config::ValidatedConfig for crate::algo::all_shortest_paths::AllShortestPathsConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(
            crate::algo::all_shortest_paths::AllShortestPathsConfig::validate(self),
            "AllShortestPathsConfig"
        )
    }
}

// Application configs (graph_store_catalog) ---------------------------------

impl crate::config::ValidatedConfig
    for crate::applications::graph_store_catalog::GraphWriteNodePropertiesConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        self.validate()
    }
}

impl crate::config::ValidatedConfig
    for crate::applications::graph_store_catalog::GraphExportNodePropertiesConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        self.validate()
    }
}

impl crate::config::ValidatedConfig
    for crate::applications::graph_store_catalog::GraphRemoveGraphPropertiesConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        self.validate()
    }
}

impl crate::config::ValidatedConfig
    for crate::applications::graph_store_catalog::GraphStreamGraphPropertiesConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        self.validate()
    }
}

impl crate::config::ValidatedConfig
    for crate::applications::graph_store_catalog::GraphNodePropertiesConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        self.validate()
    }
}

impl crate::config::ValidatedConfig
    for crate::applications::graph_store_catalog::GraphStreamRelationshipPropertiesConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        self.validate()
    }
}

impl crate::config::ValidatedConfig
    for crate::applications::graph_store_catalog::GraphStreamRelationshipsConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        self.validate()
    }
}

impl crate::config::ValidatedConfig
    for crate::applications::graph_store_catalog::GraphGenerationConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        // conservative checks: if node_count is specified, it must be > 0
        if let Some(n) = self.node_count {
            if n == 0 {
                return Err(RootConfigError::InvalidParameter {
                    parameter: "nodeCount".to_string(),
                    reason: "nodeCount must be > 0".to_string(),
                });
            }
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for crate::applications::graph_store_catalog::SamplingConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        // No strict validation currently; leave as no-op for now
        Ok(())
    }
}

impl crate::config::ValidatedConfig
    for crate::applications::graph_store_catalog::GraphAccessGraphPropertiesConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        self.validate()
    }
}

impl crate::config::ValidatedConfig
    for crate::applications::graph_store_catalog::WriteRelationshipPropertiesConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        // marker config, no-op
        Ok(())
    }
}
// Projection and pipeline configs -------------------------------------------

impl crate::config::ValidatedConfig for crate::projection::factory::arrow::ArrowProjectionConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(self.validate(), "ArrowProjectionConfig")
    }
}

impl crate::config::ValidatedConfig
    for crate::projection::eval::pipeline::link_pipeline::train::LinkPredictionTrainConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        map_err_to_root!(self.validate(), "LinkPredictionTrainConfig")
    }
}

impl crate::config::ValidatedConfig
    for crate::projection::eval::pipeline::link_pipeline::LinkPredictionSplitConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        if self.validation_folds() < 2 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "validationFolds".to_string(),
                reason: format!(
                    "validationFolds must be at least 2, got {}",
                    self.validation_folds()
                ),
            });
        }
        if self.test_fraction() <= 0.0 || self.test_fraction() >= 1.0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "testFraction".to_string(),
                reason: format!(
                    "testFraction must be in range (0.0, 1.0), got {}",
                    self.test_fraction()
                ),
            });
        }
        if self.train_fraction() <= 0.0 || self.train_fraction() >= 1.0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "trainFraction".to_string(),
                reason: format!(
                    "trainFraction must be in range (0.0, 1.0), got {}",
                    self.train_fraction()
                ),
            });
        }
        if self.negative_sampling_ratio() <= 0.0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "negativeSamplingRatio".to_string(),
                reason: format!(
                    "negativeSamplingRatio must be positive, got {}",
                    self.negative_sampling_ratio()
                ),
            });
        }
        if self.negative_relationship_type().is_some() && self.negative_sampling_ratio() != 1.0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "negativeSamplingRatio/negativeRelationshipType".to_string(),
                reason:
                    "negativeSamplingRatio and negativeRelationshipType cannot be used together"
                        .to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig
    for crate::projection::eval::pipeline::node_pipeline::NodePropertyPredictionSplitConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        if !(0.0..=1.0).contains(&self.test_fraction()) {
            return Err(RootConfigError::InvalidParameter {
                parameter: "testFraction".to_string(),
                reason: format!(
                    "testFraction must be between 0.0 and 1.0, got {}",
                    self.test_fraction()
                ),
            });
        }
        if self.validation_folds() < 2 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "validationFolds".to_string(),
                reason: format!(
                    "validationFolds must be at least 2, got {}",
                    self.validation_folds()
                ),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for crate::projection::eval::pipeline::node_pipeline::classification::NodeClassificationPipelineTrainConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        crate::config::validate_non_empty_string(self.pipeline(), "pipeline")?;
        crate::config::validate_non_empty_string(self.target_property(), "targetProperty")?;
        Ok(())
    }
}

impl crate::config::ValidatedConfig for crate::projection::eval::pipeline::node_pipeline::regression::NodeRegressionPipelineTrainConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        crate::config::validate_non_empty_string(self.pipeline(), "pipeline")?;
        crate::config::validate_non_empty_string(self.target_property(), "targetProperty")?;
        // Ensure metrics are specified
        match self.validate_metrics() {
            Ok(()) => Ok(()),
            Err(msg) => Err(RootConfigError::InvalidParameter { parameter: "metrics".to_string(), reason: msg }),
        }
    }
}

impl crate::config::ValidatedConfig
    for crate::projection::eval::pipeline::auto_tuning_config::AutoTuningConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        if self.max_trials() < 1 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "maxTrials".to_string(),
                reason: format!("maxTrials must be >= 1, got {}", self.max_trials()),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for crate::projection::eval::pipeline::node_property_step_context_config::NodePropertyStepContextConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        // no-op: empty lists are allowed
        Ok(())
    }
}

// ML / Embeddings ----------------------------------------------------------

impl crate::config::ValidatedConfig for crate::algo::embeddings::fastrp::FastRPConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        if self.embedding_dimension == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "embeddingDimension".to_string(),
                reason: "embeddingDimension must be > 0".to_string(),
            });
        }
        if self.property_dimension > self.embedding_dimension {
            return Err(RootConfigError::InvalidParameter {
                parameter: "propertyDimension".to_string(),
                reason: "propertyDimension must be <= embeddingDimension".to_string(),
            });
        }
        if self.iteration_weights.is_empty() {
            return Err(RootConfigError::InvalidParameter {
                parameter: "iterationWeights".to_string(),
                reason: "iterationWeights must be non-empty".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for crate::algo::embeddings::hashgnn::HashGNNConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        if self.iterations == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "iterations".to_string(),
                reason: "iterations must be > 0".to_string(),
            });
        }
        if self.embedding_density == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "embeddingDensity".to_string(),
                reason: "embeddingDensity must be > 0".to_string(),
            });
        }
        if self.concurrency == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be > 0".to_string(),
            });
        }
        if let Some(out) = self.output_dimension {
            if out == 0 {
                return Err(RootConfigError::InvalidParameter {
                    parameter: "outputDimension".to_string(),
                    reason: "outputDimension must be > 0".to_string(),
                });
            }
        }
        if self.feature_properties.is_empty() && self.generate_features.is_none() {
            return Err(RootConfigError::InvalidParameter {
                parameter: "featureProperties/generateFeatures".to_string(),
                reason: "HashGNN requires either featureProperties or generateFeatures".to_string(),
            });
        }
        if let Some(cfg) = &self.generate_features {
            if cfg.dimension == 0 {
                return Err(RootConfigError::InvalidParameter {
                    parameter: "generateFeatures.dimension".to_string(),
                    reason: "generateFeatures.dimension must be > 0".to_string(),
                });
            }
            if cfg.density_level == 0 {
                return Err(RootConfigError::InvalidParameter {
                    parameter: "generateFeatures.densityLevel".to_string(),
                    reason: "generateFeatures.densityLevel must be > 0".to_string(),
                });
            }
        }
        if let Some(cfg) = &self.binarize_features {
            if cfg.dimension == 0 {
                return Err(RootConfigError::InvalidParameter {
                    parameter: "binarizeFeatures.dimension".to_string(),
                    reason: "binarizeFeatures.dimension must be > 0".to_string(),
                });
            }
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for crate::algo::embeddings::gat::GATConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        if self.embedding_dimension == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "embeddingDimension".to_string(),
                reason: "embeddingDimension must be > 0".to_string(),
            });
        }
        if self.num_heads == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "numHeads".to_string(),
                reason: "numHeads must be > 0".to_string(),
            });
        }
        if self.num_layers == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "numLayers".to_string(),
                reason: "numLayers must be > 0".to_string(),
            });
        }
        if self.learning_rate <= 0.0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "learningRate".to_string(),
                reason: "learningRate must be > 0".to_string(),
            });
        }
        if self.epochs == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "epochs".to_string(),
                reason: "epochs must be > 0".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&self.dropout) {
            return Err(RootConfigError::InvalidParameter {
                parameter: "dropout".to_string(),
                reason: "dropout must be in [0.0, 1.0]".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&self.alpha) {
            return Err(RootConfigError::InvalidParameter {
                parameter: "alpha".to_string(),
                reason: "alpha must be in [0.0, 1.0]".to_string(),
            });
        }
        if self.concurrency == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be > 0".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for crate::algo::embeddings::node2vec::Node2VecConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        if self.walks_per_node == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "walksPerNode".to_string(),
                reason: "walksPerNode must be > 0".to_string(),
            });
        }
        if self.walk_length == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "walkLength".to_string(),
                reason: "walkLength must be > 0".to_string(),
            });
        }
        if self.embedding_dimension == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "embeddingDimension".to_string(),
                reason: "embeddingDimension must be > 0".to_string(),
            });
        }
        if self.window_size == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "windowSize".to_string(),
                reason: "windowSize must be > 0".to_string(),
            });
        }
        if self.iterations == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "iterations".to_string(),
                reason: "iterations must be > 0".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for crate::algo::embeddings::graphsage::GraphSageTrainConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        crate::config::validate_non_empty_string(&self.model_user, "modelUser")?;
        crate::config::validate_non_empty_string(&self.model_name, "modelName")?;
        if self.batch_size == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "batchSize".to_string(),
                reason: "batchSize must be > 0".to_string(),
            });
        }
        if self.learning_rate <= 0.0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "learningRate".to_string(),
                reason: "learningRate must be > 0".to_string(),
            });
        }
        if self.tolerance < 0.0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "tolerance".to_string(),
                reason: "tolerance must be >= 0".to_string(),
            });
        }
        if self.embedding_dimension == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "embeddingDimension".to_string(),
                reason: "embeddingDimension must be > 0".to_string(),
            });
        }
        for &s in &self.sample_sizes {
            if s == 0 {
                return Err(RootConfigError::InvalidParameter {
                    parameter: "sampleSizes".to_string(),
                    reason: "sampleSizes entries must be > 0".to_string(),
                });
            }
        }
        Ok(())
    }
}
