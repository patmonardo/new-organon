//! Java: `Layer`.

use super::aggregator::Aggregator;

pub trait Layer: Send + Sync {
    fn sample_size(&self) -> usize;
    fn aggregator(&self) -> Box<dyn Aggregator>;
}
