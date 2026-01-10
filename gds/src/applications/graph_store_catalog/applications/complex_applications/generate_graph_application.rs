use std::sync::Arc;

use crate::applications::graph_store_catalog::configs::GraphGenerationConfig;
use crate::applications::graph_store_catalog::loaders::GraphStoreCatalogService;
use crate::applications::graph_store_catalog::results::GenerationResult;
use crate::applications::services::logging::Log;
use crate::core::User;
use crate::types::graph_store::{DatabaseId, DefaultGraphStore};
use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig, Randomizable};
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::types::graph_store::GraphStore as _;

/// GenerateGraphApplication
///
/// Java parity: generates a synthetic graph and stores it in the catalog.
/// Rust pass-1: uses `DefaultGraphStore::random_with_rng`.
#[derive(Clone)]
pub struct GenerateGraphApplication {
    #[allow(dead_code)]
    log: Log,
    graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>,
}

impl GenerateGraphApplication {
    pub fn new(log: Log, graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>) -> Self {
        Self {
            log,
            graph_store_catalog_service,
        }
    }

    pub fn compute(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        cfg: &GraphGenerationConfig,
    ) -> Result<GenerationResult, String> {
        let started = std::time::Instant::now();

        let graph_name = cfg
            .graph_name
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("generated_graph")
            .to_string();

        let node_count = cfg.node_count.unwrap_or(16);
        if node_count == 0 {
            return Err("generation_config.nodeCount must be > 0".to_string());
        }

        let node_labels = if cfg.node_labels.is_empty() {
            vec!["Node".to_string()]
        } else {
            cfg.node_labels.clone()
        };

        let relationships: Vec<RandomRelationshipConfig> = if cfg.relationships.is_empty() {
            vec![RandomRelationshipConfig::new("REL", 0.1)]
        } else {
            cfg.relationships
                .iter()
                .map(|r| RandomRelationshipConfig::new(r.relationship_type.clone(), r.probability))
                .collect()
        };

        let config = RandomGraphConfig {
            graph_name: graph_name.clone(),
            database_name: database_id.to_string(),
            node_count: node_count as usize,
            node_labels,
            relationships,
            directed: cfg.directed.unwrap_or(true),
            inverse_indexed: cfg.inverse_indexed.unwrap_or(true),
            seed: cfg.seed,
        };

        let seed = cfg.seed.unwrap_or(0);
        let mut rng = StdRng::seed_from_u64(seed);
        let store: DefaultGraphStore = DefaultGraphStore::random_with_rng(&config, &mut rng)
            .map_err(|e| format!("Failed to generate graph store: {e}"))?;

        let nodes_generated = store.node_count() as u64;
        let relationships_generated = store.relationship_count() as u64;

        self.graph_store_catalog_service
            .graph_catalog(user, database_id)
            .set(graph_name.as_str(), Arc::new(store));

        Ok(GenerationResult::new(
            graph_name,
            nodes_generated,
            relationships_generated,
            started.elapsed().as_millis() as u64,
        ))
    }
}
