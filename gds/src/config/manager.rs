use crate::config::defaults::DefaultsManager;
use crate::config::limits::{LimitViolation, LimitsManager};
use crate::config::ValidatedConfig;
use serde::de::DeserializeOwned;
use serde_json::Value;

/// Result from running the manager pipeline: either the typed config or a list of errors.
pub enum PipelineError {
    ValidationErrors(Vec<String>),
    LimitViolations(Vec<LimitViolation>),
}

/// Unified config pipeline for defaults -> deserialize -> validate -> limits.
pub trait ConfigPipeline: Sized + DeserializeOwned + ValidatedConfig + 'static {
    fn from_value_with_manager(
        input: &Value,
        username: Option<&str>,
        defaults: &DefaultsManager,
        limits: &LimitsManager,
    ) -> Result<Self, PipelineError> {
        apply_defaults_validate_and_check_limits::<Self>(input, username, defaults, limits)
    }
}

impl<T> ConfigPipeline for T where T: DeserializeOwned + ValidatedConfig + 'static {}

/// Apply defaults, deserialize, validate using `ValidatedConfig`, then check limits.
pub fn apply_defaults_validate_and_check_limits<T: DeserializeOwned + ValidatedConfig + 'static>(
    input: &Value,
    username: Option<&str>,
    defaults: &DefaultsManager,
    limits: &LimitsManager,
) -> Result<T, PipelineError> {
    // Must be an object
    let mut map = match input.as_object() {
        Some(m) => m.clone(),
        None => {
            return Err(PipelineError::ValidationErrors(vec![
                "input must be an object".to_string(),
            ]))
        }
    };

    // Apply defaults
    defaults.apply(&mut map, username);

    // Try deserialize
    let merged_value = Value::Object(map.clone());
    let deserialized: T = match serde_json::from_value(merged_value.clone()) {
        Ok(v) => v,
        Err(e) => {
            return Err(PipelineError::ValidationErrors(vec![format!(
                "deserialize: {}",
                e
            )]))
        }
    };

    // Validate via trait
    if let Err(e) = deserialized.validate() {
        return Err(PipelineError::ValidationErrors(vec![e.to_string()]));
    }

    // Limit validation
    let limit_violations = limits.validate(&map, username);
    if !limit_violations.is_empty() {
        return Err(PipelineError::LimitViolations(limit_violations));
    }

    Ok(deserialized)
}
