//! TaskProgressLogger (Java GDS parity direction).
//!
//! Java’s `TaskProgressLogger` extends `BatchingProgressLogger` and adds
//! task-hierarchy-aware begin/end logging.

use crate::core::utils::progress::{BatchingProgressLogger, ProgressLogger};

use super::Task;

pub struct TaskProgressLogger {
    base_task: Task,
    logger: BatchingProgressLogger,
}

impl TaskProgressLogger {
    pub fn new(base_task: Task, concurrency: usize) -> Self {
        let volume = base_task.get_progress().volume();
        let logger =
            BatchingProgressLogger::new(base_task.description().to_string(), volume, concurrency);

        Self { base_task, logger }
    }

    pub fn log_begin_subtask(&mut self, task: &Task, parent_task: Option<&Task>) {
        let task_name = self.task_description(task, parent_task);

        if parent_task.is_none() {
            self.logger.log_start(&task_name);
        } else {
            self.logger.start_subtask(&task_name);
        }

        self.logger.reset(task.get_progress().volume() as i64);
    }

    pub fn log_end_subtask(&mut self, task: &Task, parent_task: Option<&Task>) {
        let task_name = self.task_description(task, parent_task);

        self.log_100_on_leaf_task_finish(task);

        if parent_task.is_none() {
            self.logger.log_finish(&task_name);
        } else {
            self.logger.finish_subtask(&task_name);
        }
    }

    pub fn log_end_subtask_with_failure(&mut self, task: &Task, parent_task: Option<&Task>) {
        let task_name = self.task_description(task, parent_task);

        self.log_100_on_leaf_task_finish(task);

        if parent_task.is_none() {
            self.logger.log_finish_with_failure(&task_name);
        } else {
            self.logger.log_finish_subtask_with_failure(&task_name);
        }
    }

    pub fn log_progress(&mut self, value: usize) {
        self.logger.log_progress_amount(value as i64);
    }

    pub fn reset(&mut self, volume: usize) {
        self.logger.reset(volume as i64);
    }

    pub fn log_message(&mut self, msg: &str) {
        self.logger.log_message(msg);
    }

    pub fn log_warning(&mut self, msg: &str) {
        self.logger.log_warning(msg);
    }

    pub fn log_debug(&mut self, msg: &str) {
        self.logger.log_debug(msg);
    }

    pub fn log_debug_lazy(&mut self, supplier: &dyn Fn() -> String) {
        self.logger.log_debug(&supplier());
    }

    pub fn release(&mut self) {
        self.logger.release();
    }

    fn task_description(&self, task: &Task, _parent_task: Option<&Task>) -> String {
        // Java treats the base task as having an empty name.
        if task.description() == self.base_task.description() {
            String::new()
        } else {
            task.description().to_string()
        }
    }

    fn log_100_on_leaf_task_finish(&mut self, task: &Task) {
        if task.is_leaf() {
            self.logger.log_finish_percentage();
        }
    }
}
