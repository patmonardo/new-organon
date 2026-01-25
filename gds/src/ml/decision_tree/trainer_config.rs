//! Configuration for decision tree trainers.
//!
//! Note: the `max_depth` configuration uses `0` to mean "unlimited". This is
//! harmonized with `RandomForestConfig` semantics and reflected across validation
//! and runtime logic in the repository.
//!
//! Translated from Java GDS ml-algo DecisionTreeTrainerConfig.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::config::validation::ConfigError;

#[derive(Debug, Clone)]
pub struct DecisionTreeTrainerConfig {
    max_depth: usize,
    min_split_size: usize,
    min_leaf_size: usize,
}

impl DecisionTreeTrainerConfig {
    pub fn builder() -> DecisionTreeTrainerConfigBuilder {
        DecisionTreeTrainerConfigBuilder::default()
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    pub fn min_split_size(&self) -> usize {
        self.min_split_size
    }

    pub fn min_leaf_size(&self) -> usize {
        self.min_leaf_size
    }

    fn validate(&self) -> Result<(), String> {
        if self.min_leaf_size >= self.min_split_size {
            return Err(format!(
                "Configuration parameter 'minLeafSize' which was equal to {}, must be strictly smaller than configuration parameter 'minSplitSize' which was equal to {}",
                self.min_leaf_size,
                self.min_split_size
            ));
        }
        Ok(())
    }

    pub fn validate_config(&self) -> Result<(), ConfigError> {
        if !crate::ml::decision_tree::is_unlimited_depth(self.max_depth) {
            if self.max_depth < 1 {
                return Err(ConfigError::InvalidParameter {
                    parameter: "maxDepth".to_string(),
                    reason: "maxDepth must be >= 1 when set explicitly".to_string(),
                });
            }
            if self.max_depth > 10_000 {
                return Err(ConfigError::InvalidParameter {
                    parameter: "maxDepth".to_string(),
                    reason: "maxDepth must be in [1, 10_000] when set explicitly".to_string(),
                });
            }
        }

        if self.min_split_size < 2 || self.min_split_size > 1_000_000 {
            return Err(ConfigError::InvalidParameter {
                parameter: "minSplitSize".to_string(),
                reason: "minSplitSize must be >= 2 and <= 1_000_000".to_string(),
            });
        }
        if self.min_leaf_size < 1 || self.min_leaf_size > 1_000_000 {
            return Err(ConfigError::InvalidParameter {
                parameter: "minLeafSize".to_string(),
                reason: "minLeafSize must be >= 1 and <= 1_000_000".to_string(),
            });
        }
        if self.min_leaf_size >= self.min_split_size {
            return Err(ConfigError::InvalidParameter {
                parameter: "minLeafSize".to_string(),
                reason: format!(
                    "Configuration parameter 'minLeafSize' which was equal to {}, must be strictly smaller than configuration parameter 'minSplitSize' which was equal to {}",
                    self.min_leaf_size,
                    self.min_split_size
                ),
            });
        }

        Ok(())
    }
}

impl crate::config::ValidatedConfig for DecisionTreeTrainerConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        DecisionTreeTrainerConfig::validate_config(self)
    }
}

impl Default for DecisionTreeTrainerConfig {
    fn default() -> Self {
        Self {
            // 0 now means unlimited (harmonized with RandomForest)
            max_depth: 0,
            min_split_size: 2,
            min_leaf_size: 1,
        }
    }
}

#[derive(Default)]
pub struct DecisionTreeTrainerConfigBuilder {
    max_depth: Option<usize>,
    min_split_size: Option<usize>,
    min_leaf_size: Option<usize>,
}

impl DecisionTreeTrainerConfigBuilder {
    pub fn max_depth(mut self, max_depth: usize) -> Self {
        // allow 0 as 'unlimited' to be consistent with RandomForestConfig
        assert!(
            crate::ml::decision_tree::is_unlimited_depth(max_depth) || max_depth >= 1,
            "maxDepth must be 0 (unlimited) or >= 1"
        );
        self.max_depth = Some(max_depth);
        self
    }

    pub fn min_split_size(mut self, min_split_size: usize) -> Self {
        assert!(min_split_size >= 2, "minSplitSize must be at least 2");
        self.min_split_size = Some(min_split_size);
        self
    }

    pub fn min_leaf_size(mut self, min_leaf_size: usize) -> Self {
        assert!(min_leaf_size >= 1, "minLeafSize must be at least 1");
        self.min_leaf_size = Some(min_leaf_size);
        self
    }

    pub fn build(self) -> Result<DecisionTreeTrainerConfig, String> {
        let config = DecisionTreeTrainerConfig {
            max_depth: self.max_depth.unwrap_or(0),
            min_split_size: self.min_split_size.unwrap_or(2),
            min_leaf_size: self.min_leaf_size.unwrap_or(1),
        };
        config.validate()?;
        Ok(config)
    }
}
