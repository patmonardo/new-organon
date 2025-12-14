// Atomic aggregators for parallel algorithm operations.
//
// This module provides atomic types for concurrent aggregation operations
// commonly needed in parallel graph algorithms. Implementations use
// Compare-And-Swap (CAS) loops over standard atomics.

mod atomic_double;
mod atomic_max;
mod atomic_min;
// mod double_adder;  // Coming soon - striped accumulator pattern
// mod long_adder;    // Coming soon - striped accumulator pattern

pub use atomic_double::AtomicDouble;
pub use atomic_max::AtomicMax;
pub use atomic_min::AtomicMin;
// pub use double_adder::DoubleAdder;
// pub use long_adder::LongAdder;
