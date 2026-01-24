use crate::ml::decision_tree::DecisionTreePredictor;
use crate::ml::models::BaseModelData;
use crate::ml::models::Regressor;
use crate::ml::models::RegressorData;
use crate::ml::models::TrainingMethod;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

use super::config::RandomForestConfig;

/// Random forest regressor model.
/// 1:1 translation of RandomForestRegressor.java from Java GDS.
pub struct RandomForestRegressor {
    data: RandomForestRegressorData,
}

impl RandomForestRegressor {
    pub fn new(data: RandomForestRegressorData) -> Self {
        Self { data }
    }

    pub fn num_trees(&self) -> usize {
        self.data.decision_trees.len()
    }
}

impl Regressor for RandomForestRegressor {
    fn data(&self) -> &dyn RegressorData {
        &self.data
    }

    /// Predict a single value for given features
    /// 1:1 with Regressor.predict(double[] features) in Java
    fn predict(&self, features: &[f64]) -> f64 {
        let number_of_decision_trees = self.data.decision_trees.len();
        if number_of_decision_trees == 0 {
            return 0.0;
        }

        let mut sum = 0.0;
        for i in 0..number_of_decision_trees {
            sum += *self.data.decision_trees[i].predict(features);
        }

        sum / number_of_decision_trees as f64
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Random Forest Regressor Data.
/// 1:1 translation of RandomForestRegressorData.java from Java GDS.
pub struct RandomForestRegressorData {
    pub decision_trees: Vec<Box<DecisionTreePredictor<f64>>>,
    pub num_features: usize,
}

impl fmt::Debug for RandomForestRegressor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RandomForestRegressor")
            .field("num_trees", &self.data.decision_trees.len())
            .field("num_features", &self.data.num_features)
            .finish()
    }
}

impl fmt::Debug for RandomForestRegressorData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RandomForestRegressorData")
            .field("num_trees", &self.decision_trees.len())
            .field("num_features", &self.num_features)
            .finish()
    }
}

impl BaseModelData for RandomForestRegressorData {
    fn trainer_method(&self) -> TrainingMethod {
        TrainingMethod::RandomForestRegression
    }

    fn feature_dimension(&self) -> usize {
        self.num_features
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl RegressorData for RandomForestRegressorData {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomForestRegressorConfig {
    #[serde(flatten)]
    pub forest: RandomForestConfig,
}
