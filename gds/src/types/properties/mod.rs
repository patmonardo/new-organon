// Base property system
pub mod property;
pub mod property_store;
pub mod property_values;

// Specialized property implementations
pub mod graph;
pub mod node;
pub mod relationship;

// Re-export base property system for convenience
pub use property::*;
pub use property_store::*;
pub use property_values::*;

// Note: avoid glob re-exporting `graph::*`, `node::*`, or `relationship::*` here
// because those submodules expose internal `impls`/`traits` submodules which
// can collide when re-exported into the parent namespace. Consumers should
// use the full module path, e.g. `crate::types::properties::graph::...`.
