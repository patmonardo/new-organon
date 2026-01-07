//! MemoryGuard / DefaultMemoryGuard (Java parity, simplified).
//!
//! Java references:
//! - `MemoryGuard`
//! - `DefaultMemoryGuard`
//!
//! This layer is just memory guarding. Do not conflate with UI concerns.
//!
//! Differences from Java (intentional simplifications):
//! - Java's `GraphDimensionFactory` and `MemoryRequirement` are collapsed into an
//!   `estimation_factory` closure.
//! - Java throws `IllegalStateException`; Rust returns a structured error.

use std::fmt;
use std::sync::Arc;

use crate::applications::services::logging::Log;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::JobId;
use crate::errors::MemoryEstimationError;
use crate::mem::{MemoryRange, MemoryTracker, MemoryTreeWithDimensions};

use super::Label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryGuardError {
    InsufficientMemory {
        label: String,
        required_bytes: u64,
        available_bytes: u64,
    },
}

impl fmt::Display for MemoryGuardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientMemory {
                label,
                required_bytes,
                available_bytes,
            } => write!(
                f,
                "Memory required to run {} ({}b) exceeds available memory ({}b)",
                label, required_bytes, available_bytes
            ),
        }
    }
}

/// Java-parity hook: transform dimensions before selecting min/max.
///
/// This is kept deliberately small; algorithms can plug in richer logic later.
pub trait DimensionTransformer {
    fn transform(&self, estimate: MemoryTreeWithDimensions) -> MemoryTreeWithDimensions;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IdentityDimensionTransformer;

impl DimensionTransformer for IdentityDimensionTransformer {
    fn transform(&self, estimate: MemoryTreeWithDimensions) -> MemoryTreeWithDimensions {
        estimate
    }
}

/// Minimal config contract needed by the memory guard.
pub trait AlgoBaseConfigLike {
    fn job_id(&self) -> &JobId;
    fn sudo(&self) -> bool {
        false
    }
}

/// This is just memory guarding. Do not conflate with UI concerns.
pub trait MemoryGuard {
    fn assert_algorithm_can_run<CONFIGURATION>(
        &self,
        username: &str,
        estimation_factory: impl FnOnce(
            &GraphResources,
            &CONFIGURATION,
        )
            -> Result<MemoryTreeWithDimensions, MemoryEstimationError>,
        graph_resources: &GraphResources,
        configuration: &CONFIGURATION,
        label: &dyn Label,
        dimension_transformer: &dyn DimensionTransformer,
    ) -> Result<(), MemoryGuardError>
    where
        CONFIGURATION: AlgoBaseConfigLike;
}

/// Java-parity: `MemoryGuard.DISABLED`.
#[derive(Debug, Clone, Copy, Default)]
pub struct DisabledMemoryGuard;

pub const DISABLED: DisabledMemoryGuard = DisabledMemoryGuard;

impl MemoryGuard for DisabledMemoryGuard {
    fn assert_algorithm_can_run<CONFIGURATION>(
        &self,
        _username: &str,
        _estimation_factory: impl FnOnce(
            &GraphResources,
            &CONFIGURATION,
        )
            -> Result<MemoryTreeWithDimensions, MemoryEstimationError>,
        _graph_resources: &GraphResources,
        _configuration: &CONFIGURATION,
        _label: &dyn Label,
        _dimension_transformer: &dyn DimensionTransformer,
    ) -> Result<(), MemoryGuardError>
    where
        CONFIGURATION: AlgoBaseConfigLike,
    {
        Ok(())
    }
}

pub struct DefaultMemoryGuard {
    log: Log,
    use_max_memory_estimation: bool,
    memory_tracker: Arc<MemoryTracker>,
}

impl DefaultMemoryGuard {
    pub fn create(
        log: Log,
        use_max_memory_estimation: bool,
        memory_tracker: Arc<MemoryTracker>,
    ) -> Self {
        Self {
            log,
            use_max_memory_estimation,
            memory_tracker,
        }
    }

    fn required_bytes(&self, estimated_memory_range: &MemoryRange) -> u64 {
        if self.use_max_memory_estimation {
            estimated_memory_range.max() as u64
        } else {
            estimated_memory_range.min() as u64
        }
    }
}

impl MemoryGuard for DefaultMemoryGuard {
    fn assert_algorithm_can_run<CONFIGURATION>(
        &self,
        username: &str,
        estimation_factory: impl FnOnce(
            &GraphResources,
            &CONFIGURATION,
        )
            -> Result<MemoryTreeWithDimensions, MemoryEstimationError>,
        graph_resources: &GraphResources,
        configuration: &CONFIGURATION,
        label: &dyn Label,
        dimension_transformer: &dyn DimensionTransformer,
    ) -> Result<(), MemoryGuardError>
    where
        CONFIGURATION: AlgoBaseConfigLike,
    {
        let estimate = match estimation_factory(graph_resources, configuration) {
            Ok(est) => est,
            Err(MemoryEstimationError::NotImplemented) => {
                self.log.info(&format!(
                    "Memory usage estimate not available for {}, skipping guard",
                    label.as_string()
                ));
                return Ok(());
            }
        };

        let estimate = dimension_transformer.transform(estimate);
        let estimated_memory_range = *estimate.memory_tree().memory_usage();
        let bytes_to_reserve = self.required_bytes(&estimated_memory_range);

        if configuration.sudo() {
            // Java parity: sudo bypasses guard. We best-effort reserve; if it fails, we still allow.
            if let Err(_e) = self.memory_tracker.try_to_track(
                username,
                label.as_string(),
                configuration.job_id(),
                bytes_to_reserve,
            ) {
                self.log.warn(&format!(
                    "Sudo enabled: not enough memory to reserve for {} ({}b), allowing anyway",
                    label.as_string(),
                    bytes_to_reserve
                ));
            }
            return Ok(());
        }

        match self.memory_tracker.try_to_track(
            username,
            label.as_string(),
            configuration.job_id(),
            bytes_to_reserve,
        ) {
            Ok(()) => Ok(()),
            Err(e) => Err(MemoryGuardError::InsufficientMemory {
                label: label.as_string().to_string(),
                required_bytes: e.bytes_required(),
                available_bytes: e.bytes_available(),
            }),
        }
    }
}
