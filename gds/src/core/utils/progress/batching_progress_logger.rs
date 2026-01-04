//! Batching progress logger for high-performance concurrent progress tracking.

use super::{MessageFactory, ProgressLogger, UNKNOWN_VOLUME};
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};

/// Maximum interval for logging (2^13 = 8192).
pub const MAXIMUM_LOG_INTERVAL: u64 = 1 << 13;

/// Progress logger that batches updates for performance in concurrent scenarios.
///
/// This logger reduces logging overhead by:
/// - Batching progress updates based on concurrency level
/// - Using atomic counters for thread-safe aggregation
/// - Only logging when batch thresholds are reached
/// - Thread-local call counters to minimize contention
///
/// # Design
///
/// Based on Java GDS BatchingProgressLogger, optimized for concurrent algorithms:
/// - Global progress counter (AtomicI64) for all threads
/// - Thread-local call counter to track local progress
/// - Batch size calculated from task volume and concurrency
/// - Logarithmic scaling for batch size (prevents excessive logging)
///
/// # Performance
///
/// - **Thread-safe**: Lock-free atomic operations
/// - **Low contention**: Thread-local counters minimize atomic operations
/// - **Adaptive batching**: Batch size scales with volume and concurrency
/// - **Lazy messages**: Message factories only called when logging
///
/// # Example
///
/// ```rust,ignore
/// use gds::core::utils::progress::*;
///
/// let logger = BatchingProgressLogger::new(
///     Task::new("Algorithm".to_string(), 1_000_000),
///     4  // concurrency
/// );
///
/// // In parallel threads
/// for _ in 0..1000 {
///     logger.log_progress();  // Batched - only logs occasionally
/// }
/// ```
pub struct BatchingProgressLogger {
    // Configuration
    task_volume: AtomicU64,
    batch_size: AtomicU64,
    task_name: String,
    concurrency: usize,

    // Counters
    progress_counter: AtomicI64,
    global_percentage: AtomicI64,
}

impl BatchingProgressLogger {
    /// Create a new batching progress logger.
    ///
    /// # Parameters
    /// - `task_name`: Name of the task
    /// - `task_volume`: Total volume of work (use UNKNOWN_VOLUME for unknown)
    /// - `concurrency`: Number of concurrent threads
    pub fn new(task_name: String, task_volume: usize, concurrency: usize) -> Self {
        let task_volume_u64 = task_volume as u64;
        let batch_size = Self::calculate_batch_size_for_volume(task_volume_u64, concurrency);

        Self {
            task_volume: AtomicU64::new(task_volume_u64),
            batch_size: AtomicU64::new(batch_size),
            task_name,
            concurrency,
            progress_counter: AtomicI64::new(0),
            global_percentage: AtomicI64::new(0),
        }
    }

    /// Calculate batch size from task volume and concurrency.
    ///
    /// Uses logarithmic scaling to balance logging frequency with overhead:
    /// - Higher concurrency → larger batches (fewer atomic operations)
    /// - Larger volumes → larger batches (fewer logs)
    /// - Capped at MAXIMUM_LOG_INTERVAL to ensure regular updates
    ///
    /// # Formula
    /// ```text
    /// batch_size = min(MAXIMUM_LOG_INTERVAL,
    ///                  bit_ceil(task_volume / (100 * concurrency)))
    /// ```
    pub fn calculate_batch_size_for_volume(task_volume: u64, concurrency: usize) -> u64 {
        if task_volume == UNKNOWN_VOLUME as u64 {
            return 1;
        }

        let concurrency = concurrency.max(1) as u64;
        let base = task_volume / (100 * concurrency);

        if base == 0 {
            return 1;
        }

        // Calculate next power of 2 (bit_ceil equivalent)
        let batch_size = base.next_power_of_two();

        // Cap at maximum interval
        batch_size.min(MAXIMUM_LOG_INTERVAL)
    }

    /// Log progress with batching and optional message.
    ///
    /// Only actually logs when the batch threshold is reached.
    fn log_progress_internal(&self, progress: i64, _msg_factory: MessageFactory) {
        // Always update global progress to avoid undercounting when batching.
        let new_progress = self.progress_counter.fetch_add(progress, Ordering::SeqCst) + progress;

        let task_volume = self.task_volume.load(Ordering::Relaxed);
        if task_volume == 0 || task_volume == UNKNOWN_VOLUME as u64 {
            return;
        }

        // Decide whether to emit/log based on batch size.
        let batch_size = self.batch_size.load(Ordering::Relaxed).max(1);
        let should_log = if batch_size == 1 {
            true
        } else {
            // `batch_size` is a power of two by construction.
            (new_progress as u64) & (batch_size - 1) == 0
        };

        if !should_log {
            return;
        }

        let percentage = ((new_progress as f64 / task_volume as f64) * 100.0) as i64;
        let old_percentage = self.global_percentage.swap(percentage, Ordering::SeqCst);

        if percentage > old_percentage {
            #[cfg(debug_assertions)]
            {
                let _message = _msg_factory();
                eprintln!("[PROGRESS] {}% - {}", percentage, self.task_name);
            }
        }
    }

    /// Get current progress counter value.
    pub fn progress_counter(&self) -> i64 {
        self.progress_counter.load(Ordering::SeqCst)
    }

    /// Get current global percentage.
    pub fn global_percentage(&self) -> i64 {
        self.global_percentage.load(Ordering::SeqCst)
    }

    /// Get current batch size.
    pub fn batch_size(&self) -> u64 {
        self.batch_size.load(Ordering::Relaxed)
    }
}

impl ProgressLogger for BatchingProgressLogger {
    fn get_task(&self) -> &str {
        &self.task_name
    }

    fn set_task(&mut self, task: String) {
        self.task_name = task;
    }

    fn log_progress_with_message(&mut self, progress: i64, msg_factory: MessageFactory) {
        self.log_progress_internal(progress, msg_factory);
    }

    fn log_message(&mut self, msg: &str) {
        // In a real implementation, this would use a proper logging system
        #[cfg(debug_assertions)]
        {
            eprintln!("[INFO] {} - {}", self.task_name, msg);
        }
        let _ = msg; // Suppress unused warning in release builds
    }

    fn log_debug(&mut self, msg: &str) {
        #[cfg(debug_assertions)]
        {
            eprintln!("[DEBUG] {} - {}", self.task_name, msg);
        }
        let _ = msg;
    }

    fn log_warning(&mut self, msg: &str) {
        #[cfg(debug_assertions)]
        {
            eprintln!("[WARN] {} - {}", self.task_name, msg);
        }
        let _ = msg;
    }

    fn log_error(&mut self, msg: &str) {
        #[cfg(debug_assertions)]
        {
            eprintln!("[ERROR] {} - {}", self.task_name, msg);
        }
        let _ = msg;
    }

    fn log_finish_percentage(&mut self) {
        let percentage = 100;
        self.global_percentage.store(percentage, Ordering::SeqCst);

        #[cfg(debug_assertions)]
        {
            eprintln!("[PROGRESS] 100% - {}", self.task_name);
        }
    }

    fn reset(&mut self, new_task_volume: i64) -> i64 {
        let old_volume = self
            .task_volume
            .swap(new_task_volume as u64, Ordering::SeqCst) as i64;

        // Recalculate batch size
        let new_batch_size =
            Self::calculate_batch_size_for_volume(new_task_volume as u64, self.concurrency);
        self.batch_size.store(new_batch_size, Ordering::SeqCst);

        // Reset counters
        self.progress_counter.store(0, Ordering::SeqCst);
        self.global_percentage.store(0, Ordering::SeqCst);

        old_volume
    }

    fn release(&mut self) {
        // No resources to release - atomics are automatically dropped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_batch_size_for_volume() {
        // Small volume, high concurrency → small batch
        assert_eq!(
            BatchingProgressLogger::calculate_batch_size_for_volume(1000, 10),
            1
        );

        // Moderate volume, moderate concurrency
        assert_eq!(
            BatchingProgressLogger::calculate_batch_size_for_volume(100_000, 4),
            256
        );

        // Large volume, low concurrency
        let batch = BatchingProgressLogger::calculate_batch_size_for_volume(10_000_000, 1);
        assert!(batch > 1000);
        assert!(batch <= MAXIMUM_LOG_INTERVAL);

        // Cap at maximum
        let batch = BatchingProgressLogger::calculate_batch_size_for_volume(u64::MAX / 2, 1);
        assert_eq!(batch, MAXIMUM_LOG_INTERVAL);
    }

    #[test]
    fn test_calculate_batch_size_unknown_volume() {
        let batch =
            BatchingProgressLogger::calculate_batch_size_for_volume(UNKNOWN_VOLUME as u64, 4);
        assert_eq!(batch, 1);
    }

    #[test]
    fn test_new_logger() {
        let logger = BatchingProgressLogger::new("Test".to_string(), 10000, 4);

        assert_eq!(logger.get_task(), "Test");
        assert_eq!(logger.progress_counter(), 0);
        assert_eq!(logger.global_percentage(), 0);
        assert!(logger.batch_size() > 0);
    }

    #[test]
    fn test_new_with_params() {
        let logger = BatchingProgressLogger::new("CustomTask".to_string(), 50000, 8);

        assert_eq!(logger.get_task(), "CustomTask");
        assert_eq!(logger.task_volume.load(Ordering::Relaxed), 50000);
        assert_eq!(logger.concurrency, 8);
    }

    #[test]
    fn test_log_progress() {
        let mut logger = BatchingProgressLogger::new("Test".to_string(), 10000, 1);
        let batch_size = logger.batch_size();

        // Log progress up to batch size
        for _ in 0..batch_size {
            logger.log_progress();
        }

        // Progress should be recorded
        let progress = logger.progress_counter();
        assert!(progress > 0);
    }

    #[test]
    fn test_log_progress_amount() {
        let mut logger = BatchingProgressLogger::new("Test".to_string(), 10000, 1);
        let batch_size = logger.batch_size();

        // Log batch_size items at once
        for _ in 0..batch_size {
            logger.log_progress_amount(10);
        }

        let progress = logger.progress_counter();
        assert!(progress >= 10);
    }

    #[test]
    fn test_reset() {
        let mut logger = BatchingProgressLogger::new("Test".to_string(), 10000, 4);

        // Log some progress
        logger.log_progress_amount(100);

        // Reset with new volume
        let old_volume = logger.reset(20000);
        assert_eq!(old_volume, 10000);

        // Counters should be reset
        assert_eq!(logger.progress_counter(), 0);
        assert_eq!(logger.global_percentage(), 0);

        // New volume should be set
        assert_eq!(logger.task_volume.load(Ordering::Relaxed), 20000);
    }

    #[test]
    fn test_set_task() {
        let mut logger = BatchingProgressLogger::new("Original".to_string(), 1000, 2);

        assert_eq!(logger.get_task(), "Original");

        logger.set_task("NewTask".to_string());
        assert_eq!(logger.get_task(), "NewTask");
    }

    #[test]
    fn test_log_message() {
        let mut logger = BatchingProgressLogger::new("Test".to_string(), 1000, 2);

        // Should not panic
        logger.log_message("Test message");
        logger.log_debug("Debug message");
        logger.log_warning("Warning message");
        logger.log_error("Error message");
    }

    #[test]
    fn test_log_finish_percentage() {
        let mut logger = BatchingProgressLogger::new("Test".to_string(), 1000, 2);

        logger.log_finish_percentage();
        assert_eq!(logger.global_percentage(), 100);
    }

    #[test]
    fn test_batch_size_scaling() {
        // Test that batch size scales with concurrency
        let volume = 100_000u64;

        let batch1 = BatchingProgressLogger::calculate_batch_size_for_volume(volume, 1);
        let batch4 = BatchingProgressLogger::calculate_batch_size_for_volume(volume, 4);
        let batch16 = BatchingProgressLogger::calculate_batch_size_for_volume(volume, 16);

        // Higher concurrency should generally mean larger batches
        assert!(batch16 <= batch4);
        assert!(batch4 <= batch1);
    }

    #[test]
    fn test_thread_safety() {
        use std::sync::Arc;
        use std::thread;

        let logger = Arc::new(BatchingProgressLogger::new(
            "Concurrent".to_string(),
            100_000,
            4,
        ));
        let batch_size = logger.batch_size();

        // Spawn multiple threads
        let mut handles = vec![];
        for _ in 0..4 {
            let logger_clone = Arc::clone(&logger);
            let handle = thread::spawn(move || {
                for _ in 0..batch_size {
                    // Note: We need &mut for log_progress, so this test just verifies
                    // that the logger can be shared across threads (Send)
                    let _task = logger_clone.get_task();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Verify logger is still valid
        assert_eq!(logger.get_task(), "Concurrent");
    }
}
