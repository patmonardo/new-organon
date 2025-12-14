//! Thread-safe builder for huge long arrays.
//!
//! This is used for dynamic array construction during graph loading.
//! The implementation is intentionally **safe and lock-based**:
//! - pages are stored in a `RwLock<Vec<Vec<i64>>>`
//! - growth and writes acquire the lock to ensure correctness
//!
//! This trades some write concurrency for eliminating UB-prone pointer/atomic patterns.
//!
//! # Use Cases
//!
//! - Dynamic node ID array construction during loading
//! - Streaming graph data ingestion with unknown sizes
//! - Concurrent property array building
//! - Bulk data insertion with parallel workers
//! - Memory-efficient large array construction
//!
//! # Examples
//!
//! ```
//! use gds::core::utils::paged::HugeLongArrayBuilder;
//! use std::sync::Arc;
//! use std::thread;
//!
//! // Create builder
//! let builder = Arc::new(HugeLongArrayBuilder::new());
//!
//! // Concurrent filling from multiple workers
//! let handles: Vec<_> = (0..4).map(|worker_id| {
//!     let builder_clone = Arc::clone(&builder);
//!     thread::spawn(move || {
//!         let start = worker_id * 100_000;
//!         let data: Vec<i64> = (start..start + 100_000).map(|i| i as i64).collect();
//!
//!         // Direct, thread-safe writing
//!         builder_clone.write_range(start, &data);
//!     })
//! }).collect();
//!
//! // Wait for all workers
//! for handle in handles {
//!     handle.join().unwrap();
//! }
//!
//! // Build final array
//! let array = builder.build(400_000);
//! assert_eq!(array.get(0), 0);
//! assert_eq!(array.get(399_999), 399_999);
//! ```

use crate::collections::{HugeLongArray, PageUtil};
use std::sync::RwLock;

const LONG_PAGE_SIZE: usize = PageUtil::PAGE_SIZE_32KB / std::mem::size_of::<i64>();
const LONG_PAGE_MASK: usize = LONG_PAGE_SIZE - 1;
const LONG_PAGE_SHIFT: u32 = 12; // log2(4096)

/// Thread-safe builder for huge long arrays.
///
/// # Thread Safety
///
/// - Growth and writes are synchronized via a lock on the page table.
/// - Multiple threads may call `write_range()` concurrently; correctness is preserved.
/// - Overlapping writes are allowed but yield last-writer-wins nondeterminism.
pub struct HugeLongArrayBuilder {
    pages: RwLock<Vec<Vec<i64>>>,
}

impl HugeLongArrayBuilder {
    /// Creates a new builder instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::HugeLongArrayBuilder;
    ///
    /// let builder = HugeLongArrayBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            pages: RwLock::new(Vec::new()),
        }
    }

    /// Builds the final HugeLongArray with the specified size.
    ///
    /// After this call, the builder should not be used for further writes.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::HugeLongArrayBuilder;
    ///
    /// let builder = HugeLongArrayBuilder::new();
    /// let mut allocator = gds::core::utils::paged::Allocator::new();
    ///
    /// // Allocate and fill
    /// builder.allocate(0, 1000, &mut allocator);
    /// let data: Vec<i64> = (0..1000).collect();
    /// allocator.insert(&data);
    ///
    /// // Build final array
    /// let array = builder.build(1000);
    /// assert_eq!(array.get(0), 0);
    /// assert_eq!(array.get(999), 999);
    /// ```
    pub fn build(&self, size: usize) -> HugeLongArray {
        let pages = self
            .pages
            .read()
            .expect("HugeLongArrayBuilder pages lock poisoned")
            .clone();

        // Convert Vec<Vec<i64>> to HugeLongArray
        HugeLongArray::of(pages, size)
    }

    /// Writes data to a specific range in the array with thread-safe growth.
    ///
    /// This is the primary method for filling the array. Multiple threads can safely
    /// write to different (non-overlapping) ranges concurrently. The method automatically
    /// grows the page array if needed.
    ///
    /// # Arguments
    ///
    /// * `start` - Starting index for writing
    /// * `data` - Data to write (length determines end position)
    ///
    /// # Panics
    ///
    /// Panics if `start + data.len()` would overflow usize.
    ///
    /// # Performance
    ///
    /// - O(1) amortized for page access (no growth)
    /// - O(n) for copying data where n = data.len()
    /// - O(log p) during growth where p = number of pages
    ///
    /// # Thread Safety
    ///
    /// Multiple threads can safely write to different ranges. Overlapping writes
    /// from different threads will result in undefined final values (last writer wins,
    /// but which thread finishes last is non-deterministic).
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::HugeLongArrayBuilder;
    /// use std::sync::Arc;
    /// use std::thread;
    ///
    /// let builder = Arc::new(HugeLongArrayBuilder::new());
    ///
    /// // Thread 1: write positions 0-99,999
    /// let builder1 = Arc::clone(&builder);
    /// let handle1 = thread::spawn(move || {
    ///     let data: Vec<i64> = (0..100_000).collect();
    ///     builder1.write_range(0, &data);
    /// });
    ///
    /// // Thread 2: write positions 100,000-199,999
    /// let builder2 = Arc::clone(&builder);
    /// let handle2 = thread::spawn(move || {
    ///     let data: Vec<i64> = (100_000..200_000).collect();
    ///     builder2.write_range(100_000, &data);
    /// });
    ///
    /// handle1.join().unwrap();
    /// handle2.join().unwrap();
    ///
    /// let array = builder.build(200_000);
    /// assert_eq!(array.get(0), 0);
    /// assert_eq!(array.get(199_999), 199_999);
    /// ```
    pub fn write_range(&self, start: usize, data: &[i64]) {
        if data.is_empty() {
            return;
        }

        let end = start
            .checked_add(data.len())
            .expect("HugeLongArrayBuilder::write_range overflow: start + data.len()");
        let end_page = (end - 1) >> LONG_PAGE_SHIFT;

        // Ensure we have enough pages (with lock for thread safety)
        self.ensure_pages(end_page);

        let mut pages = self
            .pages
            .write()
            .expect("HugeLongArrayBuilder pages lock poisoned");

        // Write data across pages
        let mut data_offset = 0;
        let mut current_pos = start;

        while data_offset < data.len() {
            let page_index = current_pos >> LONG_PAGE_SHIFT;
            let offset_in_page = current_pos & LONG_PAGE_MASK;
            let remaining_in_page = LONG_PAGE_SIZE - offset_in_page;
            let remaining_data = data.len() - data_offset;
            let to_copy = remaining_in_page.min(remaining_data);

            // SAFETY: We've ensured pages exist via ensure_pages
            // Each thread writes to non-overlapping ranges (enforced by caller)
            let page = &mut pages[page_index];
            page[offset_in_page..offset_in_page + to_copy]
                .copy_from_slice(&data[data_offset..data_offset + to_copy]);

            data_offset += to_copy;
            current_pos += to_copy;
        }
    }

    /// Ensures the page array has at least `required_page + 1` pages.
    fn ensure_pages(&self, required_page: usize) {
        let mut pages = self
            .pages
            .write()
            .expect("HugeLongArrayBuilder pages lock poisoned");

        if required_page < pages.len() {
            return;
        }

        for _ in pages.len()..=required_page {
            pages.push(vec![0i64; LONG_PAGE_SIZE]);
        }
    }
}

impl Default for HugeLongArrayBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let builder = HugeLongArrayBuilder::new();
        let array = builder.build(0);
        assert_eq!(array.size(), 0);
    }

    #[test]
    fn test_single_write() {
        let builder = HugeLongArrayBuilder::new();

        let data: Vec<i64> = (0..100).collect();
        builder.write_range(0, &data);

        let array = builder.build(100);
        assert_eq!(array.size(), 100);
        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(50), 50);
        assert_eq!(array.get(99), 99);
    }

    #[test]
    fn test_multiple_writes_sequential() {
        let builder = HugeLongArrayBuilder::new();

        // First write: 0-999
        let data1: Vec<i64> = (0..1000).collect();
        builder.write_range(0, &data1);

        // Second write: 1000-1999
        let data2: Vec<i64> = (1000..2000).collect();
        builder.write_range(1000, &data2);

        let array = builder.build(2000);
        assert_eq!(array.size(), 2000);
        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(999), 999);
        assert_eq!(array.get(1000), 1000);
        assert_eq!(array.get(1999), 1999);
    }

    #[test]
    fn test_write_across_page_boundary() {
        let builder = HugeLongArrayBuilder::new();

        // Write across page boundary
        let size = 10_000;
        let boundary = LONG_PAGE_SIZE;
        let data: Vec<i64> = (0..size as i64).collect();
        builder.write_range(0, &data);

        let array = builder.build(size);
        assert_eq!(array.size(), size);
        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(boundary - 1), (boundary - 1) as i64); // Last element of first page
        assert_eq!(array.get(boundary), boundary as i64); // First element of second page
        assert_eq!(array.get(size - 1), (size - 1) as i64);
    }

    #[test]
    fn test_concurrent_writes() {
        use std::sync::Arc;
        use std::thread;

        let builder = Arc::new(HugeLongArrayBuilder::new());
        let num_threads = 4;
        let elements_per_thread = 10_000;

        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let builder_clone = Arc::clone(&builder);
                thread::spawn(move || {
                    let start = thread_id * elements_per_thread;
                    let data: Vec<i64> = (start..start + elements_per_thread)
                        .map(|i| i as i64)
                        .collect();
                    builder_clone.write_range(start, &data);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let total_size = num_threads * elements_per_thread;
        let array = builder.build(total_size);

        assert_eq!(array.size(), total_size);

        // Verify all threads wrote correctly
        for thread_id in 0..num_threads {
            let start = thread_id * elements_per_thread;
            assert_eq!(array.get(start), start as i64);
            assert_eq!(
                array.get(start + elements_per_thread - 1),
                (start + elements_per_thread - 1) as i64
            );
        }
    }

    #[test]
    fn test_large_write() {
        let builder = HugeLongArrayBuilder::new();

        let size = 1_000_000;
        let data: Vec<i64> = (0..size as i64).collect();
        builder.write_range(0, &data);

        let array = builder.build(size);
        assert_eq!(array.size(), size);
        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(size / 2), (size / 2) as i64);
        assert_eq!(array.get(size - 1), (size - 1) as i64);
    }

    #[test]
    fn test_non_contiguous_writes() {
        let builder = HugeLongArrayBuilder::new();

        // Write range 0-999
        let data1: Vec<i64> = (0..1000).collect();
        builder.write_range(0, &data1);

        // Write range 5000-5999 (gap in between)
        let data2: Vec<i64> = (5000..6000).collect();
        builder.write_range(5000, &data2);

        let array = builder.build(6000);
        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(999), 999);
        assert_eq!(array.get(1000), 0); // Gap - should be zero
        assert_eq!(array.get(5000), 5000);
        assert_eq!(array.get(5999), 5999);
    }

    #[test]
    fn test_empty_write() {
        let builder = HugeLongArrayBuilder::new();

        // Empty write should be no-op
        builder.write_range(0, &[]);

        let array = builder.build(0);
        assert_eq!(array.size(), 0);
    }

    #[test]
    fn test_overwrite() {
        let builder = HugeLongArrayBuilder::new();

        // First write
        let data1 = vec![1i64, 2, 3, 4, 5];
        builder.write_range(0, &data1);

        // Overwrite same range
        let data2 = vec![10i64, 20, 30, 40, 50];
        builder.write_range(0, &data2);

        let array = builder.build(5);
        assert_eq!(array.get(0), 10);
        assert_eq!(array.get(4), 50);
    }
}
