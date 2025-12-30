//! Partitioning utilities for parallel graph processing.
//!
//! This module provides various partitioning strategies for dividing graph workloads
//! across multiple threads or workers:
//!
//! - **Range partitioning**: Simple node ID-based splits
//! - **Degree partitioning**: Relationship-aware load balancing
//! - **Iterator partitioning**: Flexible custom partitioning schemes

pub mod degree_partition;
pub mod iterator_partition;
pub mod lazy_degree_partition_iterator;
#[allow(clippy::module_inception)]
pub mod partition;
pub mod partition_consumer;
pub mod partition_utils;
pub mod partitioning;

pub use degree_partition::DegreePartition;
pub use iterator_partition::IteratorPartition;
pub use lazy_degree_partition_iterator::LazyDegreePartitionIterator;
pub use partition::Partition;
pub use partition_consumer::PartitionConsumer;
pub use partition_utils::{PartitionUtils, DEFAULT_BATCH_SIZE};
pub use partitioning::Partitioning;

/// Function interface for getting node degrees.
///
/// Used by partitioning algorithms to determine how to split work
/// based on the degree distribution of the graph.
pub trait DegreeFunction: Send + Sync {
    /// Returns the degree (number of relationships) for a node.
    ///
    /// # Arguments
    /// * `node` - The node ID
    ///
    /// # Returns
    /// The degree of the node
    fn degree(&self, node: usize) -> usize;
}

// Implement DegreeFunction for closures
impl<F> DegreeFunction for F
where
    F: Fn(usize) -> usize + Send + Sync,
{
    fn degree(&self, node: usize) -> usize {
        self(node)
    }
}
