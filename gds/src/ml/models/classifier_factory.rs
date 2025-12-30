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

//! Classifier Factory - 1:1 translation of ClassifierFactory.java from Java GDS

use crate::mem::{MemoryEstimation, MemoryRange};
use crate::ml::models::{Classifier, ClassifierData, TrainingMethod};
use crate::ml::models::neural::{MLPClassifier, MLPClassifierData};

/// Factory for creating classifiers from trained model data.
/// 1:1 translation of ClassifierFactory.java from Java GDS.
pub struct ClassifierFactory;

impl ClassifierFactory {
    /// Create a classifier from trained model data.
    /// 1:1 with ClassifierFactory.create(Classifier.ClassifierData) in Java
    pub fn create(classifier_data: &dyn ClassifierData) -> Box<dyn Classifier> {
        match classifier_data.trainer_method() {
            TrainingMethod::LogisticRegression => {
                // In Java: LogisticRegressionClassifier.from((LogisticRegressionData) classifierData)
                // We need to downcast to the specific data type
                todo!("Need to implement downcasting for LogisticRegressionData")
            }
            TrainingMethod::RandomForestClassification => {
                // In Java: new RandomForestClassifier((RandomForestClassifierData) classifierData)
                todo!("Need to implement downcasting for RandomForestClassifierData")
            }
            TrainingMethod::MLPClassification => {
                // In Java: new MLPClassifier((MLPClassifierData) classifierData)
                let mlp_data = classifier_data
                    .as_any()
                    .downcast_ref::<MLPClassifierData>()
                    .expect("Invalid ClassifierData type for MLPClassification");
                Box::new(MLPClassifier::new(mlp_data.clone()))
            }
            _ => panic!("No such classifier for training method: {:?}", classifier_data.trainer_method()),
        }
    }

    /// Estimate runtime memory overhead for predictions.
    /// 1:1 with ClassifierFactory.runtimeOverheadMemoryEstimation() in Java
    pub fn runtime_overhead_memory_estimation(
        method: TrainingMethod,
        _batch_size: usize,
        _number_of_classes: usize,
        _feature_dimension: usize,
        _is_reduced: bool,
    ) -> MemoryRange {
        match method {
            TrainingMethod::LogisticRegression => {
                // TODO: Implement logistic regression memory estimation
                MemoryRange::empty()
            }
            TrainingMethod::RandomForestClassification => {
                // TODO: Implement RandomForest runtime overhead memory estimation
                MemoryRange::empty()
            }
            TrainingMethod::MLPClassification => {
                // TODO: Implement MLP memory estimation
                MemoryRange::empty()
            }
            _ => panic!("No such classifier for training method: {:?}", method),
        }
    }

    /// Estimate memory for trained model data.
    /// 1:1 with ClassifierFactory.dataMemoryEstimation() in Java
    pub fn data_memory_estimation(
        trainer_config: &dyn crate::ml::models::base::TrainerConfigTrait,
        _number_of_training_samples: impl Fn(u64) -> u64,
        _number_of_classes: usize,
        _feature_dimension: MemoryRange,
        _is_reduced: bool,
    ) -> Box<dyn MemoryEstimation> {
        match trainer_config.method() {
            TrainingMethod::LogisticRegression => {
                todo!("Logistic regression data memory estimation")
            }
            TrainingMethod::RandomForestClassification => {
                todo!("RandomForest data memory estimation")
            }
            TrainingMethod::MLPClassification => {
                todo!("MLP data memory estimation")
            }
            _ => panic!("No such classifier for training method: {:?}", trainer_config.method()),
        }
    }
}
