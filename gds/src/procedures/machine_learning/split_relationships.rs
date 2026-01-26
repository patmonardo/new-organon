//! SplitRelationships procedure facade.
//!
//! This wires ML split relationships logic through the procedures layer,
//! keeping applications decoupled from algo modules.

use crate::algo::algorithms::ConfigValidator;
use crate::algo::algorithms::Result;
use crate::concurrency::Concurrency;
use crate::core::graph_dimensions::ConcreteGraphDimensions;
use crate::mem::{MemoryEstimation, MemoryRange};
use crate::ml::splitting::{
    InMemoryRelationshipsBuilderFactory, SplitRelationships, SplitRelationshipsConfig,
    SplitRelationshipsEstimateDefinition, SplitRelationshipsParameters,
};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::RelationshipType;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SplitRelationshipsStats {
    pub selected_rel_count: usize,
    pub remaining_rel_count: usize,
}

/// SplitRelationships procedure facade.
pub struct SplitRelationshipsFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: SplitRelationshipsConfig,
    concurrency: Concurrency,
}

impl SplitRelationshipsFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: SplitRelationshipsConfig::default(),
            concurrency: Concurrency::of(1),
        }
    }

    pub fn config(mut self, config: SplitRelationshipsConfig) -> Self {
        self.config = config;
        self
    }

    pub fn concurrency(mut self, concurrency: Concurrency) -> Self {
        self.concurrency = concurrency;
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.config.holdout_fraction, 0.0, 1.0, "holdoutFraction")?;
        ConfigValidator::in_range(
            self.config.negative_sampling_ratio,
            0.0,
            1_000_000.0,
            "negativeSamplingRatio",
        )?;
        Ok(())
    }

    fn parameters(&self) -> SplitRelationshipsParameters {
        self.config.to_parameters(self.concurrency)
    }

    fn relationship_types(&self) -> Option<HashSet<RelationshipType>> {
        if self.config.relationship_types.is_empty() {
            None
        } else {
            Some(
                self.config
                    .relationship_types
                    .iter()
                    .cloned()
                    .map(RelationshipType::of)
                    .collect(),
            )
        }
    }

    pub fn compute(&self) -> Result<SplitRelationshipsStats> {
        self.validate()?;

        let graph = match self.relationship_types() {
            Some(rel_types) => self
                .graph_store
                .get_graph_with_types(&rel_types)
                .map_err(|e| AlgorithmError::Execution(e.to_string()))?,
            None => self.graph_store.get_graph(),
        };

        let master_graph = self.graph_store.get_graph();
        let root_nodes = self.graph_store.nodes();
        let source_nodes = self.graph_store.nodes();
        let target_nodes = self.graph_store.nodes();

        let builder_factory = Arc::new(InMemoryRelationshipsBuilderFactory::default());
        let splitter = SplitRelationships::new(
            graph,
            master_graph,
            root_nodes,
            source_nodes,
            target_nodes,
            self.parameters(),
            builder_factory,
        );

        let result = splitter.compute();

        Ok(SplitRelationshipsStats {
            selected_rel_count: result.selected_rel_count,
            remaining_rel_count: result.remaining_rel_count,
        })
    }

    pub fn estimate_memory(&self) -> Result<MemoryRange> {
        self.validate()?;
        let dims = ConcreteGraphDimensions::new(
            GraphStore::node_count(self.graph_store.as_ref()),
            GraphStore::relationship_count(self.graph_store.as_ref()),
        );
        let estimate =
            SplitRelationshipsEstimateDefinition::new(self.config.to_estimate_parameters());
        let tree = estimate.estimate(&dims, self.concurrency.value());
        Ok(*tree.memory_usage())
    }
}
