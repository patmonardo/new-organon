//! Memory estimation and tracking system
//!
//! This module provides sophisticated memory estimation, tracking, and management
//! capabilities for graph data structures and algorithms.
//!
//! ## Core Components
//!
//! - **BitUtil** - Bit manipulation utilities (power of two, alignment, leading zeros)
//! - **Estimate** - Memory size calculations for data structures
//! - **MemoryRange** - Min/max byte ranges with arithmetic operations
//! - **MemoryEstimation** - Trait for components that can estimate memory usage
//! - **MemoryTree** - Tree-shaped memory descriptions for hierarchical estimation
//! - **Containers** - Track memory usage per user for graphs and tasks
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use gds::mem::*;
//!
//! // Calculate memory for an array
//! let array_size = Estimate::size_of_long_array(1_000_000);
//! println!("Array needs: {}", Estimate::human_readable(array_size));
//!
//! // Create a memory range
//! let range = MemoryRange::of(1024, 2048);
//! let doubled = range.times(2);
//!
//! // Track graph memory per user
//! let mut container = GraphStoreMemoryContainer::new();
//! container.add_graph("alice", "my-graph", 1024 * 1024 * 100);
//! ```

pub mod bit_util;
pub mod estimate;
pub mod graph_store_memory_container;
pub mod memest;
pub mod memory_estimation;
pub mod memory_range;
pub mod memory_reservation_exception;
pub mod memory_resident;
pub mod memory_tree;
pub mod task_memory_container;
pub mod user_entity_memory;
pub mod user_memory_summary;

// Re-export public API
pub use bit_util::BitUtil;
pub use estimate::Estimate;
pub use graph_store_memory_container::{
    GraphStoreAddedEvent, GraphStoreMemoryContainer, GraphStoreRemovedEvent,
};
pub use memest::MemoryEstimationResult;
pub use memory_estimation::{MemoryEstimation, MemoryEstimationWithDimensions};
pub use memory_range::MemoryRange;
pub use memory_reservation_exception::MemoryReservationExceededException;
pub use memory_resident::MemoryResident;
pub use memory_tree::{MemoryTree, MemoryTreeWithDimensions};
pub use task_memory_container::TaskMemoryContainer;
pub use user_entity_memory::UserEntityMemory;
pub use user_memory_summary::UserMemorySummary;

// Alias for applications system
pub type MemoryEstimateResult = MemoryEstimationResult;
