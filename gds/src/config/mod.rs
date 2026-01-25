//! Configuration system for rust-gds
//!
//! This module provides a type-safe, builder-based configuration system
//! for graph construction, algorithms, and I/O operations.

pub mod algo_config;
pub mod base_types;
pub mod collections_config;
pub mod config_trait;
pub mod defaults;
pub mod graph_config;
pub mod graph_store_config;
pub mod io_config;
pub mod limits;
pub mod manager;
pub mod model_config;
pub mod pregel_config;
pub mod validation;

// Re-export core types for convenience
pub use algo_config::*;
pub use base_types::*;
pub use collections_config::*;
// Export only the trait name to avoid colliding with existing `Config` in base_types
pub use config_trait::ValidatedConfig;
pub use graph_config::*;
pub use graph_store_config::*;
pub use io_config::*;
pub use model_config::*;
pub use pregel_config::*;
pub use validation::*;

// Re-export new defaults/limits/manager types for top-level convenience
pub use defaults::*;
pub use limits::*;
pub use manager::*;
