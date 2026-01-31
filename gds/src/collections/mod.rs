//! Collections: Unified Data Structures for Graph Data Science
//!
//! This module provides a unified Collections API across multiple backends:
//! - **Huge**: Paged arrays for billions of elements
//! - **Vec**: Enhanced standard library vectors
//! - **Arrow**: Apache Arrow columnar arrays
//! - **Extensions**: ndarray, GPU, distributed, compression, encryption
//! - **Magic**: Auto-optimization, AI-powered features
//!
//!

// Prelude - curated exports for common use
pub mod prelude;

// Core traits
pub mod traits;

// Backend implementations
pub mod backends;

// Extension implementations
pub mod extensions;

// Disk-first catalog
pub mod catalog;

// Dataset tooling (dataset registry + expr facade)
pub mod datasets;

// DataFrame integration
pub mod dataframe;

// Plugins
pub mod plugins;

// Utilities
pub mod utils;

// Universal adapter
pub mod adapter;

pub use adapter::*;
pub use catalog::*;
pub use dataframe::*;
pub use plugins::*;
pub use traits::*;

// Utility modules and their common types (retain legacy paths)
pub use utils::*;

#[cfg(feature = "arrow")]
pub use backends::arrow::{
    ArrowArrayBehavior, ArrowDoubleArray, ArrowFloatArray, ArrowIntArray, ArrowLongArray,
    ArrowPrimitiveArray,
};
pub use backends::huge::{
    HugeAtomicDoubleArray, HugeAtomicLongArray, HugeBooleanArray, HugeByteArray, HugeCharArray,
    HugeDoubleArray, HugeFloatArray, HugeIntArray, HugeLongArray, HugeObjectArray, HugeShortArray,
};
pub use backends::vec::{
    EnhancedVec, VecBoolean, VecByte, VecChar, VecDouble, VecDoubleArray, VecFloat, VecFloatArray,
    VecInt, VecLong, VecLongArray, VecShort,
};

// Re-export legacy modules for backward compatibility
pub mod bit_set;
pub mod huge_sparse_array;
pub mod huge_sparse_list;
pub mod indirect_comparator;
pub mod long_multiset;
pub mod primitive;

// Re-export types from core for backward compatibility
pub use crate::core::utils::paged::HugeAtomicBitSet;

// Re-export BitSet and HugeSparseLongArray for backward compatibility
pub use bit_set::*;
pub use huge_sparse_array::*;
pub use huge_sparse_list::*;
pub use indirect_comparator::*;
pub use long_multiset::*;
pub use primitive::*;

// Backend selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CollectionsBackend {
    Huge, // Paged arrays
    #[default]
    Vec, // Enhanced vectors
    Arrow, // Apache Arrow
    Std,  // Standard library
}
