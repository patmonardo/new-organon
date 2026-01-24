//! TaskProgressTracker (Java GDS parity direction).
//!
//! This is the “real tracker”: it registers the base task via TaskRegistry,
//! manages nested tasks, mutates task progress/volume, and emits logs.

use crate::concurrency::Concurrency;
use crate::core::utils::progress::{JobId, TaskRegistry, TaskRegistryFactory};
use crate::mem::MemoryRange;
use std::sync::{Arc, Mutex};

use super::{LogLevel, ProgressTracker, Status, Task, UNKNOWN_VOLUME};

use super::task_progress_logger::TaskProgressLogger;

const UNKNOWN_STEPS: isize = -1;

struct Inner {
    base_task: Task,
    task_registry: TaskRegistry,
    task_progress_logger: TaskProgressLogger,
    nested_tasks: Vec<Task>,
    current_task: Option<Task>,
    current_total_steps: isize,
    progress_leftovers: f64,
    did_log_error: bool,
}

#[derive(Clone)]
pub struct TaskProgressTracker {
    inner: Arc<Mutex<Inner>>,
}

impl TaskProgressTracker {
    /// Java parity entrypoint: fully specified construction.
    pub fn with_registry(
        base_task: Task,
        concurrency: Concurrency,
        job_id: JobId,
        task_registry_factory: &dyn TaskRegistryFactory,
    ) -> Self {
        let task_registry = task_registry_factory.new_instance(job_id);
        let task_progress_logger = TaskProgressLogger::new(base_task.clone(), concurrency.value());

        Self {
            inner: Arc::new(Mutex::new(Inner {
                base_task,
                task_registry,
                task_progress_logger,
                nested_tasks: Vec::new(),
                current_task: None,
                current_total_steps: UNKNOWN_STEPS,
                progress_leftovers: 0.0,
                did_log_error: false,
            })),
        }
    }

    /// Convenience constructor matching historical Rust call sites.
    /// Uses a fresh JobId and an empty TaskRegistry.
    pub fn new(task: super::LeafTask) -> Self {
        Self::with_concurrency(task, 1)
    }

    /// Convenience constructor matching historical Rust call sites.
    /// Uses a fresh JobId and an empty TaskRegistry.
    pub fn with_concurrency(task: super::LeafTask, concurrency: usize) -> Self {
        let registry_factory = crate::core::utils::progress::EmptyTaskRegistryFactory;
        Self::with_registry(
            task.base().clone(),
            Concurrency::of(concurrency.max(1)),
            JobId::new(),
            &registry_factory,
        )
    }

    fn format_progress_message(template: &str, value: usize) -> String {
        // Minimal formatting support for Java-style templates (Java parity).
        // Common cases in the repo are "%d" or "{}".
        if template.contains("%d") {
            template.replace("%d", &value.to_string())
        } else if template.contains("{}") {
            template.replace("{}", &value.to_string())
        } else {
            format!("{} {}", template, value)
        }
    }
}

impl ProgressTracker for TaskProgressTracker {
    fn set_estimated_resource_footprint(&mut self, memory_estimation_in_bytes: MemoryRange) {
        let inner = self.inner.lock().unwrap();
        inner.base_task.set_estimated_memory_range_in_bytes(
            memory_estimation_in_bytes.min(),
            memory_estimation_in_bytes.max(),
        );
    }

    fn requested_concurrency(&mut self, concurrency: Concurrency) {
        let inner = self.inner.lock().unwrap();
        inner.base_task.set_max_concurrency(concurrency.value());
    }

    fn begin_subtask(&mut self) {
        let mut inner = self.inner.lock().unwrap();

        if !inner.task_registry.contains_task(&inner.base_task) {
            inner.task_registry.register_task(inner.base_task.clone());
        }

        let parent_task = inner.nested_tasks.last().cloned();

        let next_task = if let Some(task) = inner.current_task.take() {
            inner.nested_tasks.push(task.clone());

            match task.next_subtask() {
                Some(next) => next.as_ref().clone(),
                None => {
                    if !inner.did_log_error {
                        inner.task_progress_logger.log_warning(&format!(
                            ":: No pending subtasks available for '{}'",
                            task.description()
                        ));
                        inner.did_log_error = true;
                    }
                    inner.base_task.clone()
                }
            }
        } else {
            inner.base_task.clone()
        };

        // Only start the task if it has not already been started. In some edge
        // cases (e.g., when there are no pending subtasks for a running task) we
        // may end up re-selecting the base task which can already be in the
        // Running state; calling `start()` on a running task panics, so guard
        // against that.
        if next_task.status() == Status::Pending {
            next_task.start();
        } else {
            inner
                .task_progress_logger
                .log_warning(&format!(
                    ":: Attempted to start task '{}' but it is already {:?}",
                    next_task.description(),
                    next_task.status()
                ));
        }

        inner
            .task_progress_logger
            .log_begin_subtask(&next_task, parent_task.as_ref());

        inner.current_task = Some(next_task);
        inner.current_total_steps = UNKNOWN_STEPS;
        inner.progress_leftovers = 0.0;
    }

    fn end_subtask(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        if inner.current_task.is_none() {
            if !inner.did_log_error {
                inner.task_progress_logger.log_warning(
                    ":: Tried to log progress, but there are no running tasks being tracked",
                );
                inner.did_log_error = true;
            }
            return;
        }

        let parent_task = inner.nested_tasks.last().cloned();
        let task = inner.current_task.take().unwrap();

        inner
            .task_progress_logger
            .log_end_subtask(&task, parent_task.as_ref());
        task.finish();

        if inner.nested_tasks.is_empty() {
            inner.task_registry.unregister_task();
            inner.task_progress_logger.release();

            if inner.base_task.status() == Status::Running {
                let message = format!(
                    "Attempted to release algorithm, but task {} is still running",
                    inner.base_task.description()
                );
                debug_assert!(false, "{}", message);
                inner
                    .task_progress_logger
                    .log_warning(&format!(":: {}", message));
            }
        } else {
            inner.current_task = inner.nested_tasks.pop();
        }
    }

    fn end_subtask_with_failure(&mut self) {
        // This mirrors Java’s recursive unwind on failure.
        let mut inner = self.inner.lock().unwrap();
        if inner.current_task.is_none() {
            if !inner.did_log_error {
                inner.task_progress_logger.log_warning(
                    ":: Tried to log progress, but there are no running tasks being tracked",
                );
                inner.did_log_error = true;
            }
            return;
        }

        let parent_task = inner.nested_tasks.last().cloned();
        let task = inner.current_task.take().unwrap();
        task.fail();
        inner
            .task_progress_logger
            .log_end_subtask_with_failure(&task, parent_task.as_ref());

        if inner.nested_tasks.is_empty() {
            inner.task_registry.unregister_task();
            inner.task_progress_logger.release();

            if inner.base_task.status() == Status::Running {
                let message = format!(
                    "Attempted to release algorithm, but task {} is still running",
                    inner.base_task.description()
                );
                debug_assert!(false, "{}", message);
                inner
                    .task_progress_logger
                    .log_warning(&format!(":: {}", message));
            }
        } else {
            inner.current_task = inner.nested_tasks.pop();
            drop(inner);
            self.end_subtask_with_failure();
        }
    }

    fn log_progress(&mut self, value: usize) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(task) = &inner.current_task {
            task.log_progress(value);
            inner.task_progress_logger.log_progress(value);
        } else if !inner.did_log_error {
            inner.task_progress_logger.log_warning(
                ":: Tried to log progress, but there are no running tasks being tracked",
            );
            inner.did_log_error = true;
        }
    }

    fn log_progress_with_message(&mut self, value: usize, message_template: &str) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(task) = &inner.current_task {
            task.log_progress(value);
            let msg = Self::format_progress_message(message_template, value);
            inner.task_progress_logger.log_message(&msg);
        } else if !inner.did_log_error {
            inner.task_progress_logger.log_warning(
                ":: Tried to log progress, but there are no running tasks being tracked",
            );
            inner.did_log_error = true;
        }
    }

    fn set_volume(&mut self, volume: usize) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(task) = &inner.current_task {
            task.set_volume(volume);
            inner.task_progress_logger.reset(volume);
        } else if !inner.did_log_error {
            inner.task_progress_logger.log_warning(
                ":: Tried to log progress, but there are no running tasks being tracked",
            );
            inner.did_log_error = true;
        }
    }

    fn current_volume(&mut self) -> usize {
        let mut inner = self.inner.lock().unwrap();
        if let Some(task) = &inner.current_task {
            task.get_progress().volume()
        } else {
            if !inner.did_log_error {
                inner.task_progress_logger.log_warning(
                    ":: Tried to log progress, but there are no running tasks being tracked",
                );
                inner.did_log_error = true;
            }
            UNKNOWN_VOLUME
        }
    }

    fn log_debug(&mut self, message_supplier: &dyn Fn() -> String) {
        let mut inner = self.inner.lock().unwrap();
        inner.task_progress_logger.log_debug_lazy(message_supplier);
    }

    fn log_message(&mut self, level: LogLevel, message: &str) {
        let mut inner = self.inner.lock().unwrap();
        match level {
            LogLevel::Warning => inner
                .task_progress_logger
                .log_warning(&format!(":: {}", message)),
            LogLevel::Info => inner
                .task_progress_logger
                .log_message(&format!(":: {}", message)),
            LogLevel::Debug => inner
                .task_progress_logger
                .log_debug(&format!(":: {}", message)),
        }
    }

    fn release(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        inner.task_registry.unregister_task();
        inner.task_progress_logger.release();

        if inner.base_task.status() == Status::Running {
            let message = format!(
                "Attempted to release algorithm, but task {} is still running",
                inner.base_task.description()
            );

            debug_assert!(false, "{}", message);
            inner
                .task_progress_logger
                .log_warning(&format!(":: {}", message));
        }
    }

    fn set_steps(&mut self, steps: usize) {
        let mut inner = self.inner.lock().unwrap();
        if steps == 0 {
            if !inner.did_log_error {
                inner
                    .task_progress_logger
                    .log_warning(":: Total steps for task must be at least 1 but was 0");
                inner.did_log_error = true;
            }
            return;
        }
        inner.current_total_steps = steps as isize;
    }

    fn log_steps(&mut self, steps: usize) {
        // Compute conversion under lock, then call `log_progress` outside lock.
        let (to_log, should_warn) = {
            let mut inner = self.inner.lock().unwrap();

            let Some(task) = &inner.current_task else {
                let should_warn = !inner.did_log_error;
                if should_warn {
                    inner.task_progress_logger.log_warning(
                        ":: Tried to log progress, but there are no running tasks being tracked",
                    );
                    inner.did_log_error = true;
                }
                return;
            };

            if inner.current_total_steps == UNKNOWN_STEPS {
                let should_warn = !inner.did_log_error;
                if should_warn {
                    inner
                        .task_progress_logger
                        .log_warning(":: Tried to log steps without setting total steps");
                    inner.did_log_error = true;
                }
                return;
            }

            let volume = task.get_progress().volume();
            if volume == UNKNOWN_VOLUME {
                (steps, false)
            } else {
                let progress = (steps as f64) * (volume as f64)
                    / (inner.current_total_steps as f64)
                    + inner.progress_leftovers;

                let whole = progress.floor() as usize;
                inner.progress_leftovers = progress - (whole as f64);
                (whole, false)
            }
        };

        let _ = should_warn;
        if to_log > 0 {
            self.log_progress(to_log);
        }
    }

    fn assert_subtask(&mut self, expected_task_description_substring: &str) {
        let inner = self.inner.lock().unwrap();
        if let Some(task) = &inner.current_task {
            let current = task.description();
            debug_assert!(
                current.contains(expected_task_description_substring),
                "Expected task name to contain `{}`, but was `{}`",
                expected_task_description_substring,
                current
            );
        }
    }
}
