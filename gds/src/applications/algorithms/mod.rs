// NOTE: This module is an in-progress port of the Java Applications algorithm system.
// For now we only compile the pieces we actively use from the TS-JSON boundary:
// - `metadata` (stable algorithm identifiers)
// - `machinery` (progress tracker creator, later: memory guard, metrics, etc.)
//
// The remainder of the Java-parity facades (centrality/community/...) are present in the
// repo but not yet compiled/wired, to avoid pulling in unfinished placeholders.

// NOTE: This module is an in-progress port of the Java Applications algorithm system.
// For now we only compile the pieces we actively use from the TS-JSON boundary:
// - `metadata` (stable algorithm identifiers)
// - `machinery` (progress tracker creator, later: memory guard, metrics, etc.)
//
// The remainder of the Java-parity facades (centrality/community/...) are present in the
// repo but not yet compiled/wired, to avoid pulling in unfinished placeholders.

pub mod centrality;
pub mod community;
pub mod embeddings;
pub mod machine_learning;
pub mod machinery;
pub mod metadata;
pub mod miscellaneous;
pub mod pathfinding;
pub mod similarity;

pub use centrality::*;
pub use embeddings::*;
pub use machine_learning::*;
pub use machinery::*;
pub use metadata::*;
pub use miscellaneous::*;
pub use pathfinding::*;
pub use similarity::*;
