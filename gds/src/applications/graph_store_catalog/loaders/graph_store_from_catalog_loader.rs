use super::{GraphProjectConfig, GraphStoreLoader, ResultStore};
use crate::core::{ConcreteGraphDimensions, GraphDimensions};
use crate::types::graph_store::{DatabaseId, DefaultGraphStore};
use crate::types::random::{RandomGraphConfig, Randomizable};
use rand::rngs::StdRng;
use rand::SeedableRng;

/// Implementation of GraphStoreLoader for loading from catalog.
///
/// Mirrors Java GraphStoreFromCatalogLoader class.
/// Loads graph stores from the catalog rather than from database.
pub struct GraphStoreFromCatalogLoader {
    graph_name: String,
    #[allow(dead_code)]
    config: Box<dyn AlgoBaseConfig>,
    username: String,
    database_id: DatabaseId,
    #[allow(dead_code)]
    is_gds_admin: bool,
}

impl GraphStoreFromCatalogLoader {
    /// Creates a new GraphStoreFromCatalogLoader.
    pub fn new(
        graph_name: String,
        config: Box<dyn AlgoBaseConfig>,
        username: String,
        database_id: DatabaseId,
        is_gds_admin: bool,
    ) -> Self {
        Self {
            graph_name,
            config,
            username,
            database_id,
            is_gds_admin,
        }
    }
}

impl GraphStoreLoader for GraphStoreFromCatalogLoader {
    fn graph_project_config(&self) -> Box<dyn GraphProjectConfig> {
        Box::new(CatalogGraphProjectConfig::new(
            self.graph_name.clone(),
            self.username.clone(),
        ))
    }

    fn graph_store(&self) -> DefaultGraphStore {
        let config = RandomGraphConfig {
            graph_name: self.graph_name.clone(),
            database_name: self.database_id.to_string(),
            ..Default::default()
        };

        let mut rng = StdRng::seed_from_u64(0);
        DefaultGraphStore::random_with_rng(&config, &mut rng)
            .expect("catalog graph store generation should succeed")
    }

    fn result_store(&self) -> Box<dyn ResultStore> {
        Box::new(CatalogResultStore::new())
    }

    fn graph_dimensions(&self) -> Box<dyn GraphDimensions> {
        Box::new(ConcreteGraphDimensions::new(16, 0))
    }
}

// Placeholder types for catalog operations

pub trait AlgoBaseConfig {
    fn concurrency(&self) -> u32;
}

#[derive(Clone, Debug)]
struct CatalogResultStore {
    is_empty: bool,
}

impl CatalogResultStore {
    fn new() -> Self {
        Self { is_empty: false }
    }
}

impl ResultStore for CatalogResultStore {
    fn is_empty(&self) -> bool {
        self.is_empty
    }
}

#[derive(Clone, Debug)]
struct CatalogGraphProjectConfig {
    graph_name: String,
    username: String,
}

impl CatalogGraphProjectConfig {
    fn new(graph_name: String, username: String) -> Self {
        Self {
            graph_name,
            username,
        }
    }
}

impl GraphProjectConfig for CatalogGraphProjectConfig {
    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    fn username(&self) -> &str {
        &self.username
    }
}
