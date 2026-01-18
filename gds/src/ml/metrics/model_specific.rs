use super::{Metric, MetricComparator};

/// Out-of-bag error metric for Random Forest models.
///
/// This metric is model-specific and only applicable to Random Forest training.
#[derive(Debug, Clone, Default)]
pub struct OutOfBagError;

impl OutOfBagError {
    pub const NAME: &'static str = "OUT_OF_BAG_ERROR";

    pub fn new() -> Self {
        Self
    }
}

impl Metric for OutOfBagError {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Natural
    }

    fn is_model_specific(&self) -> bool {
        true
    }
}
