//! User task record combining user, job, and task information.

use super::{JobId, Task};

/// A task associated with a specific user and job.
///
/// This is a simple record type that bundles together the username,
/// job identifier, and the task itself for storage and querying.
#[derive(Debug, Clone)]
pub struct UserTask {
    pub username: String,
    pub job_id: JobId,
    pub task: Task,
}

impl UserTask {
    /// Create a new UserTask.
    pub fn new(username: String, job_id: JobId, task: Task) -> Self {
        Self {
            username,
            job_id,
            task,
        }
    }

    /// Get the username.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get the job ID.
    pub fn job_id(&self) -> &JobId {
        &self.job_id
    }

    /// Get the task.
    pub fn task(&self) -> &Task {
        &self.task
    }
}
