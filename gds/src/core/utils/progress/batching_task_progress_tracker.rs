//! Java-parity `BatchingTaskProgressTracker`.
//!
//! This is a thin wrapper around another `ProgressTracker` that batches
//! `log_progress(..)` calls to reduce overhead in tight loops.

use crate::concurrency::Concurrency;
use crate::mem::MemoryRange;

use super::tasks::{LogLevel, ProgressTracker};
use super::BatchingProgressLogger;

pub struct BatchingTaskProgressTracker<P: ProgressTracker> {
    delegate: P,
    batch_size: usize,
    pending_progress: usize,
}

impl<P: ProgressTracker> BatchingTaskProgressTracker<P> {
    pub fn new(delegate: P, task_volume: usize, concurrency: Concurrency) -> Self {
        let batch_size = BatchingProgressLogger::calculate_batch_size_for_volume(
            task_volume as u64,
            concurrency.value(),
        ) as usize;

        Self {
            delegate,
            batch_size: batch_size.max(1),
            pending_progress: 0,
        }
    }

    pub fn into_inner(self) -> P {
        self.delegate
    }

    fn flush(&mut self) {
        if self.pending_progress == 0 {
            return;
        }

        self.delegate.log_progress(self.pending_progress);
        self.pending_progress = 0;
    }

    fn flush_full_batches(&mut self) {
        if self.pending_progress < self.batch_size {
            return;
        }

        let flush_amount = self.pending_progress - (self.pending_progress % self.batch_size);
        if flush_amount == 0 {
            return;
        }

        self.delegate.log_progress(flush_amount);
        self.pending_progress -= flush_amount;
    }
}

impl<P: ProgressTracker> ProgressTracker for BatchingTaskProgressTracker<P> {
    fn set_estimated_resource_footprint(&mut self, memory_estimation_in_bytes: MemoryRange) {
        self.delegate
            .set_estimated_resource_footprint(memory_estimation_in_bytes);
    }

    fn requested_concurrency(&mut self, concurrency: Concurrency) {
        self.delegate.requested_concurrency(concurrency);
    }

    fn begin_subtask(&mut self) {
        self.flush();
        self.delegate.begin_subtask();
    }

    fn end_subtask(&mut self) {
        self.flush();
        self.delegate.end_subtask();
    }

    fn end_subtask_with_failure(&mut self) {
        self.flush();
        self.delegate.end_subtask_with_failure();
    }

    fn log_progress(&mut self, value: usize) {
        self.pending_progress = self.pending_progress.saturating_add(value);
        self.flush_full_batches();
    }

    fn log_progress_with_message(&mut self, value: usize, message_template: &str) {
        self.flush();
        self.delegate
            .log_progress_with_message(value, message_template);
    }

    fn set_volume(&mut self, volume: usize) {
        self.flush();
        self.delegate.set_volume(volume);
    }

    fn current_volume(&mut self) -> usize {
        self.delegate.current_volume()
    }

    fn log_debug(&mut self, message_supplier: &dyn Fn() -> String) {
        self.delegate.log_debug(message_supplier);
    }

    fn log_message(&mut self, level: LogLevel, message: &str) {
        self.delegate.log_message(level, message);
    }

    fn release(&mut self) {
        self.flush();
        self.delegate.release();
    }

    fn set_steps(&mut self, steps: usize) {
        self.flush();
        self.delegate.set_steps(steps);
    }

    fn log_steps(&mut self, steps: usize) {
        self.flush();
        self.delegate.log_steps(steps);
    }

    fn assert_subtask(&mut self, expected_task_description_substring: &str) {
        self.delegate
            .assert_subtask(expected_task_description_substring);
    }
}
