use super::numerical_range_parameter::NumericalRangeParameter;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

/// Double range parameter with optional log scale
///
/// Java: `interface DoubleRangeParameter extends NumericalRangeParameter<Double>`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoubleRangeParameter {
    min: f64,
    max: f64,
    log_scale: bool,
}

impl DoubleRangeParameter {
    /// Create a new DoubleRangeParameter without log scale
    ///
    /// Java: `static DoubleRangeParameter of(double min, double max)`
    pub fn of(min: f64, max: f64) -> Self {
        Self::of_with_log_scale(min, max, false)
    }

    /// Create a new DoubleRangeParameter with optional log scale
    ///
    /// Java: `static DoubleRangeParameter of(double min, double max, boolean logScale)`
    pub fn of_with_log_scale(min: f64, max: f64, log_scale: bool) -> Self {
        Self {
            min,
            max,
            log_scale,
        }
    }

    /// Get log scale flag
    ///
    /// Java: `boolean logScale()`
    pub fn log_scale(&self) -> bool {
        self.log_scale
    }
}

impl NumericalRangeParameter<f64> for DoubleRangeParameter {
    fn min(&self) -> f64 {
        self.min
    }

    fn max(&self) -> f64 {
        self.max
    }

    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        let range = vec![
            serde_json::Value::Number(serde_json::Number::from_f64(self.min).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(self.max).unwrap()),
        ];
        map.insert("range".to_string(), serde_json::Value::Array(range));
        map
    }
}
