//! Termination-aware ProgressTracker wrapper.
//!
//! This is a small utility for "machinery" consumers: it threads a `TerminationFlag`
//! into progress tracking without changing the `ProgressTracker` trait.
//!
//! The wrapper calls `termination_flag.assert_running()` before delegating to the
//! underlying tracker. `assert_running()` panics with `TerminatedException` when
//! the computation has been cancelled; callers should convert that panic into an
//! error using `catch_unwind` if they need a `Result` surface.

use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::tasks::LogLevel;
use crate::core::utils::progress::ProgressTracker;
use crate::mem::MemoryRange;

pub struct TerminationAwareProgressTracker<'a> {
    inner: &'a mut dyn ProgressTracker,
    termination_flag: &'a TerminationFlag,
}

impl<'a> TerminationAwareProgressTracker<'a> {
    pub fn new(inner: &'a mut dyn ProgressTracker, termination_flag: &'a TerminationFlag) -> Self {
        Self {
            inner,
            termination_flag,
        }
    }

    #[inline]
    fn check(&self) {
        self.termination_flag.assert_running();
    }
}

impl<'a> ProgressTracker for TerminationAwareProgressTracker<'a> {
    fn set_estimated_resource_footprint(&mut self, memory_estimation_in_bytes: MemoryRange) {
        self.check();
        self.inner
            .set_estimated_resource_footprint(memory_estimation_in_bytes);
    }

    fn requested_concurrency(&mut self, concurrency: Concurrency) {
        self.check();
        self.inner.requested_concurrency(concurrency);
    }

    fn begin_subtask(&mut self) {
        self.check();
        self.inner.begin_subtask();
    }

    fn end_subtask(&mut self) {
        self.check();
        self.inner.end_subtask();
    }

    fn end_subtask_with_failure(&mut self) {
        self.check();
        self.inner.end_subtask_with_failure();
    }

    fn log_progress(&mut self, value: usize) {
        self.check();
        self.inner.log_progress(value);
    }

    fn log_progress_with_message(&mut self, value: usize, message_template: &str) {
        self.check();
        self.inner
            .log_progress_with_message(value, message_template);
    }

    fn set_volume(&mut self, volume: usize) {
        self.check();
        self.inner.set_volume(volume);
    }

    fn current_volume(&mut self) -> usize {
        self.check();
        self.inner.current_volume()
    }

    fn log_debug(&mut self, message_supplier: &dyn Fn() -> String) {
        self.check();
        self.inner.log_debug(message_supplier);
    }

    fn log_message(&mut self, level: LogLevel, message: &str) {
        self.check();
        self.inner.log_message(level, message);
    }

    fn release(&mut self) {
        self.check();
        self.inner.release();
    }

    fn set_steps(&mut self, steps: usize) {
        self.check();
        self.inner.set_steps(steps);
    }

    fn log_steps(&mut self, steps: usize) {
        self.check();
        self.inner.log_steps(steps);
    }

    fn assert_subtask(&mut self, expected_task_description_substring: &str) {
        self.check();
        self.inner
            .assert_subtask(expected_task_description_substring);
    }
}
