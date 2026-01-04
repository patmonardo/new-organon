//! Algorithm processing timings (Java parity).

const NOT_AVAILABLE: i64 = -1;

#[derive(Debug, Clone, Copy)]
pub struct AlgorithmProcessingTimings {
    pub pre_processing_millis: i64,
    pub compute_millis: i64,
    pub side_effect_millis: i64,
}

impl AlgorithmProcessingTimings {
    pub fn unavailable() -> Self {
        Self {
            pre_processing_millis: NOT_AVAILABLE,
            compute_millis: NOT_AVAILABLE,
            side_effect_millis: NOT_AVAILABLE,
        }
    }
}

#[derive(Debug, Default)]
pub struct AlgorithmProcessingTimingsBuilder {
    pre_processing_millis: i64,
    compute_millis: i64,
    side_effect_millis: i64,
}

impl AlgorithmProcessingTimingsBuilder {
    pub fn new() -> Self {
        Self {
            pre_processing_millis: NOT_AVAILABLE,
            compute_millis: NOT_AVAILABLE,
            side_effect_millis: NOT_AVAILABLE,
        }
    }

    pub fn with_pre_processing_millis(&mut self, pre_processing_millis: i64) {
        self.pre_processing_millis = pre_processing_millis;
    }

    pub fn with_compute_millis(&mut self, compute_millis: i64) {
        self.compute_millis = compute_millis;
    }

    pub fn with_side_effect_millis(&mut self, side_effect_millis: i64) {
        self.side_effect_millis = side_effect_millis;
    }

    pub fn build(self) -> AlgorithmProcessingTimings {
        AlgorithmProcessingTimings {
            pre_processing_millis: self.pre_processing_millis,
            compute_millis: self.compute_millis,
            side_effect_millis: self.side_effect_millis,
        }
    }
}
