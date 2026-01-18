//! Core utilities for rust-gds.
//!
//! This module provides foundational utilities used throughout the library:
//! - Time and clock services
//! - Progress tracking and timing
//! - Partitioning strategies
//! - Queue implementations
//! - Data shuffling utilities
//! - Bit manipulation and raw value encoding
//! - Set intersections and vector similarity operations
//! - Cache-efficient binary search (Eytzinger layout)
//! - Lazy batch collection for parallel processing

pub mod array_layout;
pub mod ascending_long_comparator;
pub mod clock_service;
pub mod intersections;
pub mod lazy_batch_collection;
pub mod mapped_id_node_property_values;
pub mod original_id_node_property_values;
pub mod progress_timer;
pub mod time_util;

pub mod paged;
pub mod partition;
pub mod progress;
pub mod queue;
pub mod set_bits_iterable;
pub mod shuffle;
pub mod string_formatting;
pub mod two_arrays_sort;
pub mod warnings;

// Re-exports for convenience
pub use array_layout::{ArrayLayout, LayoutAndSecondary};
pub use ascending_long_comparator::AscendingLongComparator;
pub use clock_service::{Clock, ClockService};
pub use intersections::Intersections;
pub use lazy_batch_collection::LazyBatchCollection;
pub use mapped_id_node_property_values::MappedIdNodePropertyValues;
pub use original_id_node_property_values::OriginalIdNodePropertyValues;
pub use progress_timer::ProgressTimer;
pub use set_bits_iterable::SetBitsIterable;
pub use shuffle::{Random as ShuffleRandom, ShuffleUtil, SplittableRandom};
pub use string_formatting::{
    format_number, format_with_locale, is_empty, to_lower_case_with_locale,
    to_upper_case_with_locale,
};
pub use time_util::{TimeUtil, ZoneId};
pub use two_arrays_sort::{
    is_correctly_sorted, sort_double_array_by_long_values, sort_double_array_by_long_values_desc,
    sort_long_array_by_double_values,
};
