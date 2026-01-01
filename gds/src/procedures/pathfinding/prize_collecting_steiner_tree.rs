use crate::mem::MemoryRange;
use crate::procedures::builder_base::{MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::algo::prize_collecting_steiner_tree::computation::PCSTreeComputationRuntime;
use crate::algo::prize_collecting_steiner_tree::PCSTreeConfig;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;

// Import upgraded systems
use crate::core::utils::progress::TaskRegistryFactory;

/// Result row for Prize-Collecting Steiner Tree stream mode
#[derive(Debug, Clone, serde::Serialize)]
pub struct PCSTreeRow {
    pub node: u64,
    pub parent: Option<u64>,
    pub cost_to_parent: f64,
}

/// Statistics for Prize-Collecting Steiner Tree computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct PCSTreeStats {
    pub node_count: usize,
    pub total_prize: f64,
    pub total_cost: f64,
    pub net_value: f64,
    pub computation_time_ms: u64,
}

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
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "prizes must be provided and non-empty".to_string(),
                ),
            );
        }

        if self.concurrency == 0 {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "concurrency must be > 0".to_string(),
                ),
            );
        }

        Ok(())
    }

    fn compute(self) -> Result<(Vec<PCSTreeRow>, PCSTreeStats)> {
        self.validate()?;

        let start = std::time::Instant::now();

        // PCST works on undirected graphs
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((
                Vec::new(),
                PCSTreeStats {
                    node_count: 0,
                    total_prize: 0.0,
                    total_cost: 0.0,
                    net_value: 0.0,
                    computation_time_ms: start.elapsed().as_millis() as u64,
                },
            ));
        }

        if self.prizes.len() != node_count {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                    "Prize vector length ({}) must match node count ({})",
                    self.prizes.len(),
                    node_count
                )),
            );
        }

        let fallback = graph_view.default_property_value();

        // Get neighbors with weights
        let get_neighbors = |node_idx: usize| -> Vec<(usize, f64)> {
            graph_view
                .stream_relationships(node_idx as i64, fallback)
                .filter_map(|cursor| {
                    let target = cursor.target_id();
                    if target >= 0 {
                        let weight = cursor.property();
                        Some((target as usize, weight))
                    } else {
                        None
                    }
                })
                .collect()
        };

        let config = PCSTreeConfig {
            prizes: self.prizes,
            relationship_weight_property: self.relationship_weight_property,
        };

        let runtime = PCSTreeComputationRuntime::new(config);
        let result = runtime.compute(node_count, get_neighbors);

        let mut rows = Vec::new();
        for (node_idx, &parent) in result.parent_array.iter().enumerate() {
            if parent >= 0 {
                rows.push(PCSTreeRow {
                    node: node_idx as u64,
                    parent: Some(parent as u64),
                    cost_to_parent: result.relationship_to_parent_cost[node_idx],
                });
            } else if parent == -1 {
                // Root node
                rows.push(PCSTreeRow {
                    node: node_idx as u64,
                    parent: None,
                    cost_to_parent: 0.0,
                });
            }
            // Skip nodes not in tree (parent == -2)
        }

        let stats = PCSTreeStats {
            node_count,
            total_prize: result.total_prize,
            total_cost: result.total_edge_cost,
            net_value: result.net_value,
            computation_time_ms: start.elapsed().as_millis() as u64,
        };

        Ok((rows, stats))
    }

    /// Stream mode: yields tree edges
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PCSTreeRow>>> {
        let (rows, _) = self.compute()?;
        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: aggregated tree stats
    pub fn stats(self) -> Result<PCSTreeStats> {
        let (_, stats) = self.compute()?;
        Ok(stats)
    }

    /// Mutate mode: writes results back to the graph store
    pub fn mutate(self) -> Result<MutationResult> {
        // TODO: Implement mutation logic
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "mutate mode not yet implemented".to_string(),
            ),
        )
    }

    /// Write mode: writes results to external storage
    pub fn write(self) -> Result<WriteResult> {
        // TODO: Implement write logic
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "write mode not yet implemented".to_string(),
            ),
        )
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
