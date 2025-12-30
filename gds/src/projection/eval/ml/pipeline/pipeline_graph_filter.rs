//! Java GDS: pipeline/src/main/java/org/neo4j/gds/ml/pipeline/PipelineGraphFilter.java
//!
//! Filter struct for dataset splits (node labels + relationship types).
//!
//! This is a simple value class that specifies which node labels and relationship types
//! should be included when filtering a graph for a specific dataset split (TRAIN, TEST, etc.).

/// Filter specification for dataset splits in pipeline execution.
///
/// Specifies which node labels and relationship types should be included
/// when creating filtered graph views for training, testing, or feature input.
///
/// # Java Source
/// ```java
/// @ValueClass
/// public interface PipelineGraphFilter {
///     Collection<NodeLabel> nodeLabels();
///
///     @Value.Default
///     default Collection<RelationshipType> relationshipTypes() {
///         return List.of();
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineGraphFilter {
    /// Node labels to include in the filtered graph
    pub node_labels: Vec<String>,

    /// Relationship types to include in the filtered graph (defaults to empty)
    pub relationship_types: Vec<String>,
}

impl PipelineGraphFilter {
    /// Create a new pipeline graph filter.
    ///
    /// # Arguments
    /// * `node_labels` - Node labels to include
    /// * `relationship_types` - Relationship types to include (defaults to empty if None)
    pub fn new(node_labels: Vec<String>, relationship_types: Option<Vec<String>>) -> Self {
        Self {
            node_labels,
            relationship_types: relationship_types.unwrap_or_default(),
        }
    }

    /// Create a filter with only node labels (no relationship type filtering).
    pub fn with_node_labels(node_labels: Vec<String>) -> Self {
        Self {
            node_labels,
            relationship_types: Vec::new(),
        }
    }
}

impl Default for PipelineGraphFilter {
    fn default() -> Self {
        Self {
            node_labels: Vec::new(),
            relationship_types: Vec::new(),
        }
    }
}
