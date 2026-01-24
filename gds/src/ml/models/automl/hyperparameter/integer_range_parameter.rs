use super::numerical_range_parameter::NumericalRangeParameter;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

/// Integer range parameter
///
/// Java: `interface IntegerRangeParameter extends NumericalRangeParameter<Integer>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntegerRangeParameter {
    min: i32,
    max: i32,
}

impl IntegerRangeParameter {
    /// Create a new IntegerRangeParameter
    ///
    /// Java: `static IntegerRangeParameter of(int min, int max)`
    pub fn of(min: i32, max: i32) -> Self {
        Self { min, max }
    }
}

impl NumericalRangeParameter<i32> for IntegerRangeParameter {
    fn min(&self) -> i32 {
        self.min
    }

    fn max(&self) -> i32 {
        self.max
    }

    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        let range = vec![
            serde_json::Value::Number(serde_json::Number::from(self.min as i64)),
            serde_json::Value::Number(serde_json::Number::from(self.max as i64)),
        ];
        map.insert("range".to_string(), serde_json::Value::Array(range));
        map
    }
}
