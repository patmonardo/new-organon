//! Best-practice walkthrough for the new `ProgressTracker` / `TaskProgressTracker`.
//!
//! This example demonstrates:
//! - Building a task tree up-front (root + leaf subtasks)
//! - Using `begin_subtask_with_description*` to enforce correct traversal
//! - Setting volume early (or as soon as it becomes known)
//! - Using `set_steps` + `log_steps` when you want step-based reporting
//! - Registering/unregistering tasks via a `TaskRegistryFactory` + `TaskStore`
//!
//! Run with:
//! `cargo run -p gds --example progress_tracker_best_practices`

use gds::concurrency::Concurrency;
use gds::core::utils::progress::{
    JobId, PerDatabaseTaskStore, ProgressTracker, TaskProgressTracker, TaskRegistryFactories,
    TaskStore, TaskStoreListener, Tasks, UserTask,
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

    // Best practice: define the full task tree upfront.
    // Each leaf corresponds to a meaningful phase of the algorithm.
    let load_input = Arc::new(
        Tasks::leaf_with_volume("Load input".to_string(), 100)
            .base()
            .clone(),
    );
    let compute = Arc::new(Tasks::leaf("Compute".to_string()).base().clone());
    let write_output = Arc::new(
        Tasks::leaf_with_volume("Write output".to_string(), 10)
            .base()
            .clone(),
    );

    let root = Tasks::task(
        "Best-practice demo".to_string(),
        vec![load_input, compute, write_output],
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

    // Start root task.
    tracker.begin_subtask_with_description("Best-practice demo");
    println!("[TASK] after root begin task_count={}", store.task_count());

    // 1) Load input: volume is known up-front via leaf factory.
    tracker.begin_subtask_with_description("Load input");
    tracker.log_progress_with_message(25, "loaded %d items");
    tracker.log_progress(75);
    tracker.end_subtask_with_description("Load input");

    // 2) Compute: volume is unknown at task construction time; set it when it becomes known.
    tracker.begin_subtask_with_description("Compute");
    tracker.log_info("planning computation");

    tracker.set_volume(200);

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
    tracker.end_subtask_with_description("Best-practice demo");

    println!("[TASK] task tree (finished):\n{}", root.render());
    println!("[TASK] final store task_count={}", store.task_count());
    println!(
        "[TASK] store query after completion: {:?}",
        store
            .query(&username, &job_id)
            .map(|ut| ut.task.description().to_string())
    );
}
