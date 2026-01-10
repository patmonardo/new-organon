/// Utility for parsing write-property inputs (string | map | list) from user configs.
/// Mirrors Java `UserInputWriteProperties`.
pub struct UserInputWriteProperties;

impl UserInputWriteProperties {
    pub fn new() -> Self {
        Self
    }

    /// Parse a serde_json value into property specs.
    /// Accepts:
    /// - String => property name
    /// - Object => {"prop": "writeProp"} renames
    /// - Array  => mix of the above
    pub fn parse(
        value: &serde_json::Value,
        configuration_key: &str,
    ) -> Result<Vec<PropertySpec>, String> {
        let mut specs = Vec::new();
        match value {
            serde_json::Value::String(s) => specs.push(PropertySpec::new(s.to_string(), None)),
            serde_json::Value::Object(map) => {
                for (k, v) in map.iter() {
                    let Some(write_prop) = v.as_str() else {
                        return Err(Self::type_mismatch(configuration_key, value));
                    };
                    specs.push(PropertySpec::new(k.clone(), Some(write_prop.to_string())));
                }
            }
            serde_json::Value::Array(items) => {
                for item in items {
                    let parsed = Self::parse(item, configuration_key)?;
                    specs.extend(parsed);
                }
            }
            _ => return Err(Self::type_mismatch(configuration_key, value)),
        }

        Ok(specs)
    }

    fn type_mismatch(configuration_key: &str, value: &serde_json::Value) -> String {
        let typename = match value {
            serde_json::Value::Null => "null",
            serde_json::Value::Bool(_) => "boolean",
            serde_json::Value::Number(_) => "number",
            serde_json::Value::String(_) => "string",
            serde_json::Value::Array(_) => "array",
            serde_json::Value::Object(_) => "object",
        };
        format!(
            "Type mismatch for {configuration_key}: expected string, object, or array of those; found {typename}"
        )
    }
}

impl Default for UserInputWriteProperties {
    fn default() -> Self {
        Self::new()
    }
}

/// Specification for a property to be written.
#[derive(Clone, Debug)]
pub struct PropertySpec {
    node_property_name: String,
    renamed_node_property: Option<String>,
}

impl PropertySpec {
    pub fn new(node_property_name: String, renamed_node_property: Option<String>) -> Self {
        Self {
            node_property_name,
            renamed_node_property,
        }
    }

    pub fn write_property(&self) -> String {
        self.renamed_node_property
            .clone()
            .unwrap_or_else(|| self.node_property_name.clone())
    }

    pub fn node_property(&self) -> String {
        self.node_property_name.clone()
    }

    pub fn renamed_property(&self) -> Option<String> {
        self.renamed_node_property.clone()
    }
}
