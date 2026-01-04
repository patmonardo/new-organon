use super::{GraphProjectConfig, GraphStoreCreator, GraphStoreLoader, ResultStore};
use crate::concurrency::TerminationFlag;
use crate::core::{ConcreteGraphDimensions, GraphDimensions};
use crate::mem::{MemoryEstimation, MemoryRange, MemoryTree};
use crate::types::graph_store::{DatabaseId, DefaultGraphStore};
use crate::types::random::{RandomGraphConfig, Randomizable};
use rand::rngs::StdRng;
use rand::SeedableRng;

/// Implementation of GraphStoreCreator for loading from database.
///
/// Mirrors Java GraphStoreFromDatabaseLoader class.
/// Loads graph stores directly from the database using GraphStoreFactory.
pub struct GraphStoreFromDatabaseLoader {
    graph_project_config: Box<dyn GraphProjectConfig>,
    username: String,
    #[allow(dead_code)]
    graph_loader_context: GraphLoaderContext,
    graph_store_factory: Box<dyn GraphStoreFactory>,
}

impl GraphStoreFromDatabaseLoader {
    /// Creates a new GraphStoreFromDatabaseLoader.
    pub fn new(
        graph_project_config: Box<dyn GraphProjectConfig>,
        username: String,
        graph_loader_context: GraphLoaderContext,
    ) -> Self {
        let graph_store_factory = Box::new(DatabaseGraphStoreFactory::new());

        Self {
            graph_project_config,
            username,
            graph_loader_context,
            graph_store_factory,
        }
    }
}

impl GraphStoreLoader for GraphStoreFromDatabaseLoader {
    fn graph_project_config(&self) -> Box<dyn GraphProjectConfig> {
        Box::new(DatabaseGraphProjectConfig::new(
            self.graph_project_config.graph_name().to_string(),
            self.username.clone(),
        ))
    }

    fn graph_store(&self) -> DefaultGraphStore {
        self.graph_store_factory.build()
    }

    fn result_store(&self) -> Box<dyn ResultStore> {
        Box::new(DatabaseResultStore::new())
    }

    fn graph_dimensions(&self) -> Box<dyn GraphDimensions> {
        self.graph_store_factory.estimation_dimensions()
    }
}

impl GraphStoreCreator for GraphStoreFromDatabaseLoader {
    fn estimate_memory_usage_during_loading(&self) -> Box<dyn MemoryEstimation> {
        self.graph_store_factory
            .estimate_memory_usage_during_loading()
    }

    fn estimate_memory_usage_after_loading(&self) -> Box<dyn MemoryEstimation> {
        self.graph_store_factory
            .estimate_memory_usage_after_loading()
    }
}

// Placeholder types for database operations

#[derive(Clone)]
#[allow(dead_code)]
pub struct GraphLoaderContext {
    database_id: DatabaseId,
    termination_flag: TerminationFlag,
    transaction_context: TransactionContext,
}

impl GraphLoaderContext {
    pub fn new(
        database_id: DatabaseId,
        termination_flag: TerminationFlag,
        transaction_context: TransactionContext,
    ) -> Self {
        Self {
            database_id,
            termination_flag,
            transaction_context,
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TransactionContext {
    transaction_id: String,
}

impl TransactionContext {
    pub fn new(transaction_id: String) -> Self {
        Self { transaction_id }
    }
}

pub trait GraphStoreFactory {
    fn build(&self) -> DefaultGraphStore;
    fn estimation_dimensions(&self) -> Box<dyn GraphDimensions>;
    fn estimate_memory_usage_during_loading(&self) -> Box<dyn MemoryEstimation>;
    fn estimate_memory_usage_after_loading(&self) -> Box<dyn MemoryEstimation>;
}

#[derive(Clone, Debug)]
struct DatabaseGraphStoreFactory {
    node_count: u64,
    relationship_count: u64,
}

impl DatabaseGraphStoreFactory {
    fn new() -> Self {
        Self {
            node_count: 5000,
            relationship_count: 25000,
        }
    }
}

impl GraphStoreFactory for DatabaseGraphStoreFactory {
    fn build(&self) -> DefaultGraphStore {
        let config = RandomGraphConfig {
            graph_name: "database-graph".to_string(),
            database_name: "database".to_string(),
            node_count: self.node_count as usize,
            ..Default::default()
        };

        let mut rng = StdRng::seed_from_u64(0);
        DefaultGraphStore::random_with_rng(&config, &mut rng)
            .expect("database graph store generation should succeed")
    }

    fn estimation_dimensions(&self) -> Box<dyn GraphDimensions> {
        Box::new(ConcreteGraphDimensions::new(
            self.node_count as usize,
            self.relationship_count as usize,
        ))
    }

    fn estimate_memory_usage_during_loading(&self) -> Box<dyn MemoryEstimation> {
        Box::new(DatabaseMemoryEstimation::new())
    }

    fn estimate_memory_usage_after_loading(&self) -> Box<dyn MemoryEstimation> {
        Box::new(DatabaseMemoryEstimation::new())
    }
}

#[derive(Clone, Debug)]
struct DatabaseResultStore {
    is_empty: bool,
}

impl DatabaseResultStore {
    fn new() -> Self {
        Self { is_empty: false }
    }
}

impl ResultStore for DatabaseResultStore {
    fn is_empty(&self) -> bool {
        self.is_empty
    }
}

#[derive(Clone, Debug)]
struct DatabaseGraphProjectConfig {
    graph_name: String,
    username: String,
}

impl DatabaseGraphProjectConfig {
    fn new(graph_name: String, username: String) -> Self {
        Self {
            graph_name,
            username,
        }
    }
}

impl GraphProjectConfig for DatabaseGraphProjectConfig {
    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    fn username(&self) -> &str {
        &self.username
    }
}

#[derive(Clone, Debug)]
struct DatabaseMemoryEstimation {
    estimated_bytes: u64,
}

impl DatabaseMemoryEstimation {
    fn new() -> Self {
        Self {
            estimated_bytes: 5 * 1024 * 1024, // 5MB
        }
    }
}

impl MemoryEstimation for DatabaseMemoryEstimation {
    fn description(&self) -> String {
        "Database graph store memory estimate".to_string()
    }

    fn estimate(&self, _dimensions: &dyn GraphDimensions, _concurrency: usize) -> MemoryTree {
        let bytes: usize = self
            .estimated_bytes
            .try_into()
            .expect("estimated_bytes should fit into usize");

        MemoryTree::leaf(self.description(), MemoryRange::of(bytes))
    }
}
