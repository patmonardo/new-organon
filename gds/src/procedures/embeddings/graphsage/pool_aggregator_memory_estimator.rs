//! Java: `PoolAggregatorMemoryEstimator`.

use crate::mem::{Estimate, MemoryRange};

use super::aggregator_memory_estimator::AggregatorMemoryEstimator;

pub struct PoolAggregatorMemoryEstimator;

impl AggregatorMemoryEstimator for PoolAggregatorMemoryEstimator {
    fn estimate(
        &self,
        min_node_count: u64,
        max_node_count: u64,
        min_previous_node_count: u64,
        max_previous_node_count: u64,
        _input_dimension: usize,
        embedding_dimension: usize,
    ) -> MemoryRange {
        let min_bound =
            3 * Estimate::size_of_double_array(
                min_previous_node_count as usize * embedding_dimension,
            ) + 6 * Estimate::size_of_double_array(min_node_count as usize * embedding_dimension);
        let max_bound =
            3 * Estimate::size_of_double_array(
                max_previous_node_count as usize * embedding_dimension,
            ) + 6 * Estimate::size_of_double_array(max_node_count as usize * embedding_dimension);
        MemoryRange::of_range(min_bound, max_bound)
    }
}
