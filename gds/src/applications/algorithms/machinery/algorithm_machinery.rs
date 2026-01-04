//! AlgorithmMachinery (Java parity).
//!
//! Encapsulates running an algorithm and managing ProgressTracker lifecycle.
//!
//! Java reference: `AlgorithmMachinery.runAlgorithmsAndManageProgressTracker(...)`.

use crate::concurrency::Concurrency;
use crate::core::utils::progress::ProgressTracker;

pub struct AlgorithmMachinery;

impl AlgorithmMachinery {
    /// Java-parity runner.
    ///
    /// Responsibilities:
    /// - propagate `requested_concurrency` into the tracker
    /// - manage begin/end (and failure) lifecycle
    /// - optionally `release()` the tracker (mirrors Java's resource handling)
    pub fn run_algorithms_and_manage_progress_tracker<R, E>(
        progress_tracker: &mut dyn ProgressTracker,
        release_progress_tracker: bool,
        requested_concurrency: Concurrency,
        algorithm: impl FnOnce(&mut dyn ProgressTracker) -> Result<R, E>,
    ) -> Result<R, E> {
        progress_tracker.requested_concurrency(requested_concurrency);
        progress_tracker.begin_subtask();

        let result = algorithm(progress_tracker);

        match result {
            Ok(value) => {
                progress_tracker.end_subtask();
                if release_progress_tracker {
                    progress_tracker.release();
                }
                Ok(value)
            }
            Err(err) => {
                progress_tracker.end_subtask_with_failure();
                if release_progress_tracker {
                    progress_tracker.release();
                }
                Err(err)
            }
        }
    }

    /// Back-compat convenience wrapper.
    pub fn run_and_manage_progress_tracker<R, E>(
        algorithm: impl FnOnce(&mut dyn ProgressTracker) -> Result<R, E>,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<R, E> {
        Self::run_algorithms_and_manage_progress_tracker(
            progress_tracker,
            false,
            Concurrency::of(1),
            algorithm,
        )
    }
}
