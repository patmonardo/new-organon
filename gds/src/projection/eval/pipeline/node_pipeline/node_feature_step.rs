use crate::projection::eval::pipeline::FeatureStep;
use serde_json::Value;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// A concrete implementation of FeatureStep for node properties.
///
/// This represents a simple feature extraction step that reads a single
/// node property and includes it in the feature vector.
#[derive(Debug, Clone)]
pub struct NodeFeatureStep {
    node_property: String,
    // Cache for trait methods that need references
    input_properties: Vec<String>,
    configuration: HashMap<String, Value>,
}

impl PartialEq for NodeFeatureStep {
    fn eq(&self, other: &Self) -> bool {
        self.node_property == other.node_property
    }
}

impl Eq for NodeFeatureStep {}

impl Hash for NodeFeatureStep {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node_property.hash(state);
    }
}
impl NodeFeatureStep {
    /// Factory method to create a new NodeFeatureStep.
    pub fn of(node_property: impl Into<String>) -> Self {
        let prop = node_property.into();
        let mut configuration = HashMap::new();
        configuration.insert("nodeProperty".to_string(), Value::String(prop.clone()));

        Self {
            input_properties: vec![prop.clone()],
            node_property: prop,
            configuration,
        }
    }

    /// Constructor for NodeFeatureStep.
    pub fn new(node_property: impl Into<String>) -> Self {
        Self::of(node_property)
    }

    /// Returns the node property name.
    pub fn node_property(&self) -> &str {
        &self.node_property
    }
}

impl FeatureStep for NodeFeatureStep {
    fn input_node_properties(&self) -> &[String] {
        &self.input_properties
    }

    fn name(&self) -> &str {
        "feature"
    }

    fn configuration(&self) -> &HashMap<String, Value> {
        &self.configuration
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            self.name().to_string(),
            Value::String(self.node_property.clone()),
        );
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_feature_step_creation() {
        let step = NodeFeatureStep::of("age");
        assert_eq!(step.node_property(), "age");
        assert_eq!(step.name(), "feature");
    }

    #[test]
    fn test_input_node_properties() {
        let step = NodeFeatureStep::of("age");
        let props = step.input_node_properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0], "age");
    }

    #[test]
    fn test_configuration() {
        let step = NodeFeatureStep::of("age");
        let config = step.configuration();
        assert_eq!(
            config.get("nodeProperty"),
            Some(&Value::String("age".to_string()))
        );
    }

    #[test]
    fn test_to_map() {
        let step = NodeFeatureStep::of("age");
        let map = step.to_map();
        assert_eq!(map.get("feature"), Some(&Value::String("age".to_string())));
    }

    #[test]
    fn test_equality() {
        let step1 = NodeFeatureStep::of("age");
        let step2 = NodeFeatureStep::of("age");
        let step3 = NodeFeatureStep::of("income");

        assert_eq!(step1, step2);
        assert_ne!(step1, step3);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let step1 = NodeFeatureStep::of("age");
        let step2 = NodeFeatureStep::of("age");
        let step3 = NodeFeatureStep::of("income");

        let mut set = HashSet::new();
        set.insert(step1);
        set.insert(step2); // Should not increase set size (same as step1)
        set.insert(step3);

        assert_eq!(set.len(), 2);
    }
}
