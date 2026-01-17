// Phase 2.6: AbstractLinkFeatureAppenderFactory - Abstract factory for type-specific appenders

use super::super::LinkFeatureAppender;
use crate::types::graph::Graph;
use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;
use std::sync::Arc;

/// Abstract factory for creating type-specific link feature appenders.
///
/// Implementations dispatch based on property value type to build the
/// appropriate appender.
pub trait AbstractLinkFeatureAppenderFactory {
    /// Creates a DoubleArray-typed appender.
    fn double_array_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender>;

    /// Creates a FloatArray-typed appender.
    fn float_array_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender>;

    /// Creates a LongArray-typed appender.
    fn long_array_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender>;

    /// Creates a Long scalar-typed appender.
    fn long_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender>;

    /// Creates a Double scalar-typed appender.
    fn double_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender>;

    /// Creates a single appender for the given property name.
    ///
    /// Inspects the property's ValueType in the graph and dispatches
    /// to the appropriate type-specific constructor method.
    ///
    /// Type dispatch:
    /// ```text
    /// match propertyType:
    ///   DoubleArray -> double_array_appender()
    ///   FloatArray  -> float_array_appender()
    ///   LongArray   -> long_array_appender()
    ///   Long        -> long_appender()
    ///   Double      -> double_appender()
    ///   Other       -> Error: Unsupported ValueType
    /// ```
    ///
    /// # Arguments
    ///
    /// * `graph` - Graph containing the property
    /// * `property_name` - Name of the property to create appender for
    ///
    /// # Returns
    ///
    /// Type-specific LinkFeatureAppender implementation.
    fn create_appender(
        &self,
        graph: &dyn Graph,
        property_name: &str,
    ) -> Result<Box<dyn LinkFeatureAppender>, String> {
        use crate::projection::eval::pipeline::feature_step_util::property_dimension;

        // Get the property values from the graph
        let props = graph
            .node_properties(property_name)
            .ok_or_else(|| format!("Property '{}' not found in graph", property_name))?;

        // Get the property type
        let property_type = props.value_type();

        // Get the dimension
        let dimension = property_dimension(&*props, property_name).map_err(|e| {
            format!(
                "Failed to get dimension for property '{}': {:?}",
                property_name, e
            )
        })?;

        // Dispatch to the appropriate type-specific constructor
        match property_type {
            ValueType::DoubleArray => Ok(self.double_array_appender(props, dimension)),
            ValueType::FloatArray => Ok(self.float_array_appender(props, dimension)),
            ValueType::LongArray => Ok(self.long_array_appender(props, dimension)),
            ValueType::Long => Ok(self.long_appender(props, dimension)),
            ValueType::Double => Ok(self.double_appender(props, dimension)),
            _ => Err(format!(
                "Unsupported ValueType {:?} for property '{}'",
                property_type, property_name
            )),
        }
    }

    /// Creates appenders for multiple properties.
    ///
    /// Returns array of appenders, one per property name.
    ///
    /// # Arguments
    ///
    /// * `graph` - Graph containing the properties
    /// * `property_names` - List of property names
    ///
    /// # Returns
    ///
    /// Vector of LinkFeatureAppenders, one per property.
    fn create_appenders(
        &self,
        graph: &dyn Graph,
        property_names: &[String],
    ) -> Result<Vec<Box<dyn LinkFeatureAppender>>, String> {
        property_names
            .iter()
            .map(|name| self.create_appender(graph, name))
            .collect()
    }
}
