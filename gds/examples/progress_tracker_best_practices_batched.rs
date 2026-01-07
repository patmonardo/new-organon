//! Best-practice walkthrough for `ProgressTracker` / `TaskProgressTracker`, using batching.
//!
//! This is a clone of `progress_tracker_best_practices`, but wraps the underlying
//! `TaskProgressTracker` with `BatchingTaskProgressTracker`.
//!
//! Why this matters for `algo/` / `procedures/` call sites:
//! - The code still programs against the `ProgressTracker` trait.
//! - The batching wrapper only changes performance characteristics (fewer delegate calls).
//! - You can enable/disable batching at construction time without touching inner algorithm code.
//!
//! Run with:
//! `cargo run -p gds --example progress_tracker_best_practices_batched`

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

    // Use larger volumes here so batching is meaningfully exercised.
    // (With very small volumes, batch size collapses to 1 and batching becomes a no-op.)
    let load_input = Arc::new(
        Tasks::leaf_with_volume("Load input".to_string(), 1_000_000)
            .base()
            .clone(),
    );
    let compute = Arc::new(Tasks::leaf("Compute".to_string()).base().clone());
    let write_output = Arc::new(
        Tasks::leaf_with_volume("Write output".to_string(), 100_000)
            .base()
            .clone(),
    );

    let root = Tasks::task(
        "Best-practice demo (batched)".to_string(),
        vec![load_input, compute, write_output],
    );

    // Construct the real tracker first.
    let base_tracker = TaskProgressTracker::with_registry(
        root.clone(),
        Concurrency::of(4),
        job_id.clone(),
        registry_factory.as_ref(),
    );

    // IMPORTANT: the root volume is `UNKNOWN_VOLUME` because `Compute` starts unknown.
    // BatchingTaskProgressTracker only needs a volume estimate to size its flush batches;
    // it does not affect correctness (only performance).
    let estimated_total_volume = 3_100_000;
    let computed_batch_size =
        BatchingProgressLogger::calculate_batch_size_for_volume(estimated_total_volume as u64, 4);

    let mut tracker =
        BatchingTaskProgressTracker::new(base_tracker, estimated_total_volume, Concurrency::of(4));

    tracker.set_estimated_resource_footprint(MemoryRange::of_range(25_000_000, 75_000_000));
    tracker.requested_concurrency(Concurrency::of(4));

    println!(
        "[TASK] batching enabled estimated_total_volume={} computed_batch_size={}",
        estimated_total_volume, computed_batch_size
    );

    println!("[TASK] initial store task_count={}", store.task_count());
    println!("[TASK] task tree (initial):\n{}", root.render());

    // Start root task.
    tracker.begin_subtask_with_description("Best-practice demo (batched)");
    println!("[TASK] after root begin task_count={}", store.task_count());

    // 1) Load input: volume known up-front via leaf factory.
    tracker.begin_subtask_with_description("Load input");
    tracker.log_progress_with_message(250_000, "loaded %d items");
    tracker.log_progress(750_000);
    tracker.end_subtask_with_description("Load input");

    // 2) Compute: volume unknown at task construction time; set it when it becomes known.
    tracker.begin_subtask_with_description("Compute");
    tracker.log_info("planning computation");

    tracker.set_volume(2_000_000);

    // If you want step-based progress, set total steps first, then log steps.
    tracker.set_steps(4);
    for step in 1..=4 {
        tracker.log_debug(&|| format!("executing compute step {step}/4"));
        tracker.log_steps(1);
    }

    tracker.end_subtask_with_description("Compute");

    // 3) Write output: demonstrate steps-to-volume conversion.
    tracker.begin_subtask_with_description("Write output");
    tracker.set_steps(5);
    for _ in 0..5 {
        tracker.log_steps(1);
    }
    tracker.end_subtask_with_description("Write output");

    // Finish root task.
    tracker.end_subtask_with_description("Best-practice demo (batched)");

    println!("[TASK] task tree (finished):\n{}", root.render());
    println!("[TASK] final store task_count={}", store.task_count());
    println!(
        "[TASK] store query after completion: {:?}",
        store
            .query(&username, &job_id)
            .map(|ut| ut.task.description().to_string())
    );
}
