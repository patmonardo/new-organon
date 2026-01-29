//! Collections: Unified Data Structures for Graph Data Science
//!
//! This module provides a unified Collections API across multiple backends:
//! - **Huge**: Paged arrays for billions of elements
//! - **Vec**: Enhanced standard library vectors
//! - **Arrow**: Apache Arrow columnar arrays
//! - **Extensions**: ndarray, GPU, distributed, compression, encryption
//! - **Magic**: Auto-optimization, AI-powered features
//!
//! ## Architecture
//!
//! ```text
//! Application Layer (PropertyValues)
//!         ↓
//! Adapter Layer (UniversalPropertyValues)
//!         ↓
//! Collections Layer (Huge/Vec/Arrow/Extensions)
//! ```
//!
//! ## Usage
//!
//! ### Using the Prelude (Recommended)
//!
//! ```rust
//! use crate::collections::prelude::*;
//!
//! // All collections work the same way
//! let huge = HugeIntArray::new(1000);
//! let vec = VecInt::new();
//!
//! // Same API for all
//! let sum1 = huge.sum();
//! let sum2 = vec.sum();
//! ```
//!
//! ### Selective Import
//!
//! ```rust
//! use crate::collections::{HugeIntArray, VecInt, CollectionsBackend};
//! ```

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

// Dataset scaffolding
pub mod datasets;

// DataFrame integration
pub mod dataframe;

// Utilities
pub mod utils;

// Universal adapter
pub mod adapter;

// Re-export commonly used types (explicit to avoid ambiguous glob re-exports)
pub use adapter::{CollectionFactory, UniversalPropertyValues};
pub use catalog::*;
pub use dataframe::*;
// pub use datasets::*;
pub use traits::*;

// Utility modules and their common types (retain legacy paths)
pub use utils::array_util;
pub use utils::cursor;
pub use utils::page_util;
pub use utils::performance;
pub use utils::{ArrayUtil, PageUtil};

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
pub use bit_set::BitSet;
pub use huge_sparse_array::*;
pub use huge_sparse_list::*;
pub use indirect_comparator::*;
pub use long_multiset::LongMultiSet;
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
