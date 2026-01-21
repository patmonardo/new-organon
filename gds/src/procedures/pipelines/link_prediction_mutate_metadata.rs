use crate::procedures::pipelines::types::AnyMap;

#[derive(Debug, Clone)]
pub struct LinkPredictionMutateMetadata {
    relationships_written: i64,
    probability_distribution: AnyMap,
}

impl LinkPredictionMutateMetadata {
    pub fn new(relationships_written: i64, probability_distribution: AnyMap) -> Self {
        Self {
            relationships_written,
            probability_distribution,
        }
    }

    pub fn relationships_written(&self) -> i64 {
        self.relationships_written
    }

    pub fn probability_distribution(&self) -> &AnyMap {
        &self.probability_distribution
    }
}
