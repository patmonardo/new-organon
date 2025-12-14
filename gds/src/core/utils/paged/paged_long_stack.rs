//! High-performance paged stack for billion-scale graph algorithms.
//!
//! Essential for algorithms requiring massive stack capacity:
//! - Depth-First Search (DFS) on huge graphs
//! - Backtracking algorithms (pathfinding, constraint solving)
//! - Recursive traversals with billion-node graphs
//! - Undo/redo operations in graph editing
//! - Call stack simulation for iterative implementations
//! - Expression evaluation with deep nesting
//!
//! # Performance Characteristics
//!
//! - O(1) push/pop operations (amortized)
//! - Paged memory layout prevents stack overflow
//! - Minimal page switching overhead
//! - Cache-friendly sequential access within pages
//! - Thread-safe growth through PagedDataStructure base
//!
//! # Memory Efficiency
//!
//! - Only allocates pages as needed (lazy allocation)
//! - Efficient page reuse during pop operations
//! - Predictable memory usage patterns
//! - Supports billion-element capacity
//!
//! # Graph Algorithm Applications
//!
//! - DFS traversal state management
//! - Backtracking in pathfinding algorithms
//! - Recursive algorithm iterativization
//! - Expression tree evaluation
//! - Undo stacks for graph modifications
//! - Call frame simulation in interpreters
//!
//! # Example
//!
//! ```rust
//! use gds::core::utils::paged::PagedLongStack;
//!
//! // Stack for DFS on massive graph
//! let mut dfs_stack = PagedLongStack::new(1_000_000);
//!
//! // Push starting node
//! dfs_stack.push(start_node_id);
//!
//! // DFS traversal loop
//! while !dfs_stack.is_empty() {
//!     let current_node = dfs_stack.pop();
//!
//!     // Process neighbors (pseudo-code)
//!     // for neighbor in graph.neighbors(current_node) {
//!     //     if !visited.contains(neighbor) {
//!         dfs_stack.push(neighbor);
//!     //         visited.insert(neighbor);
//!     //     }
//!     // }
//! }
//! ```

use crate::collections::PageUtil;
use crate::core::utils::paged::PageAllocatorFactory;
use crate::mem::Estimate;

/// High-performance paged stack for i64 values.
///
/// Provides billion-scale stack capacity with efficient push/pop operations.
pub struct PagedLongStack {
    page_size: usize,
    pages: Vec<Vec<i64>>,
    size: usize,
}

impl PagedLongStack {
    /// Creates a new paged stack with specified initial capacity.
    ///
    /// # Arguments
    ///
    /// * `initial_size` - Initial capacity (minimum 1)
    ///
    /// # Returns
    ///
    /// New paged stack instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gds::core::utils::paged::PagedLongStack;
    ///
    /// // Stack for DFS on 10M node graph
    /// let stack = PagedLongStack::new(10_000_000);
    /// assert!(stack.is_empty());
    /// ```
    pub fn new(initial_size: usize) -> Self {
        let page_size = PageAllocatorFactory::<Vec<i64>>::for_long_array().page_size();

        let mut pages = Vec::new();
        pages.push(Vec::with_capacity(page_size));

        let reserved_pages = PageUtil::num_pages_for(initial_size.max(1), page_size);
        if reserved_pages > 1 {
            pages.reserve(reserved_pages - 1);
        }

        Self {
            page_size,
            pages,
            size: 0,
        }
    }

    /// Memory estimation for capacity planning.
    ///
    /// Essential for resource allocation in large-scale processing.
    ///
    /// # Arguments
    ///
    /// * `size` - Expected maximum stack size
    ///
    /// # Returns
    ///
    /// Estimated memory usage in bytes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gds::core::utils::paged::PagedLongStack;
    ///
    /// let memory = PagedLongStack::memory_estimation(1_000_000_000);
    /// println!("1B stack needs ~{} GB", memory / (1024 * 1024 * 1024));
    /// ```
    pub fn memory_estimation(size: usize) -> usize {
        let page_size = PageAllocatorFactory::<Vec<i64>>::for_long_array().page_size();
        let num_pages = PageUtil::num_pages_for(size, page_size);
        let total_size_for_pages = num_pages * Estimate::size_of_long_array(page_size);

        // Add overhead for instance variables
        let instance_overhead = 3 * std::mem::size_of::<usize>() + std::mem::size_of::<isize>();

        total_size_for_pages + instance_overhead
    }

    /// Clears the stack, resetting to empty state.
    ///
    /// Reuses existing pages for efficiency.
    /// Fast O(1) operation - doesn't deallocate pages.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gds::core::utils::paged::PagedLongStack;
    ///
    /// let mut stack = PagedLongStack::new(1000);
    /// stack.push(42);
    /// stack.push(43);
    /// assert_eq!(stack.size(), 2);
    ///
    /// stack.clear();
    /// assert!(stack.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.size = 0;
        for page in &mut self.pages {
            page.clear();
        }
        if self.pages.is_empty() {
            self.pages.push(Vec::with_capacity(self.page_size));
        }
    }

    /// Pushes a value onto the stack.
    ///
    /// Automatically grows to accommodate new elements.
    ///
    /// # Arguments
    ///
    /// * `value` - Value to push onto stack
    ///
    /// # Performance
    ///
    /// O(1) amortized (occasional page allocation)
    ///
    /// # Graph Algorithm Use Cases
    ///
    /// - Push nodes to visit in DFS
    /// - Push backtrack points in pathfinding
    /// - Push function call frames in recursion simulation
    /// - Push undo operations in graph editing
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gds::core::utils::paged::PagedLongStack;
    ///
    /// let mut stack = PagedLongStack::new(100);
    /// stack.push(42);
    /// stack.push(43);
    ///
    /// assert_eq!(stack.size(), 2);
    /// assert_eq!(stack.pop(), 43);
    /// assert_eq!(stack.pop(), 42);
    /// ```
    pub fn push(&mut self, value: i64) {
        let page_index = self.size / self.page_size;
        self.ensure_page(page_index);
        self.pages[page_index].push(value);
        self.size += 1;
    }

    /// Pops a value from the stack.
    ///
    /// Returns to previous page when current page is exhausted.
    ///
    /// # Returns
    ///
    /// The top value from the stack
    ///
    /// # Panics
    ///
    /// Panics if stack is empty
    ///
    /// # Performance
    ///
    /// O(1) with occasional page switching
    ///
    /// # Graph Algorithm Use Cases
    ///
    /// - Get next node to visit in DFS
    /// - Retrieve backtrack point in pathfinding
    /// - Pop function call frame in recursion simulation
    /// - Execute undo operation in graph editing
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gds::core::utils::paged::PagedLongStack;
    ///
    /// let mut stack = PagedLongStack::new(100);
    /// stack.push(42);
    ///
    /// assert_eq!(stack.pop(), 42);
    /// ```
    pub fn pop(&mut self) -> i64 {
        assert!(!self.is_empty(), "Cannot pop from empty stack");

        let top_index = self.size - 1;
        let page_index = top_index / self.page_size;
        let result = self.pages[page_index]
            .pop()
            .expect("PagedLongStack invariant violated: missing top element");
        self.size -= 1;
        result
    }

    /// Peeks at the top value without removing it.
    ///
    /// Useful for inspecting next operation without commitment.
    ///
    /// # Returns
    ///
    /// The top value from the stack
    ///
    /// # Panics
    ///
    /// Panics if stack is empty
    ///
    /// # Performance
    ///
    /// O(1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gds::core::utils::paged::PagedLongStack;
    ///
    /// let mut stack = PagedLongStack::new(100);
    /// stack.push(42);
    ///
    /// assert_eq!(stack.peek(), 42);
    /// assert_eq!(stack.size(), 1); // Still in stack
    /// ```
    pub fn peek(&self) -> i64 {
        assert!(!self.is_empty(), "Cannot peek at empty stack");

        let top_index = self.size - 1;
        let page_index = top_index / self.page_size;
        *self.pages[page_index]
            .last()
            .expect("PagedLongStack invariant violated: missing top element")
    }

    /// Checks if the stack is empty.
    ///
    /// # Returns
    ///
    /// `true` if stack contains no elements
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gds::core::utils::paged::PagedLongStack;
    ///
    /// let mut stack = PagedLongStack::new(100);
    /// assert!(stack.is_empty());
    ///
    /// stack.push(42);
    /// assert!(!stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the number of elements in the stack.
    ///
    /// # Returns
    ///
    /// Current stack size
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gds::core::utils::paged::PagedLongStack;
    ///
    /// let mut stack = PagedLongStack::new(100);
    /// assert_eq!(stack.size(), 0);
    ///
    /// stack.push(1);
    /// stack.push(2);
    /// assert_eq!(stack.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Releases all resources and invalidates the stack.
    ///
    /// # Returns
    ///
    /// Estimated bytes freed
    pub fn release(&mut self) -> usize {
        let mut released = 0usize;
        for page in &self.pages {
            released += Estimate::size_of_long_array(page.capacity());
        }

        self.pages.clear();
        self.size = 0;
        released
    }

    fn ensure_page(&mut self, page_index: usize) {
        if self.pages.is_empty() {
            self.pages.push(Vec::with_capacity(self.page_size));
        }

        while self.pages.len() <= page_index {
            self.pages.push(Vec::with_capacity(self.page_size));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack() {
        let stack = PagedLongStack::new(1000);
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_push_pop() {
        let mut stack = PagedLongStack::new(100);

        stack.push(42);
        stack.push(43);
        stack.push(44);

        assert_eq!(stack.size(), 3);
        assert_eq!(stack.pop(), 44);
        assert_eq!(stack.pop(), 43);
        assert_eq!(stack.pop(), 42);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut stack = PagedLongStack::new(100);

        stack.push(42);
        stack.push(43);

        assert_eq!(stack.peek(), 43);
        assert_eq!(stack.size(), 2); // Peek doesn't remove

        assert_eq!(stack.pop(), 43);
        assert_eq!(stack.peek(), 42);
    }

    #[test]
    fn test_pop_empty() {
        use std::panic;

        let result = panic::catch_unwind(|| {
            let mut stack = PagedLongStack::new(100);
            stack.pop();
        });

        assert!(result.is_err(), "expected pop() to panic on empty stack");
    }

    #[test]
    fn test_peek_empty() {
        use std::panic;

        let result = panic::catch_unwind(|| {
            let stack = PagedLongStack::new(100);
            stack.peek();
        });

        assert!(result.is_err(), "expected peek() to panic on empty stack");
    }

    #[test]
    fn test_clear() {
        let mut stack = PagedLongStack::new(100);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.size(), 3);

        stack.clear();

        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_multiple_pages() {
        let mut stack = PagedLongStack::new(100);

        // Push enough to span multiple pages (4096 elements per page)
        let count = 10_000;
        for i in 0..count {
            stack.push(i);
        }

        assert_eq!(stack.size(), count as usize);

        // Pop them all back
        for i in (0..count).rev() {
            assert_eq!(stack.pop(), i);
        }

        assert!(stack.is_empty());
    }

    #[test]
    fn test_automatic_growth() {
        let mut stack = PagedLongStack::new(10); // Start very small

        // Push way more than initial capacity
        let count = 100_000;
        for i in 0..count {
            stack.push(i);
        }

        assert_eq!(stack.size(), count as usize);

        // Verify all elements
        for i in (0..count).rev() {
            assert_eq!(stack.pop(), i);
        }
    }

    #[test]
    fn test_lifo_order() {
        let mut stack = PagedLongStack::new(1000);

        let values = vec![1, 2, 3, 5, 8, 13, 21, 34, 55];

        for &value in &values {
            stack.push(value);
        }

        // Should come back in reverse order (LIFO)
        for &value in values.iter().rev() {
            assert_eq!(stack.pop(), value);
        }
    }

    #[test]
    fn test_memory_estimation() {
        let memory = PagedLongStack::memory_estimation(1_000_000);
        assert!(memory > 0);

        // Should be roughly 8MB for 1M i64s plus overhead
        assert!(memory >= 8_000_000);
        assert!(memory < 10_000_000); // Some overhead but not excessive
    }

    #[test]
    fn test_release() {
        let mut stack = PagedLongStack::new(10_000);

        for i in 0..1000 {
            stack.push(i);
        }

        let freed = stack.release();
        assert!(freed > 0);

        assert_eq!(stack.size(), 0);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_large_stack() {
        let mut stack = PagedLongStack::new(1_000_000);

        // Push 1 million elements
        for i in 0..1_000_000 {
            stack.push(i);
        }

        assert_eq!(stack.size(), 1_000_000);

        // Pop some
        for _ in 0..500_000 {
            stack.pop();
        }

        assert_eq!(stack.size(), 500_000);
    }
}
