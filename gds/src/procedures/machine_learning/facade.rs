//! Machine Learning procedure facade aggregator.
//!
//! This is the per-procedure call-site entrypoint (Java-shaped), similar in spirit
//! to Neo4j GDS's `LocalMachineLearningProcedureFacade`.
//!
//! Today we only expose KGE predict stream/stats because mutate/write steps are
//! not implemented in this repo yet.

use crate::algo::algorithms::Result;
use crate::procedures::machine_learning::{
    KgePredictFacade, KgePredictStats, KgeStreamResult, ScoreFunction,
};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::user::User;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// A loose map type for Java `Map<String, Object>` configurations.
pub type AnyMap = HashMap<String, Value>;

/// Request-scoped dependencies for ML procedures.
#[derive(Clone, Debug)]
pub struct RequestScopedDependencies {
    pub user: User,
}

impl RequestScopedDependencies {
    pub fn new(user: User) -> Self {
        Self { user }
    }
}

/// Procedure facade entrypoint for machine learning procedures.
#[derive(Clone)]
pub struct LocalMachineLearningProcedureFacade {
    _request_scoped_dependencies: RequestScopedDependencies,
    catalog: Arc<dyn GraphCatalog>,
}

impl LocalMachineLearningProcedureFacade {
    pub fn new(
        request_scoped_dependencies: RequestScopedDependencies,
        catalog: Arc<dyn GraphCatalog>,
    ) -> Self {
        Self {
            _request_scoped_dependencies: request_scoped_dependencies,
            catalog,
        }
    }

    pub fn kge_stream(
        &self,
        graph_name: &str,
        configuration: AnyMap,
    ) -> Result<Vec<KgeStreamResult>> {
        let graph_store = self
            .catalog
            .get(graph_name)
            .ok_or_else(|| AlgorithmError::Execution(format!("Graph '{graph_name}' not found")))?;

        let facade = build_kge_facade(graph_store, &configuration)?;

        facade.stream().map(|it| it.collect())
    }

    pub fn kge_stats(&self, graph_name: &str, configuration: AnyMap) -> Result<KgePredictStats> {
        let graph_store = self
            .catalog
            .get(graph_name)
            .ok_or_else(|| AlgorithmError::Execution(format!("Graph '{graph_name}' not found")))?;

        let facade = build_kge_facade(graph_store, &configuration)?;

        facade.stats()
    }
}

fn build_kge_facade(
    graph_store: Arc<DefaultGraphStore>,
    configuration: &AnyMap,
) -> Result<KgePredictFacade> {
    let node_embedding_property = configuration
        .get("nodeEmbeddingProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("embedding")
        .to_string();

    let relationship_type_embedding: Vec<f64> = configuration
        .get("relationshipTypeEmbedding")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_f64()).collect())
        .unwrap_or_default();

    let scoring_function = match configuration
        .get("scoringFunction")
        .and_then(|v| v.as_str())
        .unwrap_or("DISTMULT")
    {
        "TRANSE" => ScoreFunction::Transe,
        "DISTMULT" => ScoreFunction::Distmult,
        other => {
            return Err(AlgorithmError::Execution(format!(
                "Unknown scoringFunction '{other}'"
            )))
        }
    };

    let top_k = configuration
        .get("topK")
        .and_then(|v| v.as_u64())
        .unwrap_or(10) as usize;

    Ok(KgePredictFacade::new(graph_store)
        .node_embedding_property(node_embedding_property)
        .relationship_type_embedding(relationship_type_embedding)
        .scoring_function(scoring_function)
        .top_k(top_k))
}
