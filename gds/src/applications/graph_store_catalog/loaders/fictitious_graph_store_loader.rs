use super::{GraphProjectConfig, GraphStoreCreator, GraphStoreLoader, ResultStore};
use crate::core::{ConcreteGraphDimensions, GraphDimensions};
use crate::mem::{MemoryEstimation, MemoryRange, MemoryTree};
use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore as _;
use crate::types::random::{RandomGraphConfig, Randomizable};
use rand::rngs::StdRng;
use rand::SeedableRng;

/// Implementation of GraphStoreCreator for fictitious/testing scenarios.
///
/// Mirrors Java FictitiousGraphStoreLoader class.
/// Implementation for testing that doesn't actually load real data.
pub struct FictitiousGraphStoreLoader {
    graph_project_config: Box<dyn GraphProjectConfig>,
    #[allow(dead_code)]
    result_store: Box<dyn ResultStore>,
}

impl FictitiousGraphStoreLoader {
    /// Creates a new FictitiousGraphStoreLoader.
    pub fn new(graph_project_config: Box<dyn GraphProjectConfig>) -> Self {
        let result_store = Box::new(FictitiousResultStore::new());

        Self {
            graph_project_config,
            result_store,
        }
    }

    fn generate_store(&self) -> DefaultGraphStore {
        let config = RandomGraphConfig {
            graph_name: self.graph_project_config.graph_name().to_string(),
            database_name: "fictitious".to_string(),
            ..Default::default()
        };

        let mut rng = StdRng::seed_from_u64(0);
        DefaultGraphStore::random_with_rng(&config, &mut rng)
            .expect("fictitious default graph store generation should succeed")
    }
}

impl GraphStoreLoader for FictitiousGraphStoreLoader {
    fn graph_project_config(&self) -> Box<dyn GraphProjectConfig> {
        // Clone the config - in real implementation would have proper cloning
        Box::new(FictitiousGraphProjectConfig::new(
            self.graph_project_config.graph_name().to_string(),
            self.graph_project_config.username().to_string(),
        ))
    }

    fn graph_store(&self) -> DefaultGraphStore {
        self.generate_store()
    }

    fn result_store(&self) -> Box<dyn ResultStore> {
        Box::new(FictitiousResultStore::new())
    }

    fn graph_dimensions(&self) -> Box<dyn GraphDimensions> {
        let store = self.generate_store();
        Box::new(ConcreteGraphDimensions::new(
            store.node_count(),
            store.relationship_count(),
        ))
    }
}

impl GraphStoreCreator for FictitiousGraphStoreLoader {
    fn estimate_memory_usage_during_loading(&self) -> Box<dyn MemoryEstimation> {
        Box::new(FictitiousMemoryEstimation::new())
    }

    fn estimate_memory_usage_after_loading(&self) -> Box<dyn MemoryEstimation> {
        Box::new(FictitiousMemoryEstimation::new())
    }
}

// Fictitious implementations for testing

#[derive(Clone, Debug)]
struct FictitiousGraphProjectConfig {
    graph_name: String,
    username: String,
}

impl FictitiousGraphProjectConfig {
    fn new(graph_name: String, username: String) -> Self {
        Self {
            graph_name,
            username,
        }
    }
}

impl GraphProjectConfig for FictitiousGraphProjectConfig {
    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    fn username(&self) -> &str {
        &self.username
    }
}

#[derive(Clone, Debug)]
struct FictitiousResultStore {
    is_empty: bool,
}

impl FictitiousResultStore {
    fn new() -> Self {
        Self { is_empty: true }
    }
}

impl ResultStore for FictitiousResultStore {
    fn is_empty(&self) -> bool {
        self.is_empty
    }
}

#[derive(Clone, Debug)]
struct FictitiousMemoryEstimation {
    estimated_bytes: u64,
}

impl FictitiousMemoryEstimation {
    fn new() -> Self {
        Self {
            estimated_bytes: 1024 * 1024, // 1MB
        }
    }
}

impl MemoryEstimation for FictitiousMemoryEstimation {
    fn description(&self) -> String {
        "Fictitious graph store memory estimate".to_string()
    }

    fn estimate(&self, _dimensions: &dyn GraphDimensions, _concurrency: usize) -> MemoryTree {
        let bytes: usize = self
            .estimated_bytes
            .try_into()
            .expect("estimated_bytes should fit into usize");

        MemoryTree::leaf(self.description(), MemoryRange::of(bytes))
    }
}
