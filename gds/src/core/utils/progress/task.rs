//! Task abstraction for progress tracking.
//!
//! This is a placeholder - the full Task hierarchy will be implemented
//! in the tasks/ submodule.

/// Marker for unknown task volume.
pub const UNKNOWN_VOLUME: usize = usize::MAX;

/// Represents a task that can be tracked for progress.
///
/// This is a simplified placeholder. The full Task trait hierarchy
/// will be implemented in the tasks/ module.
#[derive(Debug, Clone)]
pub struct Task {
    pub description: String,
    pub volume: usize,
    start_time_millis: i64,
}

impl Task {
    /// Unknown volume constant.
    pub const UNKNOWN_VOLUME: usize = UNKNOWN_VOLUME;

    /// Create a new task with known volume.
    pub fn new(description: String, volume: usize) -> Self {
        Self {
            description,
            volume,
            start_time_millis: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        }
    }

    /// Create a task with unknown volume.
    pub fn with_unknown_volume(description: String) -> Self {
        Self {
            description,
            volume: UNKNOWN_VOLUME,
            start_time_millis: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        }
    }

    /// Get task description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get task volume.
    pub fn volume(&self) -> usize {
        self.volume
    }

    /// Check if volume is known.
    pub fn has_known_volume(&self) -> bool {
        self.volume != UNKNOWN_VOLUME
    }

    /// Get the task start time as milliseconds since Unix epoch.
    pub fn start_time(&self) -> i64 {
        self.start_time_millis
    }
}
