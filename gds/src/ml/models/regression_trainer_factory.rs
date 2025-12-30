// Copyright (c) "Neo4j"
// Neo4j Sweden AB [http://neo4j.com]
//
// This file is part of Neo4j.
//
// Neo4j is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Regression Trainer Factory - 1:1 translation of RegressionTrainerFactory.java from Java GDS

use crate::concurrency::Concurrency;
use crate::core::utils::progress::ProgressTracker;
use crate::ml::models::{RegressorTrainer, TrainingMethod, base::TrainerConfigTrait};
use crate::concurrency::TerminationFlag;
use parking_lot::RwLock;
use std::sync::Arc;

/// Factory for creating regression trainers from configuration.
/// 1:1 translation of RegressionTrainerFactory.java from Java GDS.
pub struct RegressionTrainerFactory;

impl RegressionTrainerFactory {
    /// Create a regression trainer from configuration.
    /// 1:1 with RegressionTrainerFactory.create() in Java
    pub fn create(
        config: &dyn TrainerConfigTrait,
        termination_flag: &TerminationFlag,
        progress_tracker: &ProgressTracker,
        concurrency: &Concurrency,
        random_seed: Option<u64>,
    ) -> Box<dyn RegressorTrainer> {
        match config.method() {
            TrainingMethod::LinearRegression => {
                // Downcast to LinearRegressionTrainConfig
                let linear_config = (config as &dyn std::any::Any)
                    .downcast_ref::<crate::ml::models::linear::LinearRegressionTrainConfig>()
                    .expect("Invalid config type for LinearRegression");
                // Create dummy termination flag for now (not used in training)
                let dummy_termination = Arc::new(RwLock::new(false));
                Box::new(crate::ml::models::linear::LinearRegressionTrainer::new(
                    concurrency.value(),
                    linear_config.clone(),
                    dummy_termination,
                ))
            }
            TrainingMethod::RandomForestRegression => {
                // Downcast to RandomForestRegressorTrainerConfig
                let rf_config = (config as &dyn std::any::Any)
                    .downcast_ref::<crate::ml::models::trees::RandomForestRegressorTrainerConfig>()
                    .expect("Invalid config type for RandomForestRegression");
                Box::new(crate::ml::models::trees::RandomForestRegressorTrainer::new(
                    *concurrency,
                    rf_config.clone(),
                    random_seed,
                    termination_flag.clone(),
                    progress_tracker.clone(),
                    crate::projection::eval::procedure::LogLevel::Info, // Default log level
                ))
            }
            _ => panic!("No such training method for regression: {:?}", config.method()),
        }
    }
}
