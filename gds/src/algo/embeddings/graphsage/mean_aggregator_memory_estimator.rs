//! Java: `MeanAggregatorMemoryEstimator`.

use crate::mem::{Estimate, MemoryRange};

use super::aggregator_memory_estimator::AggregatorMemoryEstimator;

pub struct MeanAggregatorMemoryEstimator;

impl AggregatorMemoryEstimator for MeanAggregatorMemoryEstimator {
    fn estimate(
        &self,
        min_node_count: u64,
        max_node_count: u64,
        _min_previous_node_count: u64,
        _max_previous_node_count: u64,
        input_dimension: usize,
        embedding_dimension: usize,
    ) -> MemoryRange {
        let min_bound = Estimate::size_of_double_array(min_node_count as usize * input_dimension)
            + 2 * Estimate::size_of_double_array(min_node_count as usize * embedding_dimension);
        let max_bound = Estimate::size_of_double_array(max_node_count as usize * input_dimension)
            + 2 * Estimate::size_of_double_array(max_node_count as usize * embedding_dimension);
        MemoryRange::of_range(min_bound, max_bound)
    }
}
