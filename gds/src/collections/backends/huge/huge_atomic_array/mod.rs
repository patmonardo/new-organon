pub mod huge_atomic_double_array;
/// Huge atomic arrays supporting thread-safe operations on massive datasets.
///
/// This module provides atomic variants of huge arrays intended for concurrent access
/// patterns common in parallel graph algorithms.
///
/// Element-wise operations are atomic and thread-safe; bulk initialization helpers like
/// `set_all` require exclusive access (no concurrent readers/writers).
pub mod huge_atomic_long_array;

pub use huge_atomic_double_array::HugeAtomicDoubleArray;
pub use huge_atomic_long_array::HugeAtomicLongArray;
