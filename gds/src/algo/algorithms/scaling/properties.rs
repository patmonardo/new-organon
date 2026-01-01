use crate::pregel::NodeValue;

/// Minimal property access abstraction for scalers
pub trait ScalarPropertyValues {
    fn node_count(&self) -> usize;
    fn value(&self, node_id: usize) -> f64;
}

/// Adapter over a slice of f64 values
pub struct VecPropertyValues<'a> {
    data: &'a [f64],
}

impl<'a> VecPropertyValues<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        Self { data }
    }
}

impl<'a> ScalarPropertyValues for VecPropertyValues<'a> {
    fn node_count(&self) -> usize {
        self.data.len()
    }

    fn value(&self, node_id: usize) -> f64 {
        self.data[node_id]
    }
}

/// Adapter over Pregel `NodeValue` for a specific key
pub struct NodeValuePropertyValues<'a> {
    node_values: &'a NodeValue,
    key: &'a str,
    node_count: usize,
}

impl<'a> NodeValuePropertyValues<'a> {
    pub fn new(node_values: &'a NodeValue, key: &'a str, node_count: usize) -> Self {
        Self {
            node_values,
            key,
            node_count,
        }
    }
}

impl<'a> ScalarPropertyValues for NodeValuePropertyValues<'a> {
    fn node_count(&self) -> usize {
        self.node_count
    }

    fn value(&self, node_id: usize) -> f64 {
        self.node_values.double_value(self.key, node_id)
    }
}


