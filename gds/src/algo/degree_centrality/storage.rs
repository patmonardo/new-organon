//! Degree Centrality Storage Runtime
//!
//! This module implements the **Gross pole** of the Functor machinery for Degree Centrality.
//! It represents persistent data structures (GraphStore and graph topology).
//!
//! **Translation Source**: `org.neo4j.gds.degree.DegreeCentrality.java`
//! **Key Features**: Orientation handling, weighted/unweighted, parallel execution

use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::orientation as projection_orientation;
use crate::types::graph::id_map::NodeId;
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

/// Edge orientation for degree computation
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Orientation {
    /// Natural orientation (outgoing edges)
    Natural,
    /// Reverse orientation (incoming edges)
    Reverse,
    /// Undirected (both incoming and outgoing)
    Undirected,
}

fn to_projection_orientation(orientation: Orientation) -> projection_orientation::Orientation {
    match orientation {
        Orientation::Natural => projection_orientation::Orientation::Natural,
        Orientation::Reverse => projection_orientation::Orientation::Reverse,
        Orientation::Undirected => projection_orientation::Orientation::Undirected,
    }
}

/// Storage Runtime for Degree Centrality
///
/// This is the **Gross pole** - persistent data structures.
/// It knows how to access the graph structure and compute node degrees.
///
/// ## The Pole's Role
///
/// In the Functor machinery:
/// - **Storage Runtime** (Gross) = persistent GraphStore and graph topology
/// - **Computation Runtime** (Subtle) = ephemeral degree scores and statistics
/// - **Functor** = the mapping between them via degree computation
pub struct DegreeCentralityStorageRuntime<'a, G: GraphStore> {
    /// Reference to the graph store
    graph_store: &'a G,
    /// Oriented graph view used for degree computations.
    graph: Arc<dyn Graph>,
    /// Edge orientation for computation
    orientation: Orientation,
    /// Whether to use relationship weights
    has_relationship_weight_property: bool,
}

impl<'a, G: GraphStore> DegreeCentralityStorageRuntime<'a, G> {
    /// Create a new storage runtime
    pub fn new(graph_store: &'a G) -> Result<Self, AlgorithmError> {
        Self::with_settings(graph_store, Orientation::Natural, false)
    }

    /// Create with specific orientation and weight settings
    pub fn with_settings(
        graph_store: &'a G,
        orientation: Orientation,
        has_relationship_weight_property: bool,
    ) -> Result<Self, AlgorithmError> {
        let rel_types = HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(
                &rel_types,
                to_projection_orientation(orientation),
            )
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        Ok(Self {
            graph_store,
            graph,
            orientation,
            has_relationship_weight_property,
        })
    }

    /// Get reference to graph store
    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }

    /// Access the graph view used for computations.
    pub fn graph(&self) -> &Arc<dyn Graph> {
        &self.graph
    }

    /// Get node degree from storage
    ///
    /// This projects from GraphStore (Gross - persistent topology)
    /// to f64 (Subtle - degree count/weight).
    ///
    /// **This is where the Functor machinery actually works**:
    /// GraphStore (Gross) → f64 (Subtle)
    ///
    /// **Translation of Java logic**:
    /// - NATURAL: Use graph.degree() directly
    /// - REVERSE: Count incoming edges
    /// - UNDIRECTED: Count both incoming and outgoing
    /// - Weighted: Sum relationship weights
    /// - Unweighted: Count relationship count
    pub fn get_node_degree(&self, node_id: NodeId) -> Result<f64, AlgorithmError> {
        if self.has_relationship_weight_property {
            Ok(self
                .graph
                .stream_relationships_weighted(node_id, self.graph.default_property_value())
                .map(|cursor| cursor.weight())
                .sum())
        } else {
            Ok(self.graph.degree(node_id) as f64)
        }
    }

    /// Get total number of nodes
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get orientation setting
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Check if using relationship weights
    pub fn has_relationship_weight_property(&self) -> bool {
        self.has_relationship_weight_property
    }
}
