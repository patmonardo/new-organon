//! Walkthrough of TaskRegistry / TaskStore / TaskStoreService behavior.
//!
//! This example demonstrates:
//! - TaskStoreService enabled/disabled behavior
//! - Per-database isolation via database-specific task stores
//! - Listener callbacks for task add/remove
//! - How `TaskProgressTracker` registers/unregisters tasks in the store
//!
//! Run with:
//! `cargo run -p gds --example task_registry_store_walkthrough`

use gds::concurrency::Concurrency;
use gds::core::utils::progress::{
    JobId, ProgressTracker, TaskProgressTracker, TaskRegistryFactories, TaskStore,
    TaskStoreListener, TaskStoreService, Tasks, UserTask,
};
use std::sync::{Arc, Mutex};

struct PrintingListener {
    label: &'static str,
    events: Mutex<Vec<String>>,
}

impl PrintingListener {
    fn new(label: &'static str) -> Self {
        Self {
            label,
            events: Mutex::new(Vec::new()),
        }
    }
}

impl TaskStoreListener for PrintingListener {
    fn on_task_added(&self, user_task: &UserTask) {
        let msg = format!(
            "[{}] added username={} job_id={} task='{}' status={:?}",
            self.label,
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
            "[{}] removed username={} job_id={}",
            self.label,
            username,
            job_id.as_string()
        );
        println!("{msg}");
        self.events.lock().unwrap().push(msg);
    }

    fn on_store_cleared(&self) {
        let msg = format!("[{}] cleared", self.label);
        println!("{msg}");
        self.events.lock().unwrap().push(msg);
    }
}

fn describe_store(store: &Arc<dyn TaskStore>, label: &str) {
    println!(
        "[STORE:{}] is_empty={} task_count={}",
        label,
        store.is_empty(),
        store.task_count()
    );
}

fn main() {
    let username = "alice".to_string();

    // 1) Enabled service: returns real per-database stores.
    let service_enabled = TaskStoreService::new(true);

    let db1 = "registry_store_example_db1";
    let db2 = "registry_store_example_db2";

    service_enabled.purge_database(db1);
    service_enabled.purge_database(db2);

    let store_db1 = service_enabled.get_task_store(db1);
    let store_db2 = service_enabled.get_task_store(db2);

    store_db1.add_listener(Box::new(PrintingListener::new("db1")));
    store_db2.add_listener(Box::new(PrintingListener::new("db2")));

    println!(
        "[SERVICE] enabled databases={:?}",
        service_enabled.database_names()
    );
    describe_store(&store_db1, db1);
    describe_store(&store_db2, db2);

    // Use a registry factory bound to (username, store_db1).
    let job_id = JobId::new();
    let registry_factory = TaskRegistryFactories::local(username.clone(), store_db1.clone());

    let root_task = Tasks::leaf_with_volume("Registry demo task".to_string(), 3)
        .base()
        .clone();

    let mut tracker = TaskProgressTracker::with_registry(
        root_task,
        Concurrency::of(1),
        job_id.clone(),
        registry_factory.as_ref(),
    );

    // Task registration happens on the first `begin_subtask()`.
    println!(
        "[QUERY] before begin: {:?}",
        store_db1
            .query(&username, &job_id)
            .map(|ut| ut.task.description().to_string())
    );

    tracker.begin_subtask_with_description("Registry demo task");

    println!(
        "[QUERY] after begin: {:?}",
        store_db1
            .query(&username, &job_id)
            .map(|ut| ut.task.description().to_string())
    );

    tracker.log_progress(1);
    tracker.log_progress(2);

    tracker.end_subtask_with_description("Registry demo task");

    println!(
        "[QUERY] after end: {:?}",
        store_db1
            .query(&username, &job_id)
            .map(|ut| ut.task.description().to_string())
    );

    // Per-db isolation: db2 should still have nothing.
    println!(
        "[QUERY] db2 tasks for username={}: {}",
        username,
        store_db2.query_by_username(&username).len()
    );

    // 2) Disabled service: always returns EmptyTaskStore.
    let service_disabled = TaskStoreService::new(false);
    let store_disabled = service_disabled.get_task_store("any_db");

    println!(
        "[SERVICE] disabled is_enabled={}",
        service_disabled.is_progress_tracking_enabled()
    );
    describe_store(&store_disabled, "disabled");

    let disabled_job_id = JobId::new();
    let disabled_registry_factory =
        TaskRegistryFactories::local(username.clone(), store_disabled.clone());

    let disabled_task = Tasks::leaf_with_volume("Disabled tracking task".to_string(), 1)
        .base()
        .clone();

    let mut disabled_tracker = TaskProgressTracker::with_registry(
        disabled_task,
        Concurrency::of(1),
        disabled_job_id.clone(),
        disabled_registry_factory.as_ref(),
    );

    disabled_tracker.begin_subtask_with_description("Disabled tracking task");
    println!(
        "[QUERY] disabled store after begin: {:?}",
        store_disabled
            .query(&username, &disabled_job_id)
            .map(|ut| ut.task.description().to_string())
    );

    disabled_tracker.log_progress(1);
    disabled_tracker.end_subtask_with_description("Disabled tracking task");

    println!(
        "[QUERY] disabled store after end: {:?}",
        store_disabled
            .query(&username, &disabled_job_id)
            .map(|ut| ut.task.description().to_string())
    );
    describe_store(&store_disabled, "disabled");

    // Cleanup global holder state for the enabled service.
    service_enabled.purge_database(db1);
    service_enabled.purge_database(db2);
}
