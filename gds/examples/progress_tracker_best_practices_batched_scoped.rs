//! Best-practice walkthrough for `ProgressTracker` / `TaskProgressTracker`, using *scoped* batching.
//!
//! This is the “recommended in hot loops” pattern:
//! - Keep a normal `TaskProgressTracker` as the owner of task/registry state.
//! - Wrap it in `BatchingTaskProgressTracker` only for the tight-loop phase.
//! - Unwrap via `into_inner()` when you leave that phase.
//!
//! Why this matters for `algo/` / `procedures/` call sites:
//! - Inner algorithm code can stay generic over `ProgressTracker`.
//! - You can enable batching per phase without changing the task tree shape.
//! - The batching volume can match the *leaf* volume (better batch sizing).
//!
//! Run with:
//! `cargo run -p gds --example progress_tracker_best_practices_batched_scoped`

use gds::concurrency::Concurrency;
use gds::core::utils::progress::{
    BatchingProgressLogger, BatchingTaskProgressTracker, JobId, PerDatabaseTaskStore,
    ProgressTracker, TaskProgressTracker, TaskRegistryFactories, TaskStore, TaskStoreListener,
    Tasks, UserTask,
};
use gds::mem::MemoryRange;
use std::sync::{Arc, Mutex};

struct PrintingListener {
    events: Mutex<Vec<String>>,
}

impl PrintingListener {
    fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
        }
    }
}

impl TaskStoreListener for PrintingListener {
    fn on_task_added(&self, user_task: &UserTask) {
        let msg = format!(
            "[STORE] added username={} job_id={} task='{}' status={:?}",
            user_task.username,
            user_task.job_id.as_string(),
            user_task.task.description(),
            user_task.task.status(),
        );
        println!("{msg}");
        self.events.lock().unwrap().push(msg);
    }

    fn on_task_removed(&self, username: &str, job_id: &JobId) {
        let msg = format!(
            "[STORE] removed username={} job_id={}",
            username,
            job_id.as_string()
        );
        println!("{msg}");
        self.events.lock().unwrap().push(msg);
    }

    fn on_store_cleared(&self) {
        let msg = "[STORE] cleared".to_string();
        println!("{msg}");
        self.events.lock().unwrap().push(msg);
    }
}

fn main() {
    let store = Arc::new(PerDatabaseTaskStore::new());
    store.add_listener(Box::new(PrintingListener::new()));

    let username = "alice".to_string();
    let job_id = JobId::new();
    let registry_factory = TaskRegistryFactories::local(username.clone(), store.clone());

    // Full task tree up-front.
    let load_input = Arc::new(
        Tasks::leaf_with_volume("Load input".to_string(), 1_000_000)
            .base()
            .clone(),
    );

    // Compute will be the “hot loop” phase; we’ll batch only inside it.
    let compute = Arc::new(
        Tasks::leaf_with_volume("Compute".to_string(), 2_000_000)
            .base()
            .clone(),
    );

    let write_output = Arc::new(
        Tasks::leaf_with_volume("Write output".to_string(), 100_000)
            .base()
            .clone(),
    );

    let root = Tasks::task(
        "Best-practice demo (scoped batching)".to_string(),
        vec![load_input, compute.clone(), write_output],
    );

    let mut tracker = TaskProgressTracker::with_registry(
        root.clone(),
        Concurrency::of(4),
        job_id.clone(),
        registry_factory.as_ref(),
    );

    tracker.set_estimated_resource_footprint(MemoryRange::of_range(25_000_000, 75_000_000));
    tracker.requested_concurrency(Concurrency::of(4));

    println!("[TASK] initial store task_count={}", store.task_count());
    println!("[TASK] task tree (initial):\n{}", root.render());

    tracker.begin_subtask_with_description("Best-practice demo (scoped batching)");

    // Phase 1: Load input (no batching here).
    tracker.begin_subtask_with_description("Load input");
    tracker.log_progress_with_message(250_000, "loaded %d items");
    tracker.log_progress(750_000);
    tracker.end_subtask_with_description("Load input");

    // Phase 2: Compute (batch only in this tight-loop phase).
    tracker.begin_subtask_with_description("Compute");

    let compute_volume = compute.get_progress().volume();
    let computed_batch_size =
        BatchingProgressLogger::calculate_batch_size_for_volume(compute_volume as u64, 4);
    println!(
        "[TASK] compute batching compute_volume={} computed_batch_size={}",
        compute_volume, computed_batch_size
    );

    // Wrap only for this phase.
    let mut tracker = BatchingTaskProgressTracker::new(tracker, compute_volume, Concurrency::of(4));

    // Simulate a hot loop (lots of tiny increments) without spamming delegate calls.
    // Note: log_progress_with_message and subtask boundaries flush.
    tracker.log_info("running compute hot loop");
    for _ in 0..2_000_000 {
        tracker.log_progress(1);
    }

    // Unwrap back to the base tracker so the outer code does not need to care.
    let mut tracker = tracker.into_inner();

    tracker.end_subtask_with_description("Compute");

    // Phase 3: Write output (no batching).
    tracker.begin_subtask_with_description("Write output");
    tracker.set_steps(5);
    for _ in 0..5 {
        tracker.log_steps(1);
    }
    tracker.end_subtask_with_description("Write output");

    tracker.end_subtask_with_description("Best-practice demo (scoped batching)");

    println!("[TASK] task tree (finished):\n{}", root.render());
    println!("[TASK] final store task_count={}", store.task_count());
    println!(
        "[TASK] store query after completion: {:?}",
        store
            .query(&username, &job_id)
            .map(|ut| ut.task.description().to_string())
    );
}
