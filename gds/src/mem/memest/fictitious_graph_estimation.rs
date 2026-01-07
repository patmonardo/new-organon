//! Fictitious graph store estimation service.
//!
//! Estimates memory usage for hypothetical graph projections with specified dimensions.
//! Unlike database-backed estimation, this doesn't require a live data source.

use super::GraphMemoryEstimation;
use crate::core::graph_dimensions::{ConcreteGraphDimensions, GraphDimensions};
use crate::mem::{MemoryEstimation, MemoryRange, MemoryTree};

/// Service for estimating memory usage of hypothetical graph projections.
///
/// This service creates memory estimations for graphs with specified dimensions,
/// without requiring a live database connection. Useful for capacity planning
/// and "what-if" analysis.
///
/// # Example
///
/// ```rust,ignore
/// use gds::mem::memest::FictitiousGraphEstimationService;
///
/// let service = FictitiousGraphEstimationService;
///
/// // Estimate for a graph with 1M nodes and 5M relationships
/// let estimation = service.estimate(1_000_000, 5_000_000);
///
/// let tree = estimation
///     .estimate_memory_usage_after_loading()
///     .estimate(estimation.dimensions(), 4);
/// println!("Min memory: {} bytes", tree.memory_usage().min());
/// println!("Max memory: {} bytes", tree.memory_usage().max());
/// ```
pub struct FictitiousGraphEstimationService;

impl FictitiousGraphEstimationService {
    /// Creates a new fictitious graph estimation service.
    pub fn new() -> Self {
        Self
    }

    /// Estimates memory usage for a graph with specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `node_count` - Number of nodes in the graph
    /// * `relationship_count` - Number of relationships in the graph
    ///
    /// # Returns
    ///
    /// Memory estimation for the graph
    pub fn estimate(&self, node_count: usize, relationship_count: usize) -> GraphMemoryEstimation {
        let dimensions = ConcreteGraphDimensions::of(node_count, relationship_count);
        let estimation = Box::new(FictitiousGraphStoreEstimation {
            mode: FictitiousMode::Simple,
            node_label_count: 0,
            property_count: 0,
        });

        GraphMemoryEstimation::new(dimensions, estimation)
    }

    /// Estimates memory usage with detailed configuration.
    ///
    /// # Arguments
    ///
    /// * `node_count` - Number of nodes in the graph
    /// * `relationship_count` - Number of relationships in the graph
    /// * `node_label_count` - Number of distinct node labels (0 if all nodes have same label)
    /// * `property_count` - Estimated number of properties per element
    ///
    /// # Returns
    ///
    /// Memory estimation for the graph
    pub fn estimate_detailed(
        &self,
        node_count: usize,
        relationship_count: usize,
        node_label_count: usize,
        property_count: usize,
    ) -> GraphMemoryEstimation {
        let dimensions = ConcreteGraphDimensions::of(node_count, relationship_count);
        let estimation = Box::new(FictitiousGraphStoreEstimation {
            mode: FictitiousMode::Detailed,
            node_label_count,
            property_count,
        });

        GraphMemoryEstimation::new(dimensions, estimation)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FictitiousMode {
    Simple,
    Detailed,
}

#[derive(Clone, Debug)]
struct FictitiousGraphStoreEstimation {
    mode: FictitiousMode,
    node_label_count: usize,
    property_count: usize,
}

impl MemoryEstimation for FictitiousGraphStoreEstimation {
    fn description(&self) -> String {
        "Graph Store".to_string()
    }

    fn estimate(&self, dimensions: &dyn GraphDimensions, _concurrency: usize) -> MemoryTree {
        match self.mode {
            FictitiousMode::Simple => estimate_memory_tree_simple(dimensions),
            FictitiousMode::Detailed => estimate_memory_tree_detailed(
                dimensions,
                self.node_label_count,
                self.property_count,
            ),
        }
    }
}

fn estimate_memory_tree_simple(dimensions: &dyn GraphDimensions) -> MemoryTree {
    let node_count = dimensions.node_count();
    let rel_count = dimensions.rel_count_upper_bound();

    let node_memory = node_count * 32;
    let rel_memory = rel_count * 24;
    let adjacency_memory = rel_count * 16;
    let overhead = (node_memory + rel_memory + adjacency_memory) / 10;
    let total_memory = node_memory + rel_memory + adjacency_memory + overhead;

    let children = vec![
        MemoryTree::leaf("Node Storage".to_string(), MemoryRange::of(node_memory)),
        MemoryTree::leaf(
            "Relationship Storage".to_string(),
            MemoryRange::of(rel_memory),
        ),
        MemoryTree::leaf(
            "Adjacency Lists".to_string(),
            MemoryRange::of(adjacency_memory),
        ),
        MemoryTree::leaf("Metadata Overhead".to_string(), MemoryRange::of(overhead)),
    ];

    MemoryTree::new(
        "Graph Store".to_string(),
        MemoryRange::of(total_memory),
        children,
    )
}

fn estimate_memory_tree_detailed(
    dimensions: &dyn GraphDimensions,
    node_label_count: usize,
    property_count: usize,
) -> MemoryTree {
    let node_count = dimensions.node_count();
    let rel_count = dimensions.rel_count_upper_bound();

    let node_memory = node_count * 32;
    let rel_memory = rel_count * 24;
    let adjacency_memory = rel_count * 16;

    let label_memory = if node_label_count > 0 {
        node_count * node_label_count * 8
    } else {
        0
    };

    let node_property_memory = node_count * property_count * 16;
    let rel_property_memory = rel_count * property_count * 16;

    let base_overhead = (node_memory + rel_memory + adjacency_memory) / 10;
    let property_overhead = (node_property_memory + rel_property_memory) / 20;
    let overhead_total = base_overhead + property_overhead;

    let mut total_memory =
        node_memory + rel_memory + adjacency_memory + label_memory + overhead_total;
    if node_property_memory > 0 || rel_property_memory > 0 {
        total_memory += node_property_memory + rel_property_memory;
    }

    let mut children = Vec::new();
    children.push(MemoryTree::leaf(
        "Node Storage".to_string(),
        MemoryRange::of(node_memory),
    ));
    children.push(MemoryTree::leaf(
        "Relationship Storage".to_string(),
        MemoryRange::of(rel_memory),
    ));
    children.push(MemoryTree::leaf(
        "Adjacency Lists".to_string(),
        MemoryRange::of(adjacency_memory),
    ));

    if label_memory > 0 {
        children.push(MemoryTree::leaf(
            "Label Storage".to_string(),
            MemoryRange::of(label_memory),
        ));
    }

    if node_property_memory > 0 || rel_property_memory > 0 {
        let mut property_children = Vec::new();
        if node_property_memory > 0 {
            property_children.push(MemoryTree::leaf(
                "Node Properties".to_string(),
                MemoryRange::of(node_property_memory),
            ));
        }
        if rel_property_memory > 0 {
            property_children.push(MemoryTree::leaf(
                "Relationship Properties".to_string(),
                MemoryRange::of(rel_property_memory),
            ));
        }

        let property_total = node_property_memory + rel_property_memory;
        children.push(MemoryTree::new(
            "Property Storage".to_string(),
            MemoryRange::of(property_total),
            property_children,
        ));
    }

    children.push(MemoryTree::leaf(
        "Metadata Overhead".to_string(),
        MemoryRange::of(overhead_total),
    ));

    MemoryTree::new(
        "Graph Store".to_string(),
        MemoryRange::of(total_memory),
        children,
    )
}

impl Default for FictitiousGraphEstimationService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::graph_dimensions::GraphDimensions;

    #[test]
    fn test_simple_estimation() {
        let service = FictitiousGraphEstimationService::new();
        let estimation = service.estimate(1000, 5000);

        assert_eq!(estimation.dimensions().node_count(), 1000);
        assert_eq!(estimation.dimensions().rel_count_upper_bound(), 5000);
        let tree = estimation
            .estimate_memory_usage_after_loading()
            .estimate(estimation.dimensions(), 4);
        assert!(tree.memory_usage().min() > 0);
    }

    #[test]
    fn test_detailed_estimation() {
        let service = FictitiousGraphEstimationService::new();
        let estimation = service.estimate_detailed(1000, 5000, 2, 3);

        assert_eq!(estimation.dimensions().node_count(), 1000);
        let detailed_tree = estimation
            .estimate_memory_usage_after_loading()
            .estimate(estimation.dimensions(), 4);
        assert!(detailed_tree.memory_usage().min() > 0);

        // Detailed estimation should include label and property overhead
        let simple_estimation = service.estimate(1000, 5000);
        let simple_tree = simple_estimation
            .estimate_memory_usage_after_loading()
            .estimate(simple_estimation.dimensions(), 4);
        assert!(detailed_tree.memory_usage().min() > simple_tree.memory_usage().min());
    }

    #[test]
    fn test_memory_scaling() {
        let service = FictitiousGraphEstimationService::new();

        let small = service.estimate(100, 500);
        let large = service.estimate(1000, 5000);

        let small_tree = small
            .estimate_memory_usage_after_loading()
            .estimate(small.dimensions(), 4);
        let large_tree = large
            .estimate_memory_usage_after_loading()
            .estimate(large.dimensions(), 4);

        // Memory should scale roughly linearly with size
        assert!(large_tree.memory_usage().min() > small_tree.memory_usage().min() * 5);
        assert!(large_tree.memory_usage().min() < small_tree.memory_usage().min() * 15);
    }

    #[test]
    fn test_zero_relationships() {
        let service = FictitiousGraphEstimationService::new();
        let estimation = service.estimate(1000, 0);

        assert_eq!(estimation.dimensions().node_count(), 1000);
        assert_eq!(estimation.dimensions().rel_count_upper_bound(), 0);
        let tree = estimation
            .estimate_memory_usage_after_loading()
            .estimate(estimation.dimensions(), 4);
        assert!(tree.memory_usage().min() > 0); // Still has node storage overhead
    }

    #[test]
    fn test_memory_tree_structure() {
        let service = FictitiousGraphEstimationService::new();
        let estimation = service.estimate(1000, 5000);

        let tree = estimation
            .estimate_memory_usage_after_loading()
            .estimate(estimation.dimensions(), 4);
        let description = tree.description();

        // Should have root node
        assert_eq!(description, "Graph Store");
    }

    #[test]
    fn test_detailed_with_properties() {
        let service = FictitiousGraphEstimationService::new();

        // Without properties
        let without_props = service.estimate_detailed(1000, 5000, 0, 0);

        // With properties
        let with_props = service.estimate_detailed(1000, 5000, 0, 5);

        let without_props_tree = without_props
            .estimate_memory_usage_after_loading()
            .estimate(without_props.dimensions(), 4);
        let with_props_tree = with_props
            .estimate_memory_usage_after_loading()
            .estimate(with_props.dimensions(), 4);

        // Properties should add significant overhead
        assert!(with_props_tree.memory_usage().min() > without_props_tree.memory_usage().min());
    }

    #[test]
    fn test_detailed_with_labels() {
        let service = FictitiousGraphEstimationService::new();

        // Without labels
        let without_labels = service.estimate_detailed(1000, 5000, 0, 0);

        // With labels
        let with_labels = service.estimate_detailed(1000, 5000, 3, 0);

        let without_labels_tree = without_labels
            .estimate_memory_usage_after_loading()
            .estimate(without_labels.dimensions(), 4);
        let with_labels_tree = with_labels
            .estimate_memory_usage_after_loading()
            .estimate(with_labels.dimensions(), 4);

        // Labels should add overhead
        assert!(with_labels_tree.memory_usage().min() > without_labels_tree.memory_usage().min());
    }
}
