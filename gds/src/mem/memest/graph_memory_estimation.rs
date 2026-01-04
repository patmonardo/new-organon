//! Graph memory estimation container.
//!
//! Java parity: this mirrors `org.neo4j.gds.memest.GraphMemoryEstimation`.
//! It pairs graph dimensions with a `MemoryEstimation` that can be evaluated
//! later (e.g. with a chosen concurrency).

use crate::core::graph_dimensions::ConcreteGraphDimensions;
use crate::mem::MemoryEstimation;

/// Memory estimation result for a graph.
///
/// Contains both the graph dimensions and the hierarchical memory tree
/// describing memory usage breakdown.
///
/// # Example
///
/// ```rust,ignore
/// use gds::mem::memest::GraphMemoryEstimation;
///
/// let estimation = GraphMemoryEstimation::new(dimensions, Box::new(my_estimation));
/// println!("Node count: {}", estimation.dimensions().node_count());
/// let tree = estimation
///     .estimate_memory_usage_after_loading()
///     .estimate(estimation.dimensions(), 4);
/// println!("Memory: {}", tree.render());
/// ```
pub struct GraphMemoryEstimation {
    dimensions: ConcreteGraphDimensions,
    estimate_memory_usage_after_loading: Box<dyn MemoryEstimation>,
}

impl GraphMemoryEstimation {
    /// Creates a new graph memory estimation.
    ///
    /// # Arguments
    ///
    /// * `dimensions` - The graph dimensions
    /// * `estimate_memory_usage_after_loading` - A deferred memory estimation
    pub fn new(
        dimensions: ConcreteGraphDimensions,
        estimate_memory_usage_after_loading: Box<dyn MemoryEstimation>,
    ) -> Self {
        Self {
            dimensions,
            estimate_memory_usage_after_loading,
        }
    }

    /// Returns a reference to the graph dimensions.
    pub fn dimensions(&self) -> &ConcreteGraphDimensions {
        &self.dimensions
    }

    /// Returns the deferred memory estimation.
    pub fn estimate_memory_usage_after_loading(&self) -> &dyn MemoryEstimation {
        &*self.estimate_memory_usage_after_loading
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::graph_dimensions::GraphDimensions;
    use crate::mem::{MemoryRange, MemoryTree};

    #[derive(Clone)]
    struct FixedEstimation;

    impl MemoryEstimation for FixedEstimation {
        fn description(&self) -> String {
            "fixed".to_string()
        }

        fn estimate(&self, _dimensions: &dyn GraphDimensions, _concurrency: usize) -> MemoryTree {
            MemoryTree::leaf("fixed".to_string(), MemoryRange::of(1024))
        }
    }

    #[test]
    fn test_graph_memory_estimation_creation() {
        let dimensions = ConcreteGraphDimensions::of(1000, 5000);
        let estimation = GraphMemoryEstimation::new(dimensions, Box::new(FixedEstimation));

        assert_eq!(estimation.dimensions().node_count(), 1000);
        let tree = estimation
            .estimate_memory_usage_after_loading()
            .estimate(estimation.dimensions(), 4);
        assert_eq!(tree.memory_usage().min(), 1024);
    }
}
