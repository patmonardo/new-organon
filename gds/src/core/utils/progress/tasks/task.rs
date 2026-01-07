use super::{Progress, Status, TaskVisitor};
use crate::core::utils::clock_service::ClockService;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// Marker for unknown task volume.
///
/// Java parity: this constant lives on `Task` (`Task.UNKNOWN_VOLUME`).
pub const UNKNOWN_VOLUME: usize = usize::MAX;

/// Base class for all tasks in the progress tracking system.
/// Handles task hierarchy, state management, timing, and memory estimation.
#[derive(Clone, Debug)]
pub struct Task {
    description: String,
    sub_tasks: Vec<Arc<Task>>,
    leaf_state: Option<Arc<LeafState>>,
    status: Arc<Mutex<Status>>,
    start_time: Arc<Mutex<u64>>,
    finish_time: Arc<Mutex<u64>>,
    estimated_memory_range_in_bytes: Arc<Mutex<(usize, usize)>>,
    max_concurrency: Arc<Mutex<usize>>,
}

#[derive(Debug)]
struct LeafState {
    volume: Mutex<usize>,
    current_progress: AtomicUsize,
}

impl Task {
    pub const UNKNOWN_CONCURRENCY: usize = usize::MAX;
    // Java parity: Java uses -1 for these. In Rust (u64) use a sentinel that cannot
    // collide with `ClockService::clock().millis()`.
    pub const NOT_STARTED: u64 = u64::MAX;
    pub const NOT_FINISHED: u64 = u64::MAX;

    /// Create a new task with description and subtasks.
    pub fn new(description: String, sub_tasks: Vec<Arc<Task>>) -> Self {
        Self {
            description,
            sub_tasks,
            leaf_state: None,
            status: Arc::new(Mutex::new(Status::Pending)),
            start_time: Arc::new(Mutex::new(Self::NOT_STARTED)),
            finish_time: Arc::new(Mutex::new(Self::NOT_FINISHED)),
            estimated_memory_range_in_bytes: Arc::new(Mutex::new((0, 0))),
            max_concurrency: Arc::new(Mutex::new(Self::UNKNOWN_CONCURRENCY)),
        }
    }

    /// Create a leaf task (terminal node) with progress tracking.
    pub fn leaf(description: String, volume: usize) -> Self {
        Self {
            description,
            sub_tasks: vec![],
            leaf_state: Some(Arc::new(LeafState {
                volume: Mutex::new(volume),
                current_progress: AtomicUsize::new(0),
            })),
            status: Arc::new(Mutex::new(Status::Pending)),
            start_time: Arc::new(Mutex::new(Self::NOT_STARTED)),
            finish_time: Arc::new(Mutex::new(Self::NOT_FINISHED)),
            estimated_memory_range_in_bytes: Arc::new(Mutex::new((0, 0))),
            max_concurrency: Arc::new(Mutex::new(Self::UNKNOWN_CONCURRENCY)),
        }
    }

    /// Whether this task is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.leaf_state.is_some()
    }

    /// Get task description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get subtasks.
    pub fn sub_tasks(&self) -> &[Arc<Task>] {
        &self.sub_tasks
    }

    /// Get current status.
    pub fn status(&self) -> Status {
        *self.status.lock().unwrap()
    }

    /// Get next subtask to execute.
    pub fn next_subtask(&self) -> Option<Arc<Task>> {
        self.validate_task_is_running();
        self.next_subtask_after_validation()
    }

    /// Start task execution.
    pub fn start(&self) {
        let current_status = self.status();
        if current_status != Status::Pending {
            panic!(
                "Cannot start task '{}' with status {:?}. Task must have status Pending.",
                self.description, current_status
            );
        }

        *self.status.lock().unwrap() = Status::Running;
        *self.start_time.lock().unwrap() = ClockService::clock().millis();
    }

    /// Finish task successfully.
    pub fn finish(&self) {
        let current_status = self.status();
        if current_status != Status::Running {
            panic!(
                "Task '{}' with state {:?} cannot be finished",
                self.description, current_status
            );
        }

        if let Some(leaf) = &self.leaf_state {
            // Leaf tasks should be considered 100% complete on finish.
            let mut volume = leaf.volume.lock().unwrap();
            if *volume == UNKNOWN_VOLUME {
                *volume = leaf.current_progress.load(Ordering::Relaxed);
            }

            let current = leaf.current_progress.load(Ordering::Relaxed);
            let remaining = volume.saturating_sub(current);
            leaf.current_progress
                .fetch_add(remaining, Ordering::Relaxed);
        }

        *self.status.lock().unwrap() = Status::Finished;
        *self.finish_time.lock().unwrap() = ClockService::clock().millis();
    }

    /// Cancel task execution.
    pub fn cancel(&self) {
        let current_status = self.status();
        if current_status == Status::Finished {
            panic!(
                "Task '{}' with state {:?} cannot be canceled",
                self.description, current_status
            );
        }

        // Java parity: cancel is allowed for any non-finished status.
        *self.status.lock().unwrap() = Status::Canceled;
    }

    /// Mark task as failed.
    pub fn fail(&self) {
        *self.status.lock().unwrap() = Status::Failed;
    }

    /// Get current progress.
    pub fn get_progress(&self) -> Progress {
        if let Some(leaf) = &self.leaf_state {
            let volume = *leaf.volume.lock().unwrap();
            let current = leaf.current_progress.load(Ordering::Relaxed);
            return Progress::of(current, volume);
        }

        // Java parity: an intermediate task with no subtasks has 0/0 progress.
        if self.sub_tasks.is_empty() {
            return Progress::of(0, 0);
        }

        let mut progress = 0usize;
        let mut volume = 0usize;
        let mut has_unknown_volume = false;

        for sub_task in &self.sub_tasks {
            let sub_progress = sub_task.get_progress();

            if sub_progress.volume() == UNKNOWN_VOLUME {
                has_unknown_volume = true;
                // Keep summing absolute progress, but the aggregated volume becomes unknown.
            }

            progress += sub_progress.progress();
            if !has_unknown_volume {
                volume += sub_progress.volume();
            }
        }

        if has_unknown_volume {
            Progress::of(progress, UNKNOWN_VOLUME)
        } else {
            Progress::of(progress, volume)
        }
    }

    /// Set task volume.
    ///
    /// Java parity: only valid for leaf tasks.
    pub fn set_volume(&self, volume: usize) {
        let Some(leaf) = &self.leaf_state else {
            panic!(
                "Should only be called on a leaf task, but task '{}' is not a leaf",
                self.description
            );
        };
        *leaf.volume.lock().unwrap() = volume;
    }

    /// Log progress.
    ///
    /// Java parity: only valid for leaf tasks.
    pub fn log_progress(&self, value: usize) {
        let Some(leaf) = &self.leaf_state else {
            panic!(
                "Should only be called on a leaf task, but task '{}' is not a leaf",
                self.description
            );
        };
        leaf.current_progress.fetch_add(value, Ordering::Relaxed);
    }

    /// Get the leaf task volume if this task is a leaf, else `UNKNOWN_VOLUME`.
    pub fn current_volume(&self) -> usize {
        self.leaf_state
            .as_ref()
            .map(|leaf| *leaf.volume.lock().unwrap())
            .unwrap_or(UNKNOWN_VOLUME)
    }

    /// Reset leaf progress to zero.
    pub fn reset_progress(&self) {
        let Some(leaf) = &self.leaf_state else {
            return;
        };
        leaf.current_progress.store(0, Ordering::Relaxed);
    }

    /// Accept a visitor (Visitor pattern).
    pub fn visit(&self, task_visitor: &dyn TaskVisitor) {
        task_visitor.visit_intermediate_task(self);
    }

    /// Get start time in milliseconds.
    pub fn start_time(&self) -> u64 {
        *self.start_time.lock().unwrap()
    }

    /// Get finish time in milliseconds.
    pub fn finish_time(&self) -> u64 {
        *self.finish_time.lock().unwrap()
    }

    /// Check if task has not started yet.
    pub fn has_not_started(&self) -> bool {
        self.status() == Status::Pending || self.start_time() == Self::NOT_STARTED
    }

    /// Get estimated memory range in bytes.
    pub fn estimated_memory_range_in_bytes(&self) -> (usize, usize) {
        *self.estimated_memory_range_in_bytes.lock().unwrap()
    }

    /// Get maximum concurrency.
    pub fn max_concurrency(&self) -> usize {
        *self.max_concurrency.lock().unwrap()
    }

    /// Set estimated memory range.
    pub fn set_estimated_memory_range_in_bytes(&self, min: usize, max: usize) {
        *self.estimated_memory_range_in_bytes.lock().unwrap() = (min, max);
    }

    /// Set maximum concurrency.
    pub fn set_max_concurrency(&self, concurrency: usize) {
        *self.max_concurrency.lock().unwrap() = concurrency;

        // Java parity: propagate to children that have unknown concurrency.
        for sub_task in &self.sub_tasks {
            if sub_task.max_concurrency() == Self::UNKNOWN_CONCURRENCY {
                sub_task.set_max_concurrency(concurrency);
            }
        }
    }

    /// Render a textual representation of the task tree.
    ///
    /// Java parity: mirrors `Task.render()` formatting (tabs + `|-- ` prefixes).
    pub fn render(&self) -> String {
        let mut out = String::new();
        Self::render_into(&mut out, self, 0);
        out
    }

    fn render_into(out: &mut String, task: &Task, depth: usize) {
        if depth > 1 {
            out.push_str(&"\t".repeat(depth - 1));
        }

        if depth > 0 {
            out.push_str("|-- ");
        }

        out.push_str(task.description());
        out.push('(');
        out.push_str(&task.status().to_string());
        out.push(')');
        out.push('\n');

        for subtask in task.sub_tasks() {
            Self::render_into(out, subtask, depth + 1);
        }
    }

    /// Get next subtask after validation (can be overridden).
    fn next_subtask_after_validation(&self) -> Option<Arc<Task>> {
        // Java parity: fail fast if any subtask is still running.
        if self.sub_tasks.iter().any(|t| t.status() == Status::Running) {
            panic!("Cannot move to next subtask, because some subtasks are still running");
        }

        for sub_task in &self.sub_tasks {
            if sub_task.status() == Status::Pending {
                return Some(Arc::clone(sub_task));
            }
        }
        None
    }

    fn validate_task_is_running(&self) {
        if self.status() != Status::Running {
            panic!(
                "Cannot retrieve next subtask, task '{}' is not running.",
                self.description
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new("Test Task".to_string(), vec![]);
        assert_eq!(task.description(), "Test Task");
        assert_eq!(task.status(), Status::Pending);
        assert_eq!(task.sub_tasks().len(), 0);
    }

    #[test]
    fn test_task_lifecycle() {
        let task = Task::new("Lifecycle Task".to_string(), vec![]);

        assert_eq!(task.status(), Status::Pending);
        assert!(task.has_not_started());

        task.start();
        assert_eq!(task.status(), Status::Running);
        assert!(!task.has_not_started());
        assert!(task.start_time() > 0);

        task.finish();
        assert_eq!(task.status(), Status::Finished);
        assert!(task.finish_time() > 0);
    }

    #[test]
    #[should_panic(expected = "Cannot start task")]
    fn test_task_cannot_start_twice() {
        let task = Task::new("Double Start".to_string(), vec![]);
        task.start();
        task.start(); // Should panic
    }

    #[test]
    fn test_task_cancellation() {
        let task = Task::new("Cancel Task".to_string(), vec![]);
        task.cancel();
        assert_eq!(task.status(), Status::Canceled);
        // Java parity: cancel does not set finish time.
        assert_eq!(task.finish_time(), Task::NOT_FINISHED);
    }

    #[test]
    fn test_task_failure() {
        let task = Task::new("Fail Task".to_string(), vec![]);
        task.fail();
        assert_eq!(task.status(), Status::Failed);
    }

    #[test]
    fn test_task_with_subtasks() {
        let sub1 = Arc::new(Task::new("Sub 1".to_string(), vec![]));
        let sub2 = Arc::new(Task::new("Sub 2".to_string(), vec![]));
        let parent = Task::new("Parent".to_string(), vec![sub1.clone(), sub2.clone()]);

        assert_eq!(parent.sub_tasks().len(), 2);

        parent.start();

        let next = parent.next_subtask();
        assert!(next.is_some());
        assert_eq!(next.unwrap().description(), "Sub 1");
    }

    #[test]
    fn test_task_progress_aggregation() {
        let sub1 = Arc::new(Task::new("Sub 1".to_string(), vec![]));
        let sub2 = Arc::new(Task::new("Sub 2".to_string(), vec![]));
        let parent = Task::new("Parent".to_string(), vec![sub1, sub2]);

        let progress = parent.get_progress();
        // Java parity: empty intermediate children contribute 0/0.
        assert_eq!(progress.volume(), 0);
        assert_eq!(progress.progress(), 0);
    }

    #[test]
    fn test_task_memory_estimation() {
        let task = Task::new("Memory Task".to_string(), vec![]);
        task.set_estimated_memory_range_in_bytes(1024, 2048);

        let (min, max) = task.estimated_memory_range_in_bytes();
        assert_eq!(min, 1024);
        assert_eq!(max, 2048);
    }

    #[test]
    fn test_task_concurrency() {
        let task = Task::new("Concurrent Task".to_string(), vec![]);
        assert_eq!(task.max_concurrency(), Task::UNKNOWN_CONCURRENCY);

        task.set_max_concurrency(8);
        assert_eq!(task.max_concurrency(), 8);
    }
}
