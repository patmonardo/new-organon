//! User log entry for tracking warnings and messages.

use crate::core::utils::progress::Task;
use std::time::{SystemTime, UNIX_EPOCH};

/// A single log entry containing task information and a message.
///
/// Captures the task name, message, and timestamp when the task started.
#[derive(Debug, Clone)]
pub struct UserLogEntry {
    task_name: String,
    message: String,
    time_started_millis: u64,
}

impl UserLogEntry {
    /// Creates a new user log entry from a task and message.
    pub fn new(task: &Task, message: String) -> Self {
        Self {
            task_name: task.description().to_string(),
            message,
            time_started_millis: task.start_time(),
        }
    }

    /// Returns the task name.
    pub fn task_name(&self) -> &str {
        &self.task_name
    }

    /// Returns the log message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the start time as milliseconds since Unix epoch.
    pub fn time_started_millis(&self) -> i64 {
        self.time_started_millis as i64
    }

    /// Returns the start time as a SystemTime.
    pub fn time_started(&self) -> SystemTime {
        UNIX_EPOCH + std::time::Duration::from_millis(self.time_started_millis)
    }

    /// Returns a formatted time string (HH:MM:SS format).
    pub fn time_started_string(&self) -> String {
        // Use UTC to avoid relying on host-local timezone configuration.
        let datetime = chrono::DateTime::<chrono::Utc>::from(self.time_started());
        datetime.format("%H:%M:%S").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::clock_service::{Clock, ClockService};
    use std::sync::atomic::{AtomicU64, Ordering};

    struct MockClock {
        current_time: AtomicU64,
    }

    impl MockClock {
        const fn new(initial_time: u64) -> Self {
            Self {
                current_time: AtomicU64::new(initial_time),
            }
        }
    }

    impl Clock for MockClock {
        fn millis(&self) -> u64 {
            self.current_time.load(Ordering::SeqCst)
        }
    }

    #[test]
    fn test_user_log_entry_creation() {
        static MOCK: MockClock = MockClock::new(1);

        ClockService::run_with_clock(&MOCK, |_clock| {
            let task = Task::new("Test Task".to_string(), vec![]);
            task.start();
            let entry = UserLogEntry::new(&task, "Test message".to_string());

            assert_eq!(entry.task_name(), "Test Task");
            assert_eq!(entry.message(), "Test message");
            assert!(entry.time_started_millis() > 0);
        });
    }

    #[test]
    fn test_time_formatting() {
        static MOCK: MockClock = MockClock::new(1);

        ClockService::run_with_clock(&MOCK, |_clock| {
            let task = Task::new("Test Task".to_string(), vec![]);
            task.start();
            let entry = UserLogEntry::new(&task, "Test message".to_string());

            let time_string = entry.time_started_string();
            // Should be in HH:MM:SS format
            assert!(time_string.len() >= 8);
            assert!(time_string.contains(':'));
        });
    }
}
