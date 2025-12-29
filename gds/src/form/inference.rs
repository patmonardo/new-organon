// GraphStore inference trait definitions (moved to form)
// See gds/doc/GNN-GRAPHSTORE-INFERENCE-LAYER.md for architecture context

use std::collections::HashMap;

/// Identifier for a node in a GraphStore (domain + local ID)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId {
    pub domain: String,
    pub local_id: String,
}

/// Identifier for an edge type between domains
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EdgeType {
    pub name: String,
}

/// Constraint that must hold during traversal
#[derive(Debug, Clone)]
pub enum TraversalConstraint {
    /// Ensure path is reversible (can trace back)
    Reversibility,
    /// Type constraints must be satisfied at each hop
    TypeSafety,
    /// Custom constraint with validation predicate name
    Custom(String),
}

/// A path through the graph with provenance
#[derive(Debug, Clone)]
pub struct InferencePath {
    pub nodes: Vec<NodeId>,
    pub edges: Vec<EdgeType>,
    /// Evidence that constraints were satisfied
    pub constraint_proofs: HashMap<String, String>,
}

/// Result of a graph traversal query
#[derive(Debug, Clone)]
pub struct InferenceResult {
    pub valid: bool,
    pub paths: Vec<InferencePath>,
    pub error: Option<String>,
}

/// Core trait for graph inference operations
///
/// Implementations must ensure:
/// - Read-only (no mutation of underlying Postgres data)
/// - Constraint validation at each traversal step
/// - Auditable results (include provenance/proofs)
pub trait GraphInference {
    /// Check if there exists a valid path from start to target
    /// respecting the given constraints
    fn reachable(
        &self,
        start: &NodeId,
        target: &NodeId,
        constraints: &[TraversalConstraint],
    ) -> Result<bool, String>;

    /// Find all paths from start to target satisfying constraints
    /// Returns ranked paths (by constraint satisfaction strength)
    fn traverse(
        &self,
        start: &NodeId,
        target: &NodeId,
        constraints: &[TraversalConstraint],
        max_paths: usize,
    ) -> Result<InferenceResult, String>;

    /// Propagate constraints from a source node through the graph
    /// Returns affected nodes and the constraints they inherit
    fn propagate_constraints(
        &self,
        source: &NodeId,
        constraint_name: &str,
        max_depth: usize,
    ) -> Result<HashMap<NodeId, Vec<String>>, String>;

    /// Validate that a proposed path satisfies all constraints
    /// Used by Agent to verify LLM suggestions before execution
    fn validate_path(
        &self,
        path: &InferencePath,
        constraints: &[TraversalConstraint],
    ) -> Result<bool, String>;
}

/// Builder for creating GraphStore instances from Postgres/DuckDB
pub trait GraphStoreBuilder {
    /// Extract SDSL instances from Postgres as a GraphStore
    /// Domains: which SDSL schemas to include
    /// Include edges: which cross-domain relations to materialize
    fn build_from_postgres(
        &self,
        connection_string: &str,
        domains: &[String],
        include_edges: &[EdgeType],
    ) -> Result<Box<dyn GraphInference>, String>;

    /// Build GraphStore from DuckDB view (for pre-materialized analytics)
    fn build_from_duckdb(
        &self,
        db_path: &str,
        view_name: &str,
    ) -> Result<Box<dyn GraphInference>, String>;

    /// Build GraphStore from Polars DataFrame (in-memory, columnar)
    fn build_from_polars(
        &self,
        nodes_df: &str, // JSON-serialized Polars DataFrame
        edges_df: &str,
    ) -> Result<Box<dyn GraphInference>, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_equality() {
        let n1 = NodeId {
            domain: "sdsl1".to_string(),
            local_id: "concept_a".to_string(),
        };
        let n2 = NodeId {
            domain: "sdsl1".to_string(),
            local_id: "concept_a".to_string(),
        };
        assert_eq!(n1, n2);
    }

    #[test]
    fn test_inference_path_creation() {
        let path = InferencePath {
            nodes: vec![
                NodeId {
                    domain: "sdsl1".to_string(),
                    local_id: "a".to_string(),
                },
                NodeId {
                    domain: "sdsl2".to_string(),
                    local_id: "b".to_string(),
                },
            ],
            edges: vec![EdgeType {
                name: "relates_to".to_string(),
            }],
            constraint_proofs: HashMap::from([("reversibility".to_string(), "valid".to_string())]),
        };
        assert_eq!(path.nodes.len(), 2);
        assert_eq!(path.edges.len(), 1);
    }
}
