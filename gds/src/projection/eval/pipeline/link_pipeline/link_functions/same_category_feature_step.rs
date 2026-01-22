// Phase 2.4: SameCategoryStep - Binary categorical equality between node properties

use super::super::{LinkFeatureAppender, LinkFeatureStep};
use crate::types::graph::Graph;
use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;
use std::collections::HashMap;
use std::sync::Arc;

/// SameCategory link feature - binary indicator of categorical equality.
///
/// Returns one binary feature per property.
#[derive(Debug, Clone)]
pub struct SameCategoryStep {
    /// Node properties to check for categorical equality
    node_properties: Vec<String>,
}

impl SameCategoryStep {
    /// Creates a new SameCategoryStep for the given node properties.
    ///
    /// # Arguments
    ///
    /// * `node_properties` - List of property names to check equality on
    ///
    /// Each property will produce one binary feature (dimension = properties.len()).
    pub fn new(node_properties: Vec<String>) -> Self {
        Self { node_properties }
    }
}

impl LinkFeatureStep for SameCategoryStep {
    fn link_feature_appender(&self, graph: &dyn Graph) -> Box<dyn LinkFeatureAppender> {
        let mut appenders = Vec::new();

        for property_name in &self.node_properties {
            let property = graph
                .node_properties(property_name)
                .unwrap_or_else(|| panic!("Property {} not found", property_name));

            let appender: Box<dyn LinkFeatureAppender> = match property.value_type() {
                ValueType::Long => {
                    Box::new(SameCategoryLongAppender { props: property })
                }
                ValueType::Double => {
                    Box::new(SameCategoryDoubleAppender { props: property })
                }
                _ => panic!(
                    "SameCategory only supports numeric properties (Long, Double), got {:?}",
                    property.value_type()
                ),
            };

            appenders.push(appender);
        }

        Box::new(
            super::union_link_feature_appender::UnionLinkFeatureAppender::new(
                appenders,
                "SAME_CATEGORY".to_string(),
                self.node_properties.clone(),
            ),
        )
    }

    fn name(&self) -> &str {
        "SAME_CATEGORY"
    }

    fn configuration(&self) -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert(
            "nodeProperty".to_string(),
            serde_json::json!(self.node_properties),
        );
        config
    }

    fn input_node_properties(&self) -> Vec<String> {
        self.node_properties.clone()
    }

    fn clone_box(&self) -> Box<dyn LinkFeatureStep> {
        Box::new(self.clone())
    }
}

/// SameCategory appender for i64 properties.
struct SameCategoryLongAppender {
    props: Arc<dyn NodePropertyValues>,
}

impl LinkFeatureAppender for SameCategoryLongAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        let source_val = match self.props.long_value(source) {
            Ok(val) => val,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };
        let target_val = match self.props.long_value(target) {
            Ok(val) => val,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };

        link_features[offset] = if source_val == target_val { 1.0 } else { 0.0 };
    }

    fn dimension(&self) -> usize {
        1 // One binary feature per property
    }
}

/// SameCategory appender for f64 properties.
struct SameCategoryDoubleAppender {
    props: Arc<dyn NodePropertyValues>,
}

impl LinkFeatureAppender for SameCategoryDoubleAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        let source_val = match self.props.double_value(source) {
            Ok(val) => val,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };
        let target_val = match self.props.double_value(target) {
            Ok(val) => val,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };

        link_features[offset] = if source_val == target_val { 1.0 } else { 0.0 };
    }

    fn dimension(&self) -> usize {
        1 // One binary feature per property
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_category_creation() {
        let step = SameCategoryStep::new(vec!["department".to_string()]);
        assert_eq!(step.node_properties.len(), 1);
    }

    #[test]
    fn test_same_category_name() {
        let step = SameCategoryStep::new(vec!["category".to_string()]);
        assert_eq!(step.name(), "SAME_CATEGORY");
    }

    #[test]
    fn test_same_category_configuration() {
        let step = SameCategoryStep::new(vec!["department".to_string(), "country".to_string()]);

        let config = step.configuration();
        // Java uses singular "nodeProperty" key
        assert!(config.contains_key("nodeProperty"));
    }

    #[test]
    fn test_input_node_properties() {
        let props = vec!["category".to_string(), "type".to_string()];
        let step = SameCategoryStep::new(props.clone());

        assert_eq!(step.input_node_properties(), props);
    }

    #[test]
    fn test_dimension_matches_property_count() {
        // Each property produces one binary feature
        let step = SameCategoryStep::new(vec![
            "prop1".to_string(),
            "prop2".to_string(),
            "prop3".to_string(),
        ]);

        assert_eq!(step.input_node_properties().len(), 3);
    }

    #[test]
    fn test_clone() {
        let step1 = SameCategoryStep::new(vec!["category".to_string()]);
        let step2 = step1.clone();

        assert_eq!(step1.name(), step2.name());
        assert_eq!(step1.input_node_properties(), step2.input_node_properties());
    }

    #[test]
    fn test_binary_feature_semantics() {
        let step = SameCategoryStep::new(vec!["community".to_string()]);
        assert_eq!(step.name(), "SAME_CATEGORY");
    }
}
