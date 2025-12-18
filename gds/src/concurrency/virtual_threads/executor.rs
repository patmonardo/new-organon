//! High-level parallel executor for graph algorithms.
//!
//! The Executor provides a clean API for parallel execution with termination support,
//! built on top of Rayon's work-stealing scheduler.

use crate::concurrency::install_with_concurrency;
use crate::concurrency::pool::PoolSizes as _;
use crate::concurrency::pool::PoolSizesService;
use crate::concurrency::{BatchSize, Concurrency};
use crate::concurrency::{TerminatedException, TerminationFlag};
use rayon::prelude::*;

const TERMINATION_POLL_EVERY: usize = 256;

/// Parallel executor for graph algorithms.
///
/// This is the main entry point for parallel execution. It wraps Rayon's thread pool
/// with termination checking and provides Pregel-friendly APIs.
///
/// # Examples
///
/// ```
/// use gds::concurrency::virtual_threads::Executor;
/// use gds::concurrency::Concurrency;
/// use gds::concurrency::TerminationFlag;
///
/// let executor = Executor::new(Concurrency::of(4));
/// let termination = TerminationFlag::running_true();
///
/// // Execute parallel work with automatic synchronization
/// executor.scope(&termination, |scope| {
///     scope.spawn_many(1000, |node_id| {
///         // Process node in parallel
///         println!("Processing node {}", node_id);
///     });
/// });
/// ```
#[derive(Clone)]
pub struct Executor {
    concurrency: Concurrency,
}

impl Executor {
    /// Create a new executor with the specified concurrency level.
    ///
    /// The concurrency level determines how many threads Rayon will use for parallel work.
    pub fn new(concurrency: Concurrency) -> Self {
        Self { concurrency }
    }

    /// Get the concurrency level for this executor.
    pub fn concurrency(&self) -> Concurrency {
        self.concurrency
    }

    /// Execute work within a synchronization scope.
    ///
    /// This is the core method for Pregel-style algorithms. All work spawned within
    /// the scope will complete before the scope ends, providing a synchronization boundary.
    ///
    /// # Arguments
    ///
    /// * `termination` - Flag to check for early termination
    /// * `work` - Function that receives a Scope and spawns parallel work
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all work completed, or `Err(())` if terminated early.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::virtual_threads::Executor;
    /// use gds::concurrency::Concurrency;
    /// use gds::concurrency::TerminationFlag;
    ///
    /// let executor = Executor::new(Concurrency::of(4));
    /// let termination = TerminationFlag::running_true();
    ///
    /// executor.scope(&termination, |scope| {
    ///     // Spawn parallel work
    ///     scope.spawn_many(100, |i| {
    ///         println!("Work {}", i);
    ///     });
    /// });
    /// ```
    pub fn scope<F, R>(
        &self,
        termination: &TerminationFlag,
        work: F,
    ) -> Result<R, TerminatedException>
    where
        F: FnOnce(&super::Scope) -> R + Send,
        R: Send,
    {
        install_with_concurrency(self.concurrency, || {
            if !termination.running() {
                return Err(TerminatedException);
            }

            let scope = super::Scope::new(self.concurrency, termination);
            let result = work(&scope);

            if termination.running() {
                Ok(result)
            } else {
                Err(TerminatedException)
            }
        })
    }

    /// Execute a simple parallel loop over a range.
    ///
    /// This is a convenience method for the common case of iterating over nodes.
    /// It's equivalent to using `scope` with `spawn_many`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::virtual_threads::Executor;
    /// use gds::concurrency::Concurrency;
    /// use gds::concurrency::TerminationFlag;
    ///
    /// let executor = Executor::new(Concurrency::of(4));
    /// let termination = TerminationFlag::running_true();
    ///
    /// executor.parallel_for(0, 1000, &termination, |node_id| {
    ///     println!("Processing node {}", node_id);
    /// });
    /// ```
    pub fn parallel_for<F>(
        &self,
        start: usize,
        end: usize,
        termination: &TerminationFlag,
        task: F,
    ) -> Result<(), TerminatedException>
    where
        F: Fn(usize) + Send + Sync,
    {
        install_with_concurrency(self.concurrency, || {
            if start >= end || !termination.running() {
                return if termination.running() {
                    Ok(())
                } else {
                    Err(TerminatedException)
                };
            }

            let total = end - start;
            let batch_size = BatchSize::for_parallel_work(total, self.concurrency).value();
            let num_batches = total.div_ceil(batch_size);

            (0..num_batches).into_par_iter().try_for_each(|batch_idx| {
                if !termination.running() {
                    return Err(TerminatedException);
                }

                let batch_start = start + batch_idx * batch_size;
                let batch_end = (batch_start + batch_size).min(end);

                for (local_idx, i) in (batch_start..batch_end).enumerate() {
                    if (local_idx & (TERMINATION_POLL_EVERY - 1)) == 0 && !termination.running() {
                        return Err(TerminatedException);
                    }
                    task(i);
                }

                Ok(())
            })
        })
    }

    /// Execute a parallel map operation over a range.
    ///
    /// This collects results from all parallel tasks into a Vec.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::virtual_threads::Executor;
    /// use gds::concurrency::Concurrency;
    /// use gds::concurrency::TerminationFlag;
    ///
    /// let executor = Executor::new(Concurrency::of(4));
    /// let termination = TerminationFlag::running_true();
    ///
    /// let results = executor.parallel_map(0, 10, &termination, |i| i * 2).unwrap();
    /// assert_eq!(results, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    /// ```
    pub fn parallel_map<F, T>(
        &self,
        start: usize,
        end: usize,
        termination: &TerminationFlag,
        mapper: F,
    ) -> Result<Vec<T>, TerminatedException>
    where
        F: Fn(usize) -> T + Send + Sync,
        T: Send,
    {
        install_with_concurrency(self.concurrency, || {
            if start >= end || !termination.running() {
                return if termination.running() {
                    Ok(Vec::new())
                } else {
                    Err(TerminatedException)
                };
            }

            (start..end)
                .into_par_iter()
                .with_max_len(BatchSize::for_parallel_work(end - start, self.concurrency).value())
                .enumerate()
                .map(|(idx, i)| {
                    if (idx & (TERMINATION_POLL_EVERY - 1)) == 0 && !termination.running() {
                        return Err(TerminatedException);
                    }
                    Ok(mapper(i))
                })
                .collect()
        })
    }

    /// Execute a parallel reduction operation over a range.
    ///
    /// This is perfect for aggregations like sum, max, count, etc.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::virtual_threads::Executor;
    /// use gds::concurrency::Concurrency;
    /// use gds::concurrency::TerminationFlag;
    ///
    /// let executor = Executor::new(Concurrency::of(4));
    /// let termination = TerminationFlag::running_true();
    ///
    /// // Sum all values 0..100
    /// let sum = executor.parallel_reduce(
    ///     0, 100, &termination,
    ///     0usize,
    ///     |i| i,
    ///     |a, b| a + b
    /// ).unwrap();
    /// assert_eq!(sum, 4950);
    /// ```
    pub fn parallel_reduce<T, M, R>(
        &self,
        start: usize,
        end: usize,
        termination: &TerminationFlag,
        identity: T,
        mapper: M,
        reducer: R,
    ) -> Result<T, TerminatedException>
    where
        T: Send + Sync + Clone,
        M: Fn(usize) -> T + Send + Sync,
        R: Fn(T, T) -> T + Send + Sync,
    {
        install_with_concurrency(self.concurrency, || {
            if start >= end || !termination.running() {
                return if termination.running() {
                    Ok(identity)
                } else {
                    Err(TerminatedException)
                };
            }

            (start..end)
                .into_par_iter()
                .with_max_len(BatchSize::for_parallel_work(end - start, self.concurrency).value())
                .try_fold(
                    || (identity.clone(), 0usize),
                    |(acc, idx), i| {
                        if (idx & (TERMINATION_POLL_EVERY - 1)) == 0 && !termination.running() {
                            return Err(TerminatedException);
                        }
                        Ok((reducer(acc, mapper(i)), idx + 1))
                    },
                )
                .try_reduce(
                    || (identity.clone(), 0usize),
                    |(a, _), (b, _)| Ok((reducer(a, b), 0usize)),
                )
                .map(|(result, _)| result)
        })
    }
}

impl Default for Executor {
    fn default() -> Self {
        // Default to the platform's configured pool sizes (Open GDS: 4).
        // This avoids unbounded thread creation and keeps defaults consistent
        // with the licensing-oriented concurrency limits.
        let pool_sizes = PoolSizesService::pool_sizes();
        Self::new(Concurrency::from_usize(pool_sizes.core_pool_size()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::current_num_threads;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    #[test]
    fn test_executor_new() {
        let executor = Executor::new(Concurrency::of(4));
        assert_eq!(executor.concurrency().value(), 4);
    }

    #[test]
    fn test_executor_default() {
        let executor = Executor::default();
        assert!(executor.concurrency().value() > 0);
    }

    #[test]
    fn test_parallel_for_executes_all() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        executor
            .parallel_for(0, 100, &termination, move |_| {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            })
            .unwrap();

        assert_eq!(counter.load(Ordering::Relaxed), 100);
    }

    #[test]
    fn test_parallel_for_respects_termination() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::stop_running();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        let result = executor.parallel_for(0, 1000, &termination, move |_| {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });

        assert!(result.is_err());
        assert!(counter.load(Ordering::Relaxed) < 1000);
    }

    #[test]
    fn test_parallel_for_empty_range() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        executor
            .parallel_for(10, 10, &termination, move |_| {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            })
            .unwrap();

        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_parallel_map() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();

        let results = executor
            .parallel_map(0, 10, &termination, |i| i * 2)
            .unwrap();

        assert_eq!(results, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    }

    #[test]
    fn test_parallel_map_respects_termination() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::stop_running();

        let result = executor.parallel_map(0, 1000, &termination, |i| i * 2);

        assert!(result.is_err());
    }

    #[test]
    fn test_parallel_reduce_sum() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();

        let sum = executor
            .parallel_reduce(0, 100, &termination, 0usize, |i| i, |a, b| a + b)
            .unwrap();

        assert_eq!(sum, 4950);
    }

    #[test]
    fn test_parallel_reduce_max() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();

        let max = executor
            .parallel_reduce(0, 100, &termination, 0usize, |i| i, |a, b| a.max(b))
            .unwrap();

        assert_eq!(max, 99);
    }

    #[test]
    fn test_parallel_reduce_respects_termination() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::stop_running();

        let result = executor.parallel_reduce(0, 1000, &termination, 0usize, |i| i, |a, b| a + b);

        assert!(result.is_err());
    }

    #[test]
    fn test_scope_basic() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        executor
            .scope(&termination, |scope| {
                scope.spawn_many(100, |_| {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                });
            })
            .unwrap();

        assert_eq!(counter.load(Ordering::Relaxed), 100);
    }

    #[test]
    fn test_scope_installs_configured_thread_pool() {
        let executor = Executor::new(Concurrency::of(3));
        let termination = TerminationFlag::running_true();

        let threads = executor
            .scope(&termination, |_scope| current_num_threads())
            .unwrap();

        assert_eq!(threads, 3);
    }

    #[test]
    fn test_scope_respects_termination() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::stop_running();

        let result = executor.scope(&termination, |_scope| {
            // Should not execute
            42
        });

        assert!(result.is_err());
    }
}
