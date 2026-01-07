/// Service for validating memory usage of operations.
///
/// Mirrors Java MemoryUsageValidator class.
/// Contains memory validation logic for graph operations.
use crate::applications::services::logging::Log;
use crate::core::utils::progress::JobId;
use crate::mem::{
    MemoryRange, MemoryReservationExceededException, MemoryTracker, MemoryTreeWithDimensions,
};

pub struct MemoryUsageValidator {
    log: Log,
    use_max_memory_estimation: bool,
    memory_tracker: MemoryTracker,
    #[allow(dead_code)]
    username: String,
}

impl MemoryUsageValidator {
    /// Creates a new MemoryUsageValidator.
    pub fn new(
        username: String,
        memory_tracker: MemoryTracker,
        use_max_memory_estimation: bool,
        log: Log,
    ) -> Self {
        Self {
            log,
            use_max_memory_estimation,
            memory_tracker,
            username,
        }
    }

    /// Tries to validate memory usage for a configuration.
    /// In Java, this returns MemoryRange and handles estimation.
    pub fn try_validate_memory_usage<C: JobIdConfig>(
        &self,
        task_name: &str,
        config: &C,
        run_estimation: impl FnOnce(&C) -> MemoryTreeWithDimensions,
    ) -> Result<MemoryRange, MemoryReservationExceededException> {
        let memory_tree_with_dimensions = run_estimation(config);
        let estimated_memory_range = self.compute_memory_range(&memory_tree_with_dimensions);

        let available_bytes = self.memory_tracker.available_memory();
        let job_id = config.job_id();
        self.validate_memory_usage(
            task_name,
            &estimated_memory_range,
            available_bytes,
            self.use_max_memory_estimation,
            job_id,
            &self.log,
        )?;

        Ok(estimated_memory_range)
    }

    /// Validates memory usage against available memory.
    /// In Java, this throws exceptions if memory is insufficient.
    pub fn validate_memory_usage(
        &self,
        task_name: &str,
        estimated_memory_range: &MemoryRange,
        available_bytes: u64,
        use_max_memory_estimation: bool,
        job_id: &JobId,
        log: &Log,
    ) -> Result<(), MemoryReservationExceededException> {
        let required_bytes: u64 = if use_max_memory_estimation {
            estimated_memory_range.max() as u64
        } else {
            estimated_memory_range.min() as u64
        };

        if required_bytes > available_bytes {
            let err = MemoryReservationExceededException::new(required_bytes, available_bytes);
            let error_message = format!(
                "Insufficient memory for {} (jobId={}): required {} but only {} available",
                task_name, job_id, required_bytes, available_bytes
            );
            log.error(&error_message);
            return Err(err);
        }

        Ok(())
    }

    /// Computes memory range from memory tree with dimensions.
    fn compute_memory_range(
        &self,
        memory_tree_with_dimensions: &MemoryTreeWithDimensions,
    ) -> MemoryRange {
        *memory_tree_with_dimensions.memory_tree().memory_usage()
    }
}

/// Trait for configurations that have a job ID.
pub trait JobIdConfig {
    fn job_id(&self) -> &JobId;
}
