//! K-Means Facade
//!
//! Clusters nodes based on an array-valued node property (feature vector).
//!
//! Parameters (Neo4j GDS aligned):
//! - `k`
//! - `max_iterations`
//! - `delta_threshold`
//! - `number_of_restarts`
//! - `compute_silhouette`
//! - `concurrency`: accepted for parity; current runtime is single-threaded.
//! - `node_property`
//! - `sampler_type` (UNIFORM, KMEANSPP)
//! - `seed_centroids`
//! - `random_seed`

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
pub use crate::algo::kmeans::KMeansSamplerType;
use crate::algo::kmeans::{
    KMeansComputationRuntime, KMeansConfig, KMeansMutateResult, KMeansMutationSummary,
    KMeansResult, KMeansResultBuilder, KMeansStats, KMeansStorageRuntime,
};
use crate::collections::backends::vec::VecLong;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{TaskProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultLongNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct KMeansRow {
    pub node_id: u64,
    pub community_id: u64,
    pub distance_from_center: f64,
}

#[derive(Clone)]
pub struct KMeansFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: KMeansConfig,
    task_registry: Option<TaskRegistry>,
}

impl KMeansFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: KMeansConfig {
                concurrency: 4,
                ..KMeansConfig::default()
            },
            task_registry: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: KMeansConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            task_registry: None,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: KMeansConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: KMeansConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    pub fn k(mut self, k: usize) -> Self {
        self.config.k = k;
        self
    }

    pub fn max_iterations(mut self, max_iterations: u32) -> Self {
        self.config.max_iterations = max_iterations;
        self
    }

    pub fn delta_threshold(mut self, delta_threshold: f64) -> Self {
        self.config.delta_threshold = delta_threshold;
        self
    }

    pub fn number_of_restarts(mut self, number_of_restarts: u32) -> Self {
        self.config.number_of_restarts = number_of_restarts;
        self
    }

    pub fn compute_silhouette(mut self, enabled: bool) -> Self {
        self.config.compute_silhouette = enabled;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    pub fn node_property(mut self, node_property: &str) -> Self {
        self.config.node_property = node_property.to_string();
        self
    }

    pub fn sampler_type(mut self, sampler_type: KMeansSamplerType) -> Self {
        self.config.sampler_type = sampler_type;
        self
    }

    pub fn seed_centroids(mut self, seed_centroids: Vec<Vec<f64>>) -> Self {
        self.config.seed_centroids = seed_centroids;
        self
    }

    pub fn random_seed(mut self, seed: u64) -> Self {
        self.config.random_seed = Some(seed);
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn validate_basic(&self) -> Result<()> {
        ConfigValidator::in_range(
            self.config.concurrency as f64,
            1.0,
            1_000_000.0,
            "concurrency",
        )?;
        ConfigValidator::in_range(self.config.k as f64, 1.0, 1_000_000.0, "k")?;
        ConfigValidator::in_range(
            self.config.max_iterations as f64,
            1.0,
            1_000_000_000.0,
            "max_iterations",
        )?;
        ConfigValidator::in_range(
            self.config.number_of_restarts as f64,
            1.0,
            1_000_000_000.0,
            "number_of_restarts",
        )?;
        ConfigValidator::in_range(self.config.delta_threshold, 0.0, 1.0, "delta_threshold")?;
        ConfigValidator::non_empty_string(&self.config.node_property, "node_property")?;
        Ok(())
    }

    fn compute(&self) -> Result<KMeansResult> {
        self.validate_basic()?;
        let start = Instant::now();

        let config = self.config.clone();
        let node_count = self.graph_store.node_count();

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("kmeans".to_string(), node_count),
            config.concurrency,
        );

        let termination_flag = TerminationFlag::default();
        let storage = KMeansStorageRuntime::new();
        let mut runtime = KMeansComputationRuntime::new();

        let result = storage.compute_kmeans(
            &mut runtime,
            self.graph_store.as_ref(),
            &config,
            &mut progress_tracker,
            &termination_flag,
        )?;

        Ok(KMeansResult {
            execution_time: start.elapsed(),
            ..result
        })
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = KMeansRow>>> {
        let result = self.compute()?;
        let iter = result
            .communities
            .into_iter()
            .zip(result.distance_from_center)
            .enumerate()
            .map(
                |(node_id, (community_id, distance_from_center))| KMeansRow {
                    node_id: node_id as u64,
                    community_id,
                    distance_from_center,
                },
            );
        Ok(Box::new(iter))
    }

    pub fn stats(&self) -> Result<KMeansStats> {
        let result = self.compute()?;
        Ok(KMeansResultBuilder::new(result).stats())
    }

    /// Mutate mode: writes community assignments back to the graph store.
    pub fn mutate(self, property_name: &str) -> Result<KMeansMutateResult> {
        self.validate_basic()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        let longs: Vec<i64> = result.communities.into_iter().map(|c| c as i64).collect();
        let backend = VecLong::from(longs);
        let values = DefaultLongNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels_set: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels_set, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("KMeans mutate failed to add property: {e}"))
            })?;

        let summary = KMeansMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(KMeansMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: writes community assignments to a new graph.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    /// Estimate memory usage.
    pub fn estimate_memory(&self) -> Result<MemoryRange> {
        self.validate_basic()?;

        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();

        // Per node: feature vector + community assignments + distance tracking.
        let per_node = 192usize;
        // Per relationship: traversal overhead (small).
        let per_relationship = 8usize;

        let base: usize = 64 * 1024;
        let total = base
            .saturating_add(node_count.saturating_mul(per_node))
            .saturating_add(relationship_count.saturating_mul(per_relationship));

        Ok(MemoryRange::of_range(total, total.saturating_mul(3)))
    }

    pub fn run(&self) -> Result<KMeansResult> {
        let result = self.compute()?;
        Ok(result)
    }
}
