use crate::applications::graph_store_catalog::loaders::GraphStoreCatalogService;
use crate::applications::graph_store_catalog::results::GraphFilterResult;
use crate::applications::services::logging::Log;
use crate::core::User;
use crate::types::graph_store::{DatabaseId, GraphName, GraphStore};

use rand::rngs::StdRng;
use rand::{seq::SliceRandom, SeedableRng};
use std::sync::Arc;
use std::time::Instant;

/// Projects a subgraph from an existing graph in the catalog.
///
/// Java parity: `SubGraphProjectApplication` filters based on Cypher-like node/relationship
/// filters.
/// Rust pass-1: we implement deterministic induced-subgraph projection by selecting a subset
/// of nodes and then committing an induced subgraph.
pub struct SubGraphProjectApplication {
    #[allow(dead_code)]
    log: Log,
    graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>,
}

impl SubGraphProjectApplication {
    pub fn new(log: Log, graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>) -> Self {
        Self {
            log,
            graph_store_catalog_service,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        origin_graph_name: &str,
        node_filter: &str,
        relationship_filter: &str,
        sample_node_count: Option<usize>,
        sample_ratio: Option<f64>,
        seed: Option<u64>,
    ) -> Result<GraphFilterResult, String> {
        let start = Instant::now();

        let origin_store = self.graph_store_catalog_service.get_graph_store(
            user,
            database_id,
            origin_graph_name,
        )?;

        let n = origin_store.node_count();
        let target = compute_target_node_count(n, sample_node_count, sample_ratio, node_filter);

        let mut candidates: Vec<i64> = (0..n as i64).collect();
        let mut rng = StdRng::seed_from_u64(seed.unwrap_or(0));
        candidates.shuffle(&mut rng);
        candidates.truncate(target);

        // Convert mapped node IDs -> original node IDs.
        let id_map = origin_store.nodes();
        let mut selected_original: Vec<i64> = Vec::with_capacity(candidates.len());
        for mapped_id in candidates {
            if let Some(original) = id_map.to_original_node_id(mapped_id) {
                selected_original.push(original);
            }
        }

        let induced = origin_store
            .commit_induced_subgraph_by_original_node_ids(
                GraphName::new(graph_name.to_string()),
                &selected_original,
            )
            .map_err(|e| format!("Failed to induce subgraph: {e}"))?;

        let node_count = induced.store.node_count() as u64;
        let relationship_count = induced.store.relationship_count() as u64;

        self.graph_store_catalog_service
            .graph_catalog(user, database_id)
            .set(graph_name, Arc::new(induced.store));

        let elapsed = start.elapsed().as_millis() as u64;
        Ok(GraphFilterResult::new(
            graph_name.to_string(),
            origin_graph_name.to_string(),
            node_filter.to_string(),
            relationship_filter.to_string(),
            node_count,
            relationship_count,
            elapsed,
        ))
    }
}

fn compute_target_node_count(
    n: usize,
    sample_node_count: Option<usize>,
    sample_ratio: Option<f64>,
    node_filter: &str,
) -> usize {
    if n == 0 {
        return 0;
    }
    if let Some(k) = sample_node_count {
        return k.clamp(0, n);
    }
    if let Some(r) = sample_ratio {
        let r = r.clamp(0.0, 1.0);
        let k = ((n as f64) * r).round() as usize;
        return k.clamp(0, n);
    }
    // If a filter is specified, default to keeping half; otherwise keep all.
    let t = node_filter.trim();
    if t.is_empty() || t == "*" || t.eq_ignore_ascii_case("true") {
        n
    } else {
        (n / 2).max(1)
    }
}
