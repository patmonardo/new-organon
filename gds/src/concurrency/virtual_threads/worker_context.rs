//! Per-thread worker context for graph algorithms.
//!
//! This provides thread-local state management for parallel algorithms,
//! useful for accumulating per-thread results before final aggregation.

use parking_lot::Mutex;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Per-thread worker context.
///
/// This allows each Rayon worker thread to maintain its own state,
/// which is useful for algorithms that need to accumulate results
/// locally before final aggregation (reducing contention).
///
/// # Examples
///
/// ```
/// use gds::concurrency::virtual_threads::{WorkerContext, Executor};
/// use gds::concurrency::Concurrency;
/// use gds::concurrency::TerminationFlag;
///
/// // Each worker accumulates a local sum
/// let context = WorkerContext::new(|| 0usize);
///
/// let executor = Executor::new(Concurrency::of(4));
/// let termination = TerminationFlag::running_true();
///
/// executor.parallel_for(0, 1000, &termination, |i| {
///     context.with(|local_sum| {
///         *local_sum += i;
///     });
/// });
///
/// // Collect results from all workers
/// let total: usize = context.collect().into_iter().sum();
/// ```
pub struct WorkerContext<T> {
    id: usize,
    init: Box<dyn Fn() -> T + Send + Sync>,
    values: Arc<Mutex<Vec<Arc<Mutex<T>>>>>,
    _phantom: PhantomData<T>,
}

impl<T: 'static> WorkerContext<T> {
    /// Create a new worker context with an initialization function.
    ///
    /// The init function is called once per worker thread to create
    /// thread-local state.
    pub fn new<F>(init: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

        Self {
            id,
            init: Box::new(init),
            values: Arc::new(Mutex::new(Vec::new())),
            _phantom: PhantomData,
        }
    }

    /// Access the thread-local state.
    ///
    /// This provides a mutable reference to the worker's local state.
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        thread_local! {
            static CONTEXTS: RefCell<HashMap<usize, Box<dyn Any>>> = RefCell::new(HashMap::new());
        }

        let handle: Arc<Mutex<T>> = CONTEXTS.with(|cell| {
            let mut contexts = cell.borrow_mut();
            let entry = contexts.entry(self.id).or_insert_with(|| {
                let handle = Arc::new(Mutex::new((self.init)()));
                self.values.lock().push(Arc::clone(&handle));
                Box::new(handle) as Box<dyn Any>
            });

            entry
                .downcast_ref::<Arc<Mutex<T>>>()
                .expect("WorkerContext internal type mismatch")
                .clone()
        });

        let mut guard = handle.lock();
        f(&mut guard)
    }

    /// Collect all worker-local values.
    ///
    /// This returns a snapshot of the values for every thread that has ever called [`WorkerContext::with`]
    /// for this context instance.
    pub fn collect(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.values
            .lock()
            .iter()
            .map(|v| v.lock().clone())
            .collect()
    }
}

/// Helper for creating worker-local aggregators.
///
/// This is a convenience wrapper around WorkerContext for common
/// aggregation patterns.
pub struct WorkerLocalAggregator<T> {
    context: WorkerContext<T>,
}

impl<T: Default + 'static> WorkerLocalAggregator<T> {
    /// Create a new worker-local aggregator with default initialization.
    pub fn new() -> Self {
        Self {
            context: WorkerContext::new(T::default),
        }
    }
}

impl<T: Default + 'static> Default for WorkerLocalAggregator<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: 'static> WorkerLocalAggregator<T> {
    /// Create a new worker-local aggregator with custom initialization.
    pub fn with_init<F>(init: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            context: WorkerContext::new(init),
        }
    }

    /// Update the worker-local value.
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        self.context.with(f);
    }

    /// Get a copy of the worker-local value.
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.context.with(|v| v.clone())
    }

    /// Collect all worker-local values.
    pub fn collect(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.context.collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concurrency::virtual_threads::Executor;
    use crate::concurrency::Concurrency;
    use crate::concurrency::TerminationFlag;

    #[test]
    fn test_worker_context_basic() {
        let context = WorkerContext::new(|| 0usize);

        context.with(|value| {
            *value = 42;
        });

        let result = context.with(|value| *value);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_worker_context_accumulation() {
        let context = WorkerContext::new(|| 0usize);
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();

        executor
            .parallel_for(0, 100, &termination, |i| {
                context.with(|local_sum| {
                    *local_sum += i;
                });
            })
            .unwrap();

        let total: usize = context.collect().into_iter().sum();
        assert_eq!(total, 4950);
    }

    #[test]
    fn test_worker_local_aggregator_default() {
        let aggregator = WorkerLocalAggregator::<usize>::new();

        aggregator.update(|v| *v += 10);
        assert_eq!(aggregator.get(), 10);

        aggregator.update(|v| *v += 5);
        assert_eq!(aggregator.get(), 15);
    }

    #[test]
    fn test_worker_local_aggregator_custom_init() {
        let aggregator = WorkerLocalAggregator::with_init(|| vec![1, 2, 3]);

        aggregator.update(|v| v.push(4));
        assert_eq!(aggregator.get(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_worker_local_aggregator_parallel() {
        let aggregator = WorkerLocalAggregator::<usize>::new();
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();

        executor
            .parallel_for(0, 100, &termination, |_| {
                aggregator.update(|count| *count += 1);
            })
            .unwrap();

        let total: usize = aggregator.collect().into_iter().sum();
        assert_eq!(total, 100);
    }
}
