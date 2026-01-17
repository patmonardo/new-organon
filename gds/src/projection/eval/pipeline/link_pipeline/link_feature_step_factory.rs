// LinkFeatureStepFactory - Factory enum for creating LinkFeatureStep instances

use super::link_functions::{
    CosineFeatureStep, HadamardFeatureStep, L2FeatureStep, SameCategoryStep,
};
use super::LinkFeatureStep;
use serde_json::Value;
use std::fmt;

/// Factory enum for creating LinkFeatureStep instances from configuration.
///
/// This enum provides a type-safe way to create different link feature extraction
/// steps (Hadamard, Cosine, L2, SameCategory) from string names or configuration.
///
/// # Usage
///
/// ```text
/// // From string name
/// let factory = LinkFeatureStepFactory::parse("HADAMARD")?;
/// let step = factory.create(vec!["embedding".to_string()]);
///
/// // Direct creation
/// let step = LinkFeatureStepFactory::create_from_name("COSINE", vec!["features".to_string()])?;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkFeatureStepFactory {
    /// Hadamard product (element-wise multiplication)
    Hadamard,
    /// Cosine similarity (angular distance)
    Cosine,
    /// L2 distance (Euclidean distance squared)
    L2,
    /// Same category (categorical equality)
    SameCategory,
}

impl LinkFeatureStepFactory {
    /// All valid factory values as strings (uppercase).
    pub const VALUES: &'static [&'static str] = &["HADAMARD", "COSINE", "L2", "SAME_CATEGORY"];

    /// Parse factory from string (case-insensitive).
    ///
    /// # Arguments
    ///
    /// * `input` - Factory name (case-insensitive)
    ///
    /// # Returns
    ///
    /// Factory variant or error if unknown.
    pub fn parse(input: &str) -> Result<Self, String> {
        let input_upper = input.to_uppercase();

        match input_upper.as_str() {
            "HADAMARD" => Ok(Self::Hadamard),
            "COSINE" => Ok(Self::Cosine),
            "L2" => Ok(Self::L2),
            "SAME_CATEGORY" | "SAMECATEGORY" => Ok(Self::SameCategory),
            _ => Err(format!(
                "LinkFeatureStep `{}` is not supported. Must be one of: {}.",
                input,
                Self::VALUES.join(", ")
            )),
        }
    }

    /// Create a LinkFeatureStep instance from this factory.
    ///
    /// # Arguments
    ///
    /// * `node_properties` - List of node property names to use for feature extraction
    ///
    /// # Returns
    ///
    /// Boxed LinkFeatureStep instance.
    pub fn create(&self, node_properties: Vec<String>) -> Box<dyn LinkFeatureStep> {
        match self {
            Self::Hadamard => Box::new(HadamardFeatureStep::new(node_properties)),
            Self::Cosine => Box::new(CosineFeatureStep::new(node_properties)),
            Self::L2 => Box::new(L2FeatureStep::new(node_properties)),
            Self::SameCategory => Box::new(SameCategoryStep::new(node_properties)),
        }
    }

    /// Create a LinkFeatureStep from task name and configuration.
    ///
    /// Create a LinkFeatureStep from task name and node properties.
    ///
    /// # Arguments
    ///
    /// * `task_name` - Name of the feature step type (e.g., "HADAMARD", "COSINE")
    /// * `node_properties` - List of node property names to use
    ///
    /// # Returns
    ///
    /// Boxed LinkFeatureStep or error if task name is unknown.
    pub fn create_from_name(
        task_name: &str,
        node_properties: Vec<String>,
    ) -> Result<Box<dyn LinkFeatureStep>, String> {
        let factory = Self::parse(task_name)?;
        Ok(factory.create(node_properties))
    }

    /// Create from name with JSON config validation.
    pub fn create_from_config(
        task_name: &str,
        config: &Value,
    ) -> Result<Box<dyn LinkFeatureStep>, String> {
        // Validate and extract node properties using the trait's validation
        // Note: Once we have a concrete config struct, use it here instead of ad-hoc JSON parsing.
        // For now, directly extract array
        let props_array = config
            .as_array()
            .ok_or_else(|| "Configuration must be an array of property names".to_string())?;

        let node_properties: Result<Vec<String>, String> = props_array
            .iter()
            .map(|v| {
                v.as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| "Property name must be a string".to_string())
            })
            .collect();

        Self::create_from_name(task_name, node_properties?)
    }

    /// Returns the name of this factory variant.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Hadamard => "HADAMARD",
            Self::Cosine => "COSINE",
            Self::L2 => "L2",
            Self::SameCategory => "SAME_CATEGORY",
        }
    }
}

impl fmt::Display for LinkFeatureStepFactory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_hadamard() {
        assert_eq!(
            LinkFeatureStepFactory::parse("HADAMARD").unwrap(),
            LinkFeatureStepFactory::Hadamard
        );
        assert_eq!(
            LinkFeatureStepFactory::parse("hadamard").unwrap(),
            LinkFeatureStepFactory::Hadamard
        );
    }

    #[test]
    fn test_parse_cosine() {
        assert_eq!(
            LinkFeatureStepFactory::parse("COSINE").unwrap(),
            LinkFeatureStepFactory::Cosine
        );
    }

    #[test]
    fn test_parse_l2() {
        assert_eq!(
            LinkFeatureStepFactory::parse("L2").unwrap(),
            LinkFeatureStepFactory::L2
        );
        assert_eq!(
            LinkFeatureStepFactory::parse("l2").unwrap(),
            LinkFeatureStepFactory::L2
        );
    }

    #[test]
    fn test_parse_same_category() {
        assert_eq!(
            LinkFeatureStepFactory::parse("SAME_CATEGORY").unwrap(),
            LinkFeatureStepFactory::SameCategory
        );
        assert_eq!(
            LinkFeatureStepFactory::parse("same_category").unwrap(),
            LinkFeatureStepFactory::SameCategory
        );
    }

    #[test]
    fn test_parse_unknown() {
        let result = LinkFeatureStepFactory::parse("UNKNOWN");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not supported"));
    }

    #[test]
    fn test_create_hadamard() {
        let factory = LinkFeatureStepFactory::Hadamard;
        let step = factory.create(vec!["prop1".to_string()]);
        assert_eq!(step.name(), "HADAMARD");
    }

    #[test]
    fn test_create_cosine() {
        let factory = LinkFeatureStepFactory::Cosine;
        let step = factory.create(vec!["embedding".to_string()]);
        assert_eq!(step.name(), "COSINE");
    }

    #[test]
    fn test_create_l2() {
        let factory = LinkFeatureStepFactory::L2;
        let step = factory.create(vec!["features".to_string()]);
        assert_eq!(step.name(), "L2");
    }

    #[test]
    fn test_create_same_category() {
        let factory = LinkFeatureStepFactory::SameCategory;
        let step = factory.create(vec!["category".to_string()]);
        assert_eq!(step.name(), "SAME_CATEGORY");
    }

    #[test]
    fn test_create_from_name() {
        let props = vec!["prop1".to_string(), "prop2".to_string()];
        let step = LinkFeatureStepFactory::create_from_name("HADAMARD", props).unwrap();
        assert_eq!(step.name(), "HADAMARD");
        assert_eq!(step.input_node_properties().len(), 2);
    }

    #[test]
    fn test_create_from_config() {
        let config = json!(["prop1", "prop2"]);
        let step = LinkFeatureStepFactory::create_from_config("COSINE", &config).unwrap();
        assert_eq!(step.name(), "COSINE");
        assert_eq!(step.input_node_properties().len(), 2);
    }

    #[test]
    fn test_values_list() {
        assert_eq!(LinkFeatureStepFactory::VALUES.len(), 4);
        assert!(LinkFeatureStepFactory::VALUES.contains(&"HADAMARD"));
        assert!(LinkFeatureStepFactory::VALUES.contains(&"COSINE"));
        assert!(LinkFeatureStepFactory::VALUES.contains(&"L2"));
        assert!(LinkFeatureStepFactory::VALUES.contains(&"SAME_CATEGORY"));
    }

    #[test]
    fn test_parse_and_create() {
        let factory = LinkFeatureStepFactory::parse("COSINE").unwrap();
        assert_eq!(factory, LinkFeatureStepFactory::Cosine);

        let step = factory.create(vec!["embedding".to_string()]);
        assert_eq!(step.name(), "COSINE");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", LinkFeatureStepFactory::Hadamard), "HADAMARD");
        assert_eq!(format!("{}", LinkFeatureStepFactory::Cosine), "COSINE");
        assert_eq!(format!("{}", LinkFeatureStepFactory::L2), "L2");
        assert_eq!(
            format!("{}", LinkFeatureStepFactory::SameCategory),
            "SAME_CATEGORY"
        );
    }
}
