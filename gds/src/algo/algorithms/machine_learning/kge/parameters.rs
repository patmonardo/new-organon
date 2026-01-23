//! Parameter types and shared traits for KGE prediction.

use crate::types::graph::graph::Graph;
use crate::types::properties::relationship::RelationshipPredicate;
use serde::{Deserialize, Serialize};

/// Graph capabilities required by KGE prediction.
pub trait KgeGraph: Graph + RelationshipPredicate {}

impl<T> KgeGraph for T where T: Graph + RelationshipPredicate {}

/// Supported KGE scoring functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScoreFunction {
    Transe,
    Distmult,
}

impl ScoreFunction {
    /// Returns whether larger scores indicate better links.
    pub fn higher_is_better(self) -> bool {
        matches!(self, ScoreFunction::Distmult)
    }
}

/// Input parameters for KGE link prediction.
#[derive(Debug, Clone)]
pub struct KgePredictParameters {
    /// Node property name for embeddings.
    pub node_embedding_property: String,
    /// Embedding for the relationship type being predicted.
    pub relationship_type_embedding: Vec<f64>,
    /// Scoring function to apply.
    pub scoring_function: ScoreFunction,
    /// Maximum predictions per source node.
    pub top_k: usize,
    /// Optional source-node filter (node ids).
    pub source_nodes: Option<Vec<i64>>,
    /// Optional target-node filter (node ids).
    pub target_nodes: Option<Vec<i64>>,
}
