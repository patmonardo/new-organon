use crate::mem::{MemoryEstimation, MemoryEstimations};

pub struct LinkPredictionPipelineEstimator;

impl LinkPredictionPipelineEstimator {
    pub fn new() -> Self {
        Self
    }

    pub fn estimate(&self) -> Box<dyn MemoryEstimation> {
        MemoryEstimations::empty()
    }
}
