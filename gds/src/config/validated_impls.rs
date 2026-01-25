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
    for crate::applications::graph_store_catalog::GraphStreamRelationshipPropertiesConfig
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

// ML / Trainer configs ----------------------------------------------------

impl crate::config::ValidatedConfig for crate::ml::gradient_descent::GradientDescentConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        if self.batch_size() == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "batchSize".to_string(),
                reason: "batchSize must be > 0".to_string(),
            });
        }
        if self.min_epochs() == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "minEpochs".to_string(),
                reason: "minEpochs must be >= 1".to_string(),
            });
        }
        if self.max_epochs() < self.min_epochs() {
            return Err(RootConfigError::InvalidParameter {
                parameter: "maxEpochs".to_string(),
                reason: "maxEpochs must be >= minEpochs".to_string(),
            });
        }
        if self.learning_rate() <= 0.0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "learningRate".to_string(),
                reason: "learningRate must be > 0".to_string(),
            });
        }
        if self.tolerance() < 0.0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "tolerance".to_string(),
                reason: "tolerance must be >= 0".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for crate::ml::models::mlp::MLPClassifierTrainConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        // reuse gradient-like checks
        if self.batch_size == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "batchSize".to_string(),
                reason: "batchSize must be > 0".to_string(),
            });
        }
        if self.max_epochs < self.min_epochs {
            return Err(RootConfigError::InvalidParameter {
                parameter: "maxEpochs/minEpochs".to_string(),
                reason: "maxEpochs must be >= minEpochs".to_string(),
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
        if self.hidden_layer_sizes.is_empty() {
            return Err(RootConfigError::InvalidParameter {
                parameter: "hiddenLayerSizes".to_string(),
                reason: "hiddenLayerSizes must not be empty".to_string(),
            });
        }
        for &n in &self.hidden_layer_sizes {
            if n == 0 {
                return Err(RootConfigError::InvalidParameter {
                    parameter: "hiddenLayerSizes".to_string(),
                    reason: "hiddenLayerSizes entries must be > 0".to_string(),
                });
            }
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig
    for crate::ml::models::logistic_regression::LogisticRegressionTrainConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
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
        if self.max_epochs == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "maxEpochs".to_string(),
                reason: "maxEpochs must be > 0".to_string(),
            });
        }
        if self.tolerance < 0.0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "tolerance".to_string(),
                reason: "tolerance must be >= 0".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig
    for crate::ml::models::linear_regression::LinearRegressionTrainConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        // Validate nested gradient descent config
        crate::config::ValidatedConfig::validate(self.gradient())?;
        if self.penalty() < 0.0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "penalty".to_string(),
                reason: "penalty must be >= 0".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for crate::ml::models::random_forest::RandomForestConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        // numSamplesRatio must be a finite number in (0, 1]
        if !self.num_samples_ratio.is_finite()
            || !(self.num_samples_ratio > 0.0 && self.num_samples_ratio <= 1.0)
        {
            return Err(RootConfigError::InvalidParameter {
                parameter: "numSamplesRatio".to_string(),
                reason: "numSamplesRatio must be finite and in (0.0, 1.0]".to_string(),
            });
        }

        // sensible upper bound on number of trees to catch accidental misuse (1M trees)
        if self.num_decision_trees == 0 || self.num_decision_trees > 1_000_000 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "numDecisionTrees".to_string(),
                reason: "numDecisionTrees must be > 0 and <= 1_000_000".to_string(),
            });
        }

        // maxDepth: 0 means unlimited, otherwise must be >= 1 and not unreasonably large
        if !crate::ml::decision_tree::is_unlimited_depth(self.max_depth) {
            if self.max_depth < 1 {
                return Err(RootConfigError::InvalidParameter {
                    parameter: "maxDepth".to_string(),
                    reason: "maxDepth must be 0 (unlimited) or >= 1".to_string(),
                });
            }
            if self.max_depth > 10_000 {
                return Err(RootConfigError::InvalidParameter {
                    parameter: "maxDepth".to_string(),
                    reason: "maxDepth must be 0 (unlimited) or in [1, 10_000]".to_string(),
                });
            }
        }

        if self.min_samples_split < 2 || self.min_samples_split > 1_000_000 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "minSamplesSplit".to_string(),
                reason: "minSamplesSplit must be >= 2 and <= 1_000_000".to_string(),
            });
        }
        if self.min_samples_leaf < 1 || self.min_samples_leaf > 1_000_000 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "minSamplesLeaf".to_string(),
                reason: "minSamplesLeaf must be >= 1 and <= 1_000_000".to_string(),
            });
        }
        // Ensure per-tree split/leaf sizes are consistent
        if self.min_samples_leaf >= self.min_samples_split {
            return Err(RootConfigError::InvalidParameter {
                parameter: "minSamplesLeaf/minSamplesSplit".to_string(),
                reason: "minSamplesLeaf must be strictly smaller than minSamplesSplit".to_string(),
            });
        }

        if let Some(r) = self.max_features_ratio {
            if !r.is_finite() || !(r > 0.0 && r <= 1.0) {
                return Err(RootConfigError::InvalidParameter {
                    parameter: "maxFeaturesRatio".to_string(),
                    reason: "maxFeaturesRatio must be finite and in (0.0, 1.0]".to_string(),
                });
            }
        }

        Ok(())
    }
}

impl crate::config::ValidatedConfig
    for crate::ml::models::random_forest::RandomForestClassifierTrainerConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        self.forest.validate()
    }
}

impl crate::config::ValidatedConfig
    for crate::ml::models::random_forest::RandomForestRegressorTrainerConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        self.forest.validate()
    }
}

impl crate::config::ValidatedConfig
    for crate::ml::decision_tree::trainer_config::DecisionTreeTrainerConfig
{
    fn validate(&self) -> Result<(), RootConfigError> {
        // max_depth: 0 means unlimited; otherwise enforce reasonable bounds
        if !crate::ml::decision_tree::is_unlimited_depth(self.max_depth()) {
            if self.max_depth() < 1 {
                return Err(RootConfigError::InvalidParameter {
                    parameter: "maxDepth".to_string(),
                    reason: "maxDepth must be >= 1 when set explicitly".to_string(),
                });
            }
            if self.max_depth() > 10_000 {
                return Err(RootConfigError::InvalidParameter {
                    parameter: "maxDepth".to_string(),
                    reason: "maxDepth must be in [1, 10_000] when set explicitly".to_string(),
                });
            }
        }

        if self.min_split_size() < 2 || self.min_split_size() > 1_000_000 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "minSplitSize".to_string(),
                reason: "minSplitSize must be >= 2 and <= 1_000_000".to_string(),
            });
        }
        if self.min_leaf_size() < 1 || self.min_leaf_size() > 1_000_000 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "minLeafSize".to_string(),
                reason: "minLeafSize must be >= 1 and <= 1_000_000".to_string(),
            });
        }

        // Ensure per-tree split/leaf sizes are consistent
        if self.min_leaf_size() >= self.min_split_size() {
            return Err(RootConfigError::InvalidParameter {
                parameter: "minLeafSize".to_string(),
                reason: format!(
                    "Configuration parameter 'minLeafSize' which was equal to {}, must be strictly smaller than configuration parameter 'minSplitSize' which was equal to {}",
                    self.min_leaf_size(),
                    self.min_split_size()
                ),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for crate::algo::kmeans::KMeansConfig {
    fn validate(&self) -> Result<(), RootConfigError> {
        if self.k == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "k".to_string(),
                reason: "k must be > 0".to_string(),
            });
        }
        if self.max_iterations == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "maxIterations".to_string(),
                reason: "maxIterations must be > 0".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&self.delta_threshold) {
            return Err(RootConfigError::InvalidParameter {
                parameter: "deltaThreshold".to_string(),
                reason: "deltaThreshold must be in [0.0, 1.0]".to_string(),
            });
        }
        if self.number_of_restarts == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "numberOfRestarts".to_string(),
                reason: "numberOfRestarts must be > 0".to_string(),
            });
        }
        if self.concurrency == 0 {
            return Err(RootConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be > 0".to_string(),
            });
        }
        if self.node_property.trim().is_empty() {
            return Err(RootConfigError::InvalidParameter {
                parameter: "nodeProperty".to_string(),
                reason: "nodeProperty cannot be empty".to_string(),
            });
        }
        Ok(())
    }
}
