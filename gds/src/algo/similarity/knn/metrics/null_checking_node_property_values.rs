use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::PropertyValues;
use crate::types::properties::{PropertyValuesError, PropertyValuesResult};
use crate::types::ValueType;
use std::sync::Arc;

/// A wrapper for `NodePropertyValues` that fails fast when a node is missing a value.
///
/// This is useful for matching “missing value is an error” semantics on array properties.
#[derive(Debug, Clone)]
pub struct NullCheckingNodePropertyValues {
    inner: Arc<dyn NodePropertyValues>,
    name: String,
}

impl NullCheckingNodePropertyValues {
    pub fn new(inner: Arc<dyn NodePropertyValues>, name: impl Into<String>) -> Self {
        Self {
            inner,
            name: name.into(),
        }
    }

    fn check(&self, node_id: u64) -> PropertyValuesResult<()> {
        if self.inner.has_value(node_id) {
            Ok(())
        } else {
            Err(PropertyValuesError::ValueNotFound(node_id))
        }
    }

    fn missing_message(&self, node_id: u64) -> String {
        format!(
            "Missing `{}` node property `{}` for node with id `{}`.",
            self.inner.value_type().name(),
            self.name,
            node_id
        )
    }
}

impl PropertyValues for NullCheckingNodePropertyValues {
    fn value_type(&self) -> ValueType {
        self.inner.value_type()
    }

    fn element_count(&self) -> usize {
        self.inner.element_count()
    }
}

impl NodePropertyValues for NullCheckingNodePropertyValues {
    fn double_value(&self, node_id: u64) -> PropertyValuesResult<f64> {
        self.check(node_id).map_err(|_| {
            PropertyValuesError::UnsupportedOperation(self.missing_message(node_id))
        })?;
        self.inner.double_value(node_id)
    }

    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        self.check(node_id).map_err(|_| {
            PropertyValuesError::UnsupportedOperation(self.missing_message(node_id))
        })?;
        self.inner.long_value(node_id)
    }

    fn double_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        self.check(node_id).map_err(|_| {
            PropertyValuesError::UnsupportedOperation(self.missing_message(node_id))
        })?;
        self.inner.double_array_value(node_id)
    }

    fn float_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        self.check(node_id).map_err(|_| {
            PropertyValuesError::UnsupportedOperation(self.missing_message(node_id))
        })?;
        self.inner.float_array_value(node_id)
    }

    fn long_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        self.check(node_id).map_err(|_| {
            PropertyValuesError::UnsupportedOperation(self.missing_message(node_id))
        })?;
        self.inner.long_array_value(node_id)
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        self.check(node_id).map_err(|_| {
            PropertyValuesError::UnsupportedOperation(self.missing_message(node_id))
        })?;
        self.inner.get_object(node_id)
    }

    fn dimension(&self) -> Option<usize> {
        self.inner.dimension()
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        self.inner.get_max_long_property_value()
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        self.inner.get_max_double_property_value()
    }

    fn has_value(&self, node_id: u64) -> bool {
        self.inner.has_value(node_id)
    }
}
