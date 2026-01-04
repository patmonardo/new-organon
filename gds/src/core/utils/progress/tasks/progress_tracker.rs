//! ProgressTracker interface (Java GDS parity).
//!
//! This module intentionally lives under `tasks/`.
//!
//! We keep the API close to Java’s `ProgressTracker` interface, but expressed
//! as a Rust trait.

use crate::concurrency::Concurrency;
use crate::mem::MemoryRange;

use super::{LogLevel, UNKNOWN_VOLUME};

/// Java-parity progress tracker interface.
///
/// Implementations are expected to manage nested tasks (`begin_subtask` / `end_subtask`),
/// mutate the currently running task’s progress/volume, and emit log messages.
pub trait ProgressTracker {
    fn set_estimated_resource_footprint(&mut self, memory_estimation_in_bytes: MemoryRange);

    fn requested_concurrency(&mut self, concurrency: Concurrency);

    fn begin_subtask(&mut self);

    fn begin_subtask_with_volume(&mut self, task_volume: usize) {
        self.begin_subtask();
        self.set_volume(task_volume);
    }

    fn begin_subtask_unknown(&mut self) {
        self.begin_subtask_with_volume(UNKNOWN_VOLUME);
    }

    fn begin_subtask_with_description(&mut self, expected_task_description: &str) {
        self.begin_subtask();
        self.assert_subtask(expected_task_description);
    }

    fn begin_subtask_with_description_and_volume(
        &mut self,
        expected_task_description: &str,
        task_volume: usize,
    ) {
        self.begin_subtask_with_description(expected_task_description);
        self.set_volume(task_volume);
    }

    fn end_subtask(&mut self);

    fn end_subtask_with_description(&mut self, expected_task_description: &str) {
        self.assert_subtask(expected_task_description);
        self.end_subtask();
    }

    fn end_subtask_with_failure(&mut self);

    fn end_subtask_with_failure_and_description(&mut self, expected_task_description: &str) {
        self.assert_subtask(expected_task_description);
        self.end_subtask_with_failure();
    }

    fn log_progress(&mut self, value: usize);

    fn log_progress_one(&mut self) {
        self.log_progress(1);
    }

    fn log_progress_with_message(&mut self, value: usize, message_template: &str);

    /// Prefer setting volume via the leaf-task factory so root progress is known from the start.
    fn set_volume(&mut self, volume: usize);

    /// Returns the currently running task’s volume, or UNKNOWN_VOLUME if none.
    fn current_volume(&mut self) -> usize;

    fn log_debug(&mut self, message_supplier: &dyn Fn() -> String);

    fn log_message(&mut self, level: LogLevel, message: &str);

    fn log_debug_message(&mut self, message: &str) {
        self.log_message(LogLevel::Debug, message);
    }

    fn log_warning(&mut self, message: &str) {
        self.log_message(LogLevel::Warning, message);
    }

    fn log_info(&mut self, message: &str) {
        self.log_message(LogLevel::Info, message);
    }

    fn release(&mut self);

    fn set_steps(&mut self, steps: usize);

    fn log_steps(&mut self, steps: usize);

    fn assert_subtask(&mut self, _expected_task_description_substring: &str) {
        // default: do nothing
    }
}

/// A single no-op tracker value.
///
/// This avoids carrying a Java-style nested empty class, while still giving a
/// canonical “null tracker”.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopProgressTracker;

pub const NULL_TRACKER: NoopProgressTracker = NoopProgressTracker;

impl ProgressTracker for NoopProgressTracker {
    fn set_estimated_resource_footprint(&mut self, _memory_estimation_in_bytes: MemoryRange) {}

    fn requested_concurrency(&mut self, _concurrency: Concurrency) {}

    fn begin_subtask(&mut self) {}

    fn end_subtask(&mut self) {}

    fn end_subtask_with_failure(&mut self) {}

    fn log_progress(&mut self, _value: usize) {}

    fn log_progress_with_message(&mut self, _value: usize, _message_template: &str) {}

    fn set_volume(&mut self, _volume: usize) {}

    fn current_volume(&mut self) -> usize {
        UNKNOWN_VOLUME
    }

    fn log_debug(&mut self, _message_supplier: &dyn Fn() -> String) {}

    fn log_message(&mut self, _level: LogLevel, _message: &str) {}

    fn release(&mut self) {}

    fn set_steps(&mut self, _steps: usize) {}

    fn log_steps(&mut self, _steps: usize) {}
}
