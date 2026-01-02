use crate::concurrency::TerminationFlag;
use crate::core::utils::queue::BoundedLongLongPriorityQueue;
use crate::types::graph::graph::Graph;
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::relationship::traits::RelationshipPredicate;
use crate::types::ValueType;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub trait KgeGraph: Graph + RelationshipPredicate {}

impl<T> KgeGraph for T where T: Graph + RelationshipPredicate {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScoreFunction {
    Transe,
    Distmult,
}

impl ScoreFunction {
    pub fn higher_is_better(self) -> bool {
        matches!(self, ScoreFunction::Distmult)
    }
}

#[derive(Debug, Clone)]
pub struct KgePredictParameters {
    pub node_embedding_property: String,
    pub relationship_type_embedding: Vec<f64>,
    pub scoring_function: ScoreFunction,
    pub top_k: usize,
    pub source_nodes: Option<Vec<i64>>,
    pub target_nodes: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct KgePrediction {
    pub source_node_id: i64,
    pub target_node_id: i64,
    pub score: f64,
}

#[derive(Debug, Clone)]
pub struct KgePredictResult {
    by_source: HashMap<i64, Vec<KgePrediction>>,
    links_considered: u64,
}

impl KgePredictResult {
    pub fn by_source(&self) -> &HashMap<i64, Vec<KgePrediction>> {
        &self.by_source
    }

    pub fn links_considered(&self) -> u64 {
        self.links_considered
    }

    pub fn relationship_count(&self) -> u64 {
        self.by_source.values().map(|v| v.len() as u64).sum()
    }

    pub fn iter(&self) -> impl Iterator<Item = &KgePrediction> {
        self.by_source.values().flat_map(|v| v.iter())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum KgeError {
    #[error("missing node property: {0}")]
    MissingNodeProperty(String),

    #[error("unsupported embedding value type: {0}")]
    UnsupportedEmbeddingValueType(String),

    #[error("invalid relationshipTypeEmbedding length")]
    InvalidRelationshipTypeEmbedding,

    #[error("topK must be > 0")]
    InvalidTopK,
}

pub fn compute_kge_predict(
    graph: &dyn KgeGraph,
    parameters: &KgePredictParameters,
    termination_flag: &TerminationFlag,
) -> Result<KgePredictResult, KgeError> {
    if parameters.top_k == 0 {
        return Err(KgeError::InvalidTopK);
    }

    let embeddings = graph
        .node_properties(&parameters.node_embedding_property)
        .ok_or_else(|| KgeError::MissingNodeProperty(parameters.node_embedding_property.clone()))?;

    let dim = parameters.relationship_type_embedding.len();
    if dim == 0 {
        return Err(KgeError::InvalidRelationshipTypeEmbedding);
    }

    let node_count = graph.node_count() as i64;
    let sources: Vec<i64> = parameters
        .source_nodes
        .clone()
        .unwrap_or_else(|| (0..node_count).collect());
    let targets: Vec<i64> = parameters
        .target_nodes
        .clone()
        .unwrap_or_else(|| (0..node_count).collect());

    let higher_is_better = parameters.scoring_function.higher_is_better();

    // Use per-source bounded queues; parallelize across sources.
    let embeddings = Arc::clone(&embeddings);
    let relationship_type_embedding = parameters.relationship_type_embedding.clone();

    let results: Vec<(i64, Vec<KgePrediction>, u64)> = sources
        .par_iter()
        .map(|&source| {
            termination_flag.assert_running();

            let mut scorer = LinkScorer::new(
                Arc::clone(&embeddings),
                relationship_type_embedding.clone(),
                parameters.scoring_function,
            );

            scorer.init(source);

            let mut queue = if higher_is_better {
                BoundedLongLongPriorityQueue::max(parameters.top_k)
            } else {
                BoundedLongLongPriorityQueue::min(parameters.top_k)
            };

            let mut considered: u64 = 0;

            for &target in &targets {
                if source == target {
                    continue;
                }
                if graph.exists(source, target) {
                    continue;
                }

                considered += 1;

                let score = scorer.compute_score(target);
                if score.is_nan() {
                    continue;
                }

                let _ = queue.offer(source, target, score);
            }

            let mut preds = Vec::with_capacity(queue.size());
            queue.for_each(|s, t, p| {
                preds.push(KgePrediction {
                    source_node_id: s,
                    target_node_id: t,
                    score: p,
                })
            });

            (source, preds, considered)
        })
        .collect();

    let mut by_source = HashMap::new();
    let mut links_considered = 0u64;

    for (source, preds, considered) in results {
        links_considered += considered;
        if !preds.is_empty() {
            by_source.insert(source, preds);
        }
    }

    Ok(KgePredictResult {
        by_source,
        links_considered,
    })
}

// -------------------------------------------------------------------------------------
// Scoring (clean-room reimplementation of Java LinkScorer + LinkScorerFactory)
// -------------------------------------------------------------------------------------

struct LinkScorer {
    embeddings: Arc<dyn NodePropertyValues>,
    relationship_type_embedding: Vec<f64>,
    score_function: ScoreFunction,
    current_candidate_target_f64: Vec<f64>,
    current_candidate_target_f32: Vec<f32>,
    use_f32: bool,
}

impl LinkScorer {
    fn new(
        embeddings: Arc<dyn NodePropertyValues>,
        relationship_type_embedding: Vec<f64>,
        score_function: ScoreFunction,
    ) -> Self {
        let use_f32 = embeddings.value_type() == ValueType::FloatArray;
        let dim = relationship_type_embedding.len();

        Self {
            embeddings,
            relationship_type_embedding,
            score_function,
            current_candidate_target_f64: vec![0.0; dim],
            current_candidate_target_f32: vec![0.0; dim],
            use_f32,
        }
    }

    fn init(&mut self, source_node: i64) {
        let dim = self.relationship_type_embedding.len();

        if self.use_f32 {
            let current_source = self
                .embeddings
                .float_array_value(source_node as u64)
                .unwrap_or_else(|_| vec![0.0; dim]);

            match self.score_function {
                ScoreFunction::Transe => {
                    for i in 0..dim {
                        self.current_candidate_target_f32[i] =
                            current_source[i] + self.relationship_type_embedding[i] as f32;
                    }
                }
                ScoreFunction::Distmult => {
                    for i in 0..dim {
                        self.current_candidate_target_f32[i] =
                            current_source[i] * self.relationship_type_embedding[i] as f32;
                    }
                }
            }
        } else {
            let current_source = self
                .embeddings
                .double_array_value(source_node as u64)
                .unwrap_or_else(|_| vec![0.0; dim]);

            match self.score_function {
                ScoreFunction::Transe => {
                    for i in 0..dim {
                        self.current_candidate_target_f64[i] =
                            current_source[i] + self.relationship_type_embedding[i];
                    }
                }
                ScoreFunction::Distmult => {
                    for i in 0..dim {
                        self.current_candidate_target_f64[i] =
                            current_source[i] * self.relationship_type_embedding[i];
                    }
                }
            }
        }
    }

    fn compute_score(&self, target_node: i64) -> f64 {
        let dim = self.relationship_type_embedding.len();

        if self.use_f32 {
            let target_vec = self
                .embeddings
                .float_array_value(target_node as u64)
                .unwrap_or_else(|_| vec![0.0; dim]);

            match self.score_function {
                ScoreFunction::Distmult => {
                    let mut res = 0.0f64;
                    for i in 0..dim {
                        res += (self.current_candidate_target_f32[i] * target_vec[i]) as f64;
                    }
                    res
                }
                ScoreFunction::Transe => {
                    let mut res = 0.0f64;
                    for i in 0..dim {
                        let elem = (self.current_candidate_target_f32[i] - target_vec[i]) as f64;
                        res += elem * elem;
                    }
                    res.sqrt()
                }
            }
        } else {
            let target_vec = self
                .embeddings
                .double_array_value(target_node as u64)
                .unwrap_or_else(|_| vec![0.0; dim]);

            match self.score_function {
                ScoreFunction::Distmult => {
                    let mut res = 0.0f64;
                    for i in 0..dim {
                        res += self.current_candidate_target_f64[i] * target_vec[i];
                    }
                    res
                }
                ScoreFunction::Transe => {
                    let mut res = 0.0f64;
                    for i in 0..dim {
                        let elem = self.current_candidate_target_f64[i] - target_vec[i];
                        res += elem * elem;
                    }
                    res.sqrt()
                }
            }
        }
    }
}
