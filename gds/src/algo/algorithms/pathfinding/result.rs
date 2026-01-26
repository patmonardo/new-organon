use crate::algo::algorithms::{
    ExecutionMetadata, ResultBuilder, ResultBuilderError,
};
use crate::algo::algorithms::statistics::{
    Histogram, StatisticalSummary, StatisticsConfig, StatisticsEngine,
};
use crate::collections::backends::vec::VecDouble;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::RelationshipType;
use crate::types::graph_store::GraphStore;
use crate::types::prelude::DefaultGraphStore;
use crate::types::properties::relationship::DefaultDoubleRelationshipPropertyValues;
use crate::types::properties::relationship::RelationshipPropertyValues;
use crate::types::schema::Direction;
use std::sync::Arc;
use std::time::Duration;

/// Path finding algorithm result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PathFindingResult {
    /// Paths found during execution
    pub paths: Vec<PathResult>,
    /// Statistical summary
    pub statistics: Option<StatisticalSummary>,
    /// Histogram of path costs
    pub histogram: Option<Histogram>,
    /// Execution metadata
    pub metadata: ExecutionMetadata,
    /// Post-processing time in milliseconds
    pub post_processing_millis: i64,
}

/// Path finding result builder (family-level alias).
///
/// Prefer this name for pathfinding algorithms (e.g., Bellman-Ford, Dijkstra).
pub type PathFindingResultBuilder = PathResultBuilder;

/// Path result for individual paths
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PathResult {
    pub source: u64,
    pub target: u64,
    pub path: Vec<u64>,
    pub cost: f64,
}

/// Path finding result builder
///
/// **Translation**: `AbstractPathFindingResultBuilder` (inferred from Java patterns)
///
/// Provides result building for path finding algorithms with histogram computation
/// and path statistics.
pub struct PathResultBuilder {
    paths: Option<Vec<PathResult>>,
    statistics: Option<StatisticalSummary>,
    histogram: Option<Histogram>,
    metadata: Option<ExecutionMetadata>,
    compute_statistics: bool,
    compute_histogram: bool,
    post_processing_millis: i64,
}

impl PathResultBuilder {
    /// Create a new path result builder
    pub fn new() -> Self {
        Self {
            paths: None,
            statistics: None,
            histogram: None,
            metadata: None,
            compute_statistics: true,
            compute_histogram: true,
            post_processing_millis: -1,
        }
    }

    /// Set pre-computed paths
    pub fn with_paths(mut self, paths: Vec<PathResult>) -> Self {
        self.paths = Some(paths);
        self
    }

    /// Enable or disable statistics computation
    pub fn with_statistics(mut self, compute: bool) -> Self {
        self.compute_statistics = compute;
        self
    }

    /// Enable or disable histogram computation
    pub fn with_histogram(mut self, compute: bool) -> Self {
        self.compute_histogram = compute;
        self
    }

    /// Set execution metadata
    pub fn with_metadata(mut self, metadata: ExecutionMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl ResultBuilder<PathFindingResult> for PathResultBuilder {
    fn build(mut self) -> Result<PathFindingResult, ResultBuilderError> {
        use std::time::Instant;

        let post_processing_start = Instant::now();

        // Get paths
        let paths = self
            .paths
            .ok_or_else(|| ResultBuilderError::MissingData("No paths provided".to_string()))?;

        // Compute statistics if requested and not already provided
        let mut statistics = self.statistics;
        let mut histogram = self.histogram;

        if self.compute_statistics && statistics.is_none() {
            // Compute path cost statistics
            let costs: Vec<f64> = paths.iter().map(|p| p.cost).collect();
            let config = StatisticsConfig {
                compute_histogram: self.compute_histogram,
                ..Default::default()
            };

            let (stats, hist) = StatisticsEngine::compute_statistics_from_values(costs, config)?;

            statistics = Some(stats);
            if self.compute_histogram {
                histogram = hist;
            }
        }

        self.post_processing_millis = post_processing_start.elapsed().as_millis() as i64;

        let metadata = self
            .metadata
            .unwrap_or_else(|| ExecutionMetadata::new(Duration::from_secs(0)));

        Ok(PathFindingResult {
            paths,
            statistics,
            histogram,
            metadata,
            post_processing_millis: self.post_processing_millis,
        })
    }

    fn with_statistics(mut self, stats: StatisticalSummary) -> Self {
        self.statistics = Some(stats);
        self
    }

    fn with_histogram(mut self, hist: Option<Histogram>) -> Self {
        self.histogram = hist;
        self
    }

    fn with_metadata(mut self, metadata: ExecutionMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl Default for PathResultBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub fn build_path_relationship_store(
    graph_store: &DefaultGraphStore,
    relationship_type: &str,
    paths: &[PathResult],
) -> Result<Arc<DefaultGraphStore>, AlgorithmError> {
    let node_count = graph_store.node_count();
    let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
    let mut costs_by_source: Vec<Vec<f64>> = vec![Vec::new(); node_count];

    for path in paths {
        let source = path.source as usize;
        let target = path.target as usize;
        if source >= node_count || target >= node_count {
            continue;
        }
        outgoing[source].push(path.target as i64);
        costs_by_source[source].push(path.cost);
    }

    let rel_type = RelationshipType::of(relationship_type);

    let mut updated = graph_store
        .with_added_relationship_type_preserve_name(rel_type.clone(), outgoing, Direction::Directed)
        .map_err(|e| AlgorithmError::Execution(format!("Failed to add path relationships: {e}")))?;

    let mut flat_costs: Vec<f64> = Vec::new();
    for costs in costs_by_source {
        flat_costs.extend(costs);
    }

    let pv: Arc<dyn RelationshipPropertyValues> = Arc::new(
        DefaultDoubleRelationshipPropertyValues::<VecDouble>::from_collection(
            VecDouble::from(flat_costs),
            updated.relationship_count(),
        ),
    );

    updated
        .add_relationship_property(rel_type, "totalCost".to_string(), pv)
        .map_err(|e| {
            AlgorithmError::Execution(format!(
                "Failed to add totalCost relationship property: {e}"
            ))
        })?;

    Ok(Arc::new(updated))
}
