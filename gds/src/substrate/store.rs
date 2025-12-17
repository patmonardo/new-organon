use crate::substrate::SubstrateResult;

/// The minimal in-memory product of the Factory.
///
/// This is intentionally a skeleton: the goal is to converge on the *inevitable* fields
/// (schema, id map, relationship topologies, property stores, metadata, capabilities)
/// without importing the TS/Java service graph.
#[derive(Debug, Clone)]
pub struct CoreGraphStore {
    node_count: usize,
}

impl CoreGraphStore {
    /// Constructs a minimal store with a node count.
    pub fn new(node_count: usize) -> SubstrateResult<Self> {
        Ok(Self { node_count })
    }

    /// Number of nodes in the union graph.
    pub fn node_count(&self) -> usize {
        self.node_count
    }
}
