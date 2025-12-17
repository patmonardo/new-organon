use crate::substrate::{NodeId, RelTypeId};

/// A read-only runtime view over a graph topology (cursor/iterator oriented).
///
/// This trait is deliberately tiny; property access and schema are separate layers.
pub trait CoreGraph {
    fn node_count(&self) -> usize;

    /// Iterates neighbors of `node` for the given relationship type.
    fn neighbors<'a>(
        &'a self,
        node: NodeId,
        rel_type: RelTypeId,
    ) -> Box<dyn Iterator<Item = NodeId> + 'a>;
}

/// Placeholder CSR-backed view.
///
/// This exists only to provide a concrete type for early wiring.
#[derive(Debug, Clone)]
pub struct CsrGraphView {
    node_count: usize,
}

impl CsrGraphView {
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }
}

impl CoreGraph for CsrGraphView {
    fn node_count(&self) -> usize {
        self.node_count
    }

    fn neighbors<'a>(
        &'a self,
        _node: NodeId,
        _rel_type: RelTypeId,
    ) -> Box<dyn Iterator<Item = NodeId> + 'a> {
        Box::new(std::iter::empty())
    }
}
