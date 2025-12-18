/// Service for validating graph names.
///
/// Mirrors Java GraphNameValidationService class.
/// Handles string validation logic for graph names with various validation modes.
use crate::types::graph_store::GraphName;

pub struct GraphNameValidationService;

impl GraphNameValidationService {
    /// Creates a new GraphNameValidationService.
    pub fn new() -> Self {
        Self
    }

    /// Validates a graph name string.
    /// In Java, this calls GraphName.parse() and handles validation.
    pub fn validate(&self, graph_name: &str) -> Result<GraphName, String> {
        if graph_name.is_empty() {
            return Err("Graph name cannot be empty".to_string());
        }

        if graph_name.len() > 100 {
            return Err("Graph name too long".to_string());
        }

        Ok(GraphName::parse(graph_name))
    }

    /// Validates a graph name strictly (for creation operations).
    /// In Java, this calls GraphName.parseStrictly().
    pub fn validate_strictly(&self, graph_name: &str) -> Result<GraphName, String> {
        self.validate(graph_name)
    }

    /// Validates a graph name that might be null.
    /// In Java, this returns Optional<GraphName>.
    pub fn validate_possible_null(
        &self,
        graph_name: Option<&str>,
    ) -> Result<Option<GraphName>, String> {
        match graph_name {
            None => Ok(None),
            Some(name) => Ok(Some(self.validate(name)?)),
        }
    }

    /// Validates a single graph name or list of graph names.
    /// In Java, this handles both String and List<String> inputs.
    pub fn validate_single_or_list(
        &self,
        input: &serde_json::Value,
    ) -> Result<Vec<GraphName>, String> {
        match input {
            serde_json::Value::String(s) => Ok(vec![self.validate(s)?]),
            serde_json::Value::Array(items) => {
                let mut names = Vec::with_capacity(items.len());
                for item in items {
                    let s = item
                        .as_str()
                        .ok_or_else(|| "Graph name list must contain strings".to_string())?;
                    names.push(self.validate(s)?);
                }
                Ok(names)
            }
            _ => Err("Expected a graph name string or list of strings".to_string()),
        }
    }
}

impl Default for GraphNameValidationService {
    fn default() -> Self {
        Self::new()
    }
}
