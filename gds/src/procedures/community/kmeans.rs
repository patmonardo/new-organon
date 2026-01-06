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

use crate::core::utils::progress::{TaskRegistry, Tasks};
use crate::procedures::builder_base::ConfigValidator;
use crate::procedures::traits::Result;
use crate::algo::kmeans::{KMeansComputationRuntime, KMeansConfig, KMeansResult, KMeansStorageRuntime};
pub use crate::algo::kmeans::KMeansSamplerType;
use crate::concurrency::TerminationFlag;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct KMeansRow {
    pub node_id: u64,
    pub community_id: u64,
    pub distance_from_center: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct KMeansStats {
    pub k: usize,
    pub community_count: usize,
    pub average_distance_to_centroid: f64,
    pub average_silhouette: f64,
    pub ran_iterations: u32,
    pub restarts: u32,
    pub execution_time_ms: u64,
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

    fn compute(&self) -> Result<(KMeansResult, u64)> {
        self.validate_basic()?;
        let start = Instant::now();

        let config = self.config.clone();
        let node_count = self.graph_store.node_count();

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
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

        Ok((result, start.elapsed().as_millis() as u64))
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = KMeansRow>>> {
        let (result, _elapsed) = self.compute()?;
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
        let (result, elapsed) = self.compute()?;
        let community_count = result
            .communities
            .iter()
            .copied()
            .collect::<HashSet<u64>>()
            .len();

        Ok(KMeansStats {
            k: self.config.k,
            community_count,
            average_distance_to_centroid: result.average_distance_to_centroid,
            average_silhouette: result.average_silhouette,
            ran_iterations: result.ran_iterations,
            restarts: result.restarts,
            execution_time_ms: elapsed,
        })
    }

    pub fn run(&self) -> Result<KMeansResult> {
        let (result, _elapsed) = self.compute()?;
        Ok(result)
    }
}
