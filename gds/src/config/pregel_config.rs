//! Pregel execution configuration
//!
//! Macro-generated demo of PregelConfig using the in-repo `define_config!` macro.

// The demo macro is defined in `projection::codegen::config::define_config.rs` and exported
// as `define_config!`. This file invokes that macro at crate level to produce
// a struct, builder, Default, and validation bridge.

use crate::config::base_types::AlgoBaseConfig;
use crate::config::validation::ConfigValidation;
use crate::config::{ConcurrencyConfig, IterationsConfig};
use crate::core::utils::partition::Partitioning;
use crate::define_config;

define_config!(
    pub struct PregelConfig {
        validate = |cfg: &PregelConfig| {
            ConfigValidation::validate_positive(cfg.base.concurrency as f64, "concurrency")?;
            ConfigValidation::validate_positive(cfg.max_iterations as f64, "maxIterations")?;
            if let Some(tol) = cfg.tolerance {
                ConfigValidation::validate_positive(tol, "tolerance")?;
            }
            Ok(())
        },
        base: AlgoBaseConfig = AlgoBaseConfig::default(),
        max_iterations: usize = 20,
        tolerance: Option<f64> = None,
        is_asynchronous: bool = false,
        partitioning: Partitioning = Partitioning::Range,
        track_sender: bool = false,
    }
);

impl IterationsConfig for PregelConfig {
    fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn tolerance(&self) -> Option<f64> {
        self.tolerance
    }
}

impl ConcurrencyConfig for PregelConfig {
    fn concurrency(&self) -> usize {
        self.base.concurrency
    }
}

/// Common interface for Pregel runtime configuration.
pub trait PregelRuntimeConfig: IterationsConfig + ConcurrencyConfig + Clone + Send + Sync {
    fn is_asynchronous(&self) -> bool;
    fn partitioning(&self) -> Partitioning;
    fn track_sender(&self) -> bool;

    fn use_fork_join(&self) -> bool {
        matches!(self.partitioning(), Partitioning::Auto)
    }
}

impl PregelRuntimeConfig for PregelConfig {
    fn is_asynchronous(&self) -> bool {
        self.is_asynchronous
    }

    fn partitioning(&self) -> Partitioning {
        self.partitioning
    }

    fn track_sender(&self) -> bool {
        self.track_sender
    }
}
