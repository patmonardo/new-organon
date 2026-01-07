//! Database-backed graph store estimation service.
//!
//! Java parity target: this mirrors the role of Neo4j GDS' database-backed
//! graph store estimation service (i.e., estimating a projection against a
//! live database / projection config).
//!
//! In this repo, the graph-store loading layer lives under
//! `crate::applications::graph_store_catalog::loaders`. This service bridges
//! into that layer and adapts it to the `memest` API surface.

use super::GraphMemoryEstimation;
use crate::applications::graph_store_catalog::loaders::{
    GraphLoaderContext, GraphProjectConfig, GraphStoreCreator, GraphStoreFromDatabaseLoader,
    GraphStoreLoader,
};
use crate::core::graph_dimensions::ConcreteGraphDimensions;

/// Service for estimating memory usage for database-backed graph projections.
///
/// This service is intentionally thin: it delegates to the graph-store loader
/// layer and returns a `GraphMemoryEstimation` that contains a deferred
/// `MemoryEstimation`.
pub struct DatabaseGraphEstimationService {
    graph_loader_context: GraphLoaderContext,
    username: String,
}

impl DatabaseGraphEstimationService {
    /// Creates a new service.
    pub fn new(graph_loader_context: GraphLoaderContext, username: String) -> Self {
        Self {
            graph_loader_context,
            username,
        }
    }

    /// Estimates memory usage for a graph projection.
    pub fn estimate(
        &self,
        graph_project_config: Box<dyn GraphProjectConfig>,
    ) -> GraphMemoryEstimation {
        let loader = GraphStoreFromDatabaseLoader::new(
            graph_project_config,
            self.username.clone(),
            self.graph_loader_context.clone(),
        );

        let dimensions = loader.graph_dimensions();
        let concrete_dimensions = ConcreteGraphDimensions::of(
            dimensions.node_count(),
            dimensions.rel_count_upper_bound(),
        );

        GraphMemoryEstimation::new(
            concrete_dimensions,
            loader.estimate_memory_usage_after_loading(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::applications::graph_store_catalog::loaders::TransactionContext;
    use crate::concurrency::TerminationFlag;
    use crate::core::graph_dimensions::GraphDimensions;
    use crate::types::graph_store::DatabaseId;

    #[derive(Clone, Debug)]
    struct TestProjectConfig {
        graph_name: String,
        username: String,
    }

    impl GraphProjectConfig for TestProjectConfig {
        fn graph_name(&self) -> &str {
            &self.graph_name
        }

        fn username(&self) -> &str {
            &self.username
        }
    }

    #[test]
    fn estimate_returns_deferred_estimation_and_dimensions() {
        let ctx = GraphLoaderContext::new(
            DatabaseId::new("test-db"),
            TerminationFlag::default(),
            TransactionContext::new("tx".to_string()),
        );

        let service = DatabaseGraphEstimationService::new(ctx, "alice".to_string());
        let estimation = service.estimate(Box::new(TestProjectConfig {
            graph_name: "g".to_string(),
            username: "alice".to_string(),
        }));

        assert!(estimation.dimensions().node_count() > 0);
        assert!(estimation.dimensions().rel_count_upper_bound() > 0);

        let tree = estimation
            .estimate_memory_usage_after_loading()
            .estimate(estimation.dimensions(), 4);
        assert!(tree.memory_usage().min() > 0);
    }
}
