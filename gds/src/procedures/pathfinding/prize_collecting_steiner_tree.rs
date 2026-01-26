use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::prize_collecting_steiner_tree::{
    PCSTreeComputationRuntime, PCSTreeConfig, PCSTreeMutateResult, PCSTreeResult,
    PCSTreeResultBuilder, PCSTreeRow, PCSTreeStats, PCSTreeStorageRuntime,
};
use crate::mem::MemoryRange;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

// Import upgraded systems
use crate::algo::algorithms::pathfinding::PathResult;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::{TaskRegistryFactory, Tasks};
use crate::projection::eval::algorithm::AlgorithmError;

/// Prize-Collecting Steiner Tree algorithm builder
pub struct PCSTreeBuilder {
    graph_store: Arc<DefaultGraphStore>,
    prizes: Vec<f64>,
    relationship_weight_property: Option<String>,
    concurrency: usize,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

impl PCSTreeBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            prizes: Vec::new(),
            relationship_weight_property: None,
            concurrency: 4,
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Set the prizes for each node
    ///
    /// `prizes[i]` is the value gained by including node `i` in the tree.
    /// The algorithm seeks to maximize: sum(prizes) - sum(edge_costs)
    ///
    /// Must be provided and length must match node count at execution time.
    pub fn prizes(mut self, prizes: Vec<f64>) -> Self {
        self.prizes = prizes;
        self
    }

    /// Set the relationship weight property name
    pub fn relationship_weight_property(mut self, property: &str) -> Self {
        self.relationship_weight_property = Some(property.to_string());
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// PCST benefits from parallelism in large graphs.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Set task registry factory for progress tracking
    pub fn task_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.task_registry_factory = Some(factory);
        self
    }

    /// Set user log registry factory for progress tracking
    pub fn user_log_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.user_log_registry_factory = Some(factory);
        self
    }
    fn validate(&self) -> Result<()> {
        if self.prizes.is_empty() {
            return Err(AlgorithmError::Execution(
                "prizes must be provided and non-empty".to_string(),
            ));
        }

        if self.concurrency == 0 {
            return Err(AlgorithmError::Execution(
                "concurrency must be > 0".to_string(),
            ));
        }

        Ok(())
    }

    fn compute(self) -> Result<(PCSTreeResult, std::time::Duration)> {
        self.validate()?;

        let start = std::time::Instant::now();

        // PCST works on undirected graphs.
        // Use all relationship types by default.
        let rel_types: HashSet<RelationshipType> = self.graph_store.relationship_types();

        let graph_view = if let Some(prop) = self.relationship_weight_property.as_ref() {
            let selectors: HashMap<RelationshipType, String> = rel_types
                .iter()
                .map(|t| (t.clone(), prop.clone()))
                .collect();
            self.graph_store
                .get_graph_with_types_selectors_and_orientation(
                    &rel_types,
                    &selectors,
                    Orientation::Undirected,
                )
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        } else {
            self.graph_store
                .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        };

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((
                PCSTreeResult {
                    parent_array: Vec::new(),
                    relationship_to_parent_cost: Vec::new(),
                    total_edge_cost: 0.0,
                    total_prize: 0.0,
                    net_value: 0.0,
                    effective_node_count: 0,
                },
                start.elapsed(),
            ));
        }

        if self.prizes.len() != node_count {
            return Err(AlgorithmError::Execution(format!(
                "Prize vector length ({}) must match node count ({})",
                self.prizes.len(),
                node_count
            )));
        }

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("prize_collecting_steiner_tree".to_string(), node_count),
            self.concurrency,
        );

        // Build config + runtimes.
        // Storage runtime owns graph access and drives the algorithm.
        let prizes = self.prizes;
        let config = PCSTreeConfig {
            prizes: prizes.clone(),
            relationship_weight_property: self.relationship_weight_property,
        };
        let storage = PCSTreeStorageRuntime::new(config, self.concurrency);
        let mut computation = PCSTreeComputationRuntime::new(prizes, node_count);
        let result = storage.compute_prize_collecting_steiner_tree(
            &mut computation,
            Some(graph_view.as_ref()),
            &mut progress_tracker,
        )?;
        Ok((result, start.elapsed()))
    }

    /// Stream mode: yields tree edges
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PCSTreeRow>>> {
        let (result, elapsed) = self.compute()?;
        let rows = PCSTreeResultBuilder::new(result, elapsed).rows();
        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: aggregated tree stats
    pub fn stats(self) -> Result<PCSTreeStats> {
        let (result, elapsed) = self.compute()?;
        Ok(PCSTreeResultBuilder::new(result, elapsed).stats())
    }

    /// Mutate mode: writes results back to the graph store
    pub fn mutate(self, property_name: &str) -> Result<PCSTreeMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let graph_store = Arc::clone(&self.graph_store);
        let (result, elapsed) = self.compute()?;
        let builder = PCSTreeResultBuilder::new(result, elapsed);
        let paths: Vec<PathResult> = builder.paths();

        let updated_store = crate::algo::algorithms::pathfinding::build_path_relationship_store(
            graph_store.as_ref(),
            property_name,
            &paths,
        )?;

        let summary = builder.mutation_summary(property_name, paths.len() as u64);

        Ok(PCSTreeMutateResult {
            summary,
            updated_store,
        })
    }

    /// Write mode: writes results to external storage
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    /// Estimate memory usage for the computation
    pub fn estimate_memory(&self) -> Result<MemoryRange> {
        // Estimate based on node count and expected tree structure
        let node_count = self.graph_store.node_count();
        let estimated_bytes = node_count * std::mem::size_of::<f64>() * 4; // prizes, costs, parents, etc.
        Ok(MemoryRange::of_range(
            estimated_bytes / 2,
            estimated_bytes * 2,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(13),
            node_count: 10,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_pcst_builder() {
        let store = store();
        let builder = PCSTreeBuilder::new(store).prizes(vec![0.0, 5.0, 10.0, 3.0]);

        // Builder creates correct prizes
        assert_eq!(builder.prizes, vec![0.0, 5.0, 10.0, 3.0]);
    }

    #[test]
    fn test_pcst_high_prize() {
        let store = store();
        let prizes = vec![0.0, 100.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]; // Node 1 has huge prize

        let result = PCSTreeBuilder::new(store).prizes(prizes).stats().unwrap();

        // Should have positive net value (high prize from node 1)
        assert!(result.net_value > 0.0); // Positive net value
        assert!(result.total_prize > 0.0); // Collected prizes
    }

    #[test]
    fn test_pcst_wrong_prize_count() {
        let store = store();
        let prizes = vec![1.0]; // Wrong number of prizes

        let result = PCSTreeBuilder::new(store).prizes(prizes).stats();

        // Should fail validation
        assert!(result.is_err());
    }
}
