//! Computation hook (Java parity).
//!
//! Java reference: `Computation<RESULT>.compute(Graph graph, GraphStore graphStore)`.

use crate::procedures::Graph;
use crate::types::graph_store::DefaultGraphStore;

pub trait Computation<RESULT> {
    fn compute(&self, graph: &Graph, graph_store: &DefaultGraphStore) -> RESULT;
}

impl<RESULT, F> Computation<RESULT> for F
where
    F: Fn(&Graph, &DefaultGraphStore) -> RESULT + Send + Sync,
{
    fn compute(&self, graph: &Graph, graph_store: &DefaultGraphStore) -> RESULT {
        (self)(graph, graph_store)
    }
}
