// Phase 2.5: LinkFeatureStepConfiguration - Shared configuration interface for link feature steps

use serde_json::Value;
use std::collections::HashMap;

/// Configuration interface for link feature steps.
///
/// All link feature steps share a list of node properties.
pub trait LinkFeatureStepConfiguration {
    /// Returns the list of node properties this feature step operates on.
    ///
    /// All link feature steps require at least one node property.
    fn node_properties(&self) -> &[String];

    /// Converts a raw object (typically from JSON/config) to validated property list.
    ///
    /// Validation rules:
    /// - Must be a list/array
    /// - Must be non-empty
    /// - All elements must be non-blank strings
    fn from_object(node_properties: &Value) -> Result<Vec<String>, String> {
        // Validate it's an array
        let properties_array = node_properties.as_array().ok_or_else(|| {
            format!(
                "The value of `nodeProperties` must be of type `List` but was `{}`.",
                match node_properties {
                    Value::Null => "Null",
                    Value::Bool(_) => "Boolean",
                    Value::Number(_) => "Number",
                    Value::String(_) => "String",
                    Value::Array(_) => "Array", // shouldn't reach
                    Value::Object(_) => "Object",
                }
            )
        })?;

        // Validate non-empty
        if properties_array.is_empty() {
            return Err("`nodeProperties` must be non-empty.".to_string());
        }

        // Validate all elements are non-blank strings
        let mut validated_properties = Vec::new();
        let mut invalid_properties = Vec::new();

        for property in properties_array {
            if let Some(s) = property.as_str() {
                if s.trim().is_empty() {
                    invalid_properties.push(format!("\"{}\"", s));
                } else {
                    validated_properties.push(s.to_string());
                }
            } else {
                invalid_properties.push(format!("{}", property));
            }
        }

        if !invalid_properties.is_empty() {
            return Err(format!(
                "Invalid property names defined in `nodeProperties`: [{}]. Expecting a String with at least one non-white space character.",
                invalid_properties.join(", ")
            ));
        }

        Ok(validated_properties)
    }

    /// Returns the configuration as a map.
    ///
    /// Default implementation returns nodeProperties under "nodeProperties" key.
    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "nodeProperties".to_string(),
            Value::Array(
                self.node_properties()
                    .iter()
                    .map(|s| Value::String(s.clone()))
                    .collect(),
            ),
        );
        map
    }

    /// Returns the config keys that should be collected.
    ///
    /// Default implementation returns empty (used by Java's @CollectKeys annotation).
    fn config_keys(&self) -> Vec<String> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Test struct implementing the trait
    struct TestConfig {
        properties: Vec<String>,
    }

    impl LinkFeatureStepConfiguration for TestConfig {
        fn node_properties(&self) -> &[String] {
            &self.properties
        }
    }

    #[test]
    fn test_from_object_valid_list() {
        let input = json!(["prop1", "prop2", "prop3"]);
        let result = TestConfig::from_object(&input).unwrap();
        assert_eq!(result, vec!["prop1", "prop2", "prop3"]);
    }

    #[test]
    fn test_from_object_empty_list() {
        let input = json!([]);
        let result = TestConfig::from_object(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be non-empty"));
    }

    #[test]
    fn test_from_object_not_array() {
        let input = json!("not an array");
        let result = TestConfig::from_object(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be of type `List`"));
    }

    #[test]
    fn test_from_object_invalid_element_types() {
        let input = json!(["valid", 123, "another_valid"]);
        let result = TestConfig::from_object(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid property names"));
    }

    #[test]
    fn test_from_object_blank_strings() {
        let input = json!(["valid", "  ", "another_valid"]);
        let result = TestConfig::from_object(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid property names"));
    }

    #[test]
    fn test_to_map() {
        let config = TestConfig {
            properties: vec!["prop1".to_string(), "prop2".to_string()],
        };
        let map = config.to_map();
        assert!(map.contains_key("nodeProperties"));
        assert_eq!(
            map.get("nodeProperties").unwrap().as_array().unwrap().len(),
            2
        );
    }

    #[test]
    fn test_config_keys_default() {
        let config = TestConfig {
            properties: vec!["prop".to_string()],
        };
        assert_eq!(config.config_keys().len(), 0);
    }

    #[test]
    fn test_validation_round_trip() {
        let config = TestConfig {
            properties: vec!["embedding".to_string()],
        };

        assert_eq!(config.node_properties().len(), 1);

        let valid = TestConfig::from_object(&json!(["prop"]));
        assert!(valid.is_ok());
    }
}
