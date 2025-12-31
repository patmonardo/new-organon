//! Prelude - Common imports for procedure core
//!
//! Re-exports commonly used types and traits from the procedure core module.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gds::procedures::core::prelude::*;
//! ```

// Re-export result processing
pub use super::result::*;
pub use super::result_builders::{
    CentralityResult, CentralityResultBuilder, CommunityResult, CommunityResultBuilder,
    ExecutionMetadata, PathFindingResult, PathResult, PathResultBuilder, ResultBuilder,
    SimilarityResult, SimilarityResultBuilder,
};

// Re-export core systems (progress tracking and memory estimation)
pub use crate::core::utils::progress::*;
pub use crate::mem::*;
