//! Request-scoped dependencies (Java parity).
//!
//! Mirrors `RequestScopedDependencies` in Java:
//! - termination flag (cancellation)
//! - progress task registry factory + job id (for trackers)

use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{JobId, TaskRegistryFactory};
use std::sync::Arc;

#[derive(Clone)]
pub struct RequestScopedDependencies {
    pub job_id: JobId,
    pub task_registry_factory: Arc<dyn TaskRegistryFactory>,
    pub termination_flag: TerminationFlag,
}

impl RequestScopedDependencies {
    pub fn new(
        job_id: JobId,
        task_registry_factory: Arc<dyn TaskRegistryFactory>,
        termination_flag: TerminationFlag,
    ) -> Self {
        Self {
            job_id,
            task_registry_factory,
            termination_flag,
        }
    }
}
