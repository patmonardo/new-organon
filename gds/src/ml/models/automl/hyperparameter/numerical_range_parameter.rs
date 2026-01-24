use serde::Serialize;
use std::collections::HashMap;

/// Trait for numerical range parameters
///
/// Java: `interface NumericalRangeParameter<T extends Number> { T min(); T max(); }`
pub trait NumericalRangeParameter<T>: Send + Sync
where
    T: Clone + Send + Sync + Serialize,
{
    /// Get the minimum value
    fn min(&self) -> T;

    /// Get the maximum value
    fn max(&self) -> T;

    /// Convert to map representation
    ///
    /// Java: `default Map<String, Object> toMap() { return Map.of("range", List.of(min(), max())); }`
    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        let range = vec![
            serde_json::to_value(self.min()).unwrap(),
            serde_json::to_value(self.max()).unwrap(),
        ];
        map.insert("range".to_string(), serde_json::Value::Array(range));
        map
    }
}
