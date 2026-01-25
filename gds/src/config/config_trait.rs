//! Config trait abstraction for gds
//!
//! A minimal, object-safe trait that all runtime configuration types can implement
//! to provide uniform validation behaviour.

use crate::config::validation::ConfigError;

/// A Config is a typed runtime configuration with validation semantics.
///
/// Implementors must provide a `validate` method which returns `Ok(())` when the
/// configuration is valid or a `ConfigError` describing the failure.
pub trait ValidatedConfig: Send + Sync {
    /// Validate this configuration.
    fn validate(&self) -> Result<(), ConfigError>;

    /// Optional human-friendly name for the config type (defaults to Rust type name).
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}
