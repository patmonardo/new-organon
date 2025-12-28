//! Procedure Infrastructure - Algorithm Implementations and Catalog
//!
//! **Architecture Note**: This is NOT the executor runtime!
//! The executor runtime lives in `src/projection/eval/procedure/`.
//!
//! ## Key Distinction
//!
//! - **This module** (src/procedure) = Algorithm implementations (extensible content)
//! - **eval/procedure** = Executor runtime (fixed GDSL Runtime)
//!
//! Think of it this way:
//! - **This module** = WHAT to execute (PageRank, Louvain, etc.)
//! - **eval/procedure** = HOW to execute (orchestration machinery)
//!
//! ## What Lives Here
//!
//! This module contains the **Java GDS algo packages** translated to Rust:
//!
//! ```text
//! src/procedure/
//! ├── algo/               ← Algorithm implementations
//! │   ├── pagerank.rs
//! │   ├── louvain.rs
//! │   └── ...
//! ├── common/             ← Shared utilities (algo-common)
//! │   ├── convergence.rs
//! │   ├── tolerance.rs
//! │   └── ...
//! ├── params/             ← Parameter handling (algo-params)
//! └── specifications/     ← Algorithm catalog
//! ```
//!
//! ## The Pattern
//!
//! Each algorithm:
//! 1. **Implements** `AlgorithmSpec` trait (defined in eval/procedure)
//! 2. **Provides** specific computation logic
//! 3. **Registers** in the algorithm catalog
//!
//! Example:
//! ```rust,ignore
//! use crate::projection::eval::procedure::AlgorithmSpec;
//!
//! pub struct PageRankSpec { /* ... */ }
//!
//! impl AlgorithmSpec for PageRankSpec {
//!     // Implement the contract
//!     fn execute(&self, graph, config, context) -> Result<...> {
//!         // PageRank-specific logic here
//!     }
//! }
//! ```
//!
//! ## Relationship to Executor
//!
//! ```text
//! Executor Runtime (eval/procedure):
//!   1. Defines AlgorithmSpec trait (the contract)
//!   2. Provides orchestration (parse → validate → load → execute → consume)
//!   3. Integrates with TypeValidator + AdaptiveProjector
//!
//! Algorithm Implementations (this module):
//!   1. Implement AlgorithmSpec trait
//!   2. Provide specific computation logic
//!   3. Register in catalog
//! ```
//!
//! **The executor USES the algorithms through the trait.**
//!
//!
//! The eval layer RAISES this infrastructure into consciousness through:
//! - AlgorithmSpec (bridges machine to projectors)
//! - TypeValidator (validates forms)
//! - AdaptiveProjector (chooses optimal manifestations)
//!
//! ## Usage
//!
//! Most users will NOT use this module directly. Instead, use the eval/procedure
//! layer which provides projection-aware algorithm specifications.
//!
//! See: `src/projection/eval/procedure/` for the consciousness layer.
/// Core utilities from Java GDS algo-common
/// - Result builders and statistics (centrality, community, similarity)
/// - Feature scaling for ML pipelines
/// - Common algorithm utilities
pub mod core;

/// Algorithm infrastructure (Genera)
/// - Centrality algorithm utilities
/// - Community detection utilities
/// - Algorithm-specific result types and transformations
pub mod algorithms;

// Procedure Facades - User-facing idiomatic Rust API
pub mod facades;

// Module structure

pub mod all_shortest_paths;
pub mod approx_max_k_cut;
pub mod articulation_points;
pub mod astar;
pub mod bellman_ford;
pub mod betweenness;
pub mod bfs;
pub mod bridges;
pub mod celf;
pub mod closeness;
pub mod conductance;
pub mod dag_longest_path;
pub mod degree_centrality;
pub mod delta_stepping;
pub mod dfs;
pub mod dijkstra;
pub mod harmonic;
pub mod hits;
pub mod k1coloring;
pub mod kcore;
pub mod kmeans;
pub mod kspanningtree;
pub mod label_propagation;
pub mod leiden;
pub mod local_clustering_coefficient;
pub mod louvain;
pub mod modularity;
pub mod msbfs;
pub mod pagerank;
pub mod prize_collecting_steiner_tree;
pub mod random_walk;
pub mod scc;
pub mod similarity;
pub mod spanning_tree;
pub mod steiner_tree;
pub mod topological_sort;
pub mod traversal;
pub mod triangle_count;
pub mod wcc;
pub mod yens;
pub mod embeddings;

// Future modules (to be implemented)
// pub mod facade;      // Public API facades

// Re-export commonly used types
pub use all_shortest_paths::{
    ALL_SHORTEST_PATHSAlgorithmSpec, AllShortestPathsComputationRuntime, AllShortestPathsConfig,
    AllShortestPathsResult, AllShortestPathsStorageRuntime,
};
pub use articulation_points::{
    ArticulationPointsAlgorithmSpec, ArticulationPointsComputationRuntime,
    ArticulationPointsConfig, ArticulationPointsResult, ArticulationPointsStorageRuntime,
};
pub use astar::{
    ASTARAlgorithmSpec, AStarComputationRuntime, AStarConfig, AStarResult, AStarStorageRuntime,
};
pub use bellman_ford::{
    BELLMAN_FORDAlgorithmSpec, BellmanFordComputationRuntime, BellmanFordConfig, BellmanFordResult,
    BellmanFordStorageRuntime,
};
pub use betweenness::{
    BetweennessCentralityAlgorithmSpec, BetweennessCentralityComputationRuntime,
    BetweennessCentralityConfig, BetweennessCentralityResult, BetweennessCentralityStorageRuntime,
};
pub use bfs::{BFSAlgorithmSpec, BfsComputationRuntime, BfsConfig, BfsResult, BfsStorageRuntime};
pub use bridges::{
    BridgesAlgorithmSpec, BridgesComputationRuntime, BridgesConfig, BridgesResult,
    BridgesStorageRuntime,
};
pub use celf::{CELFAlgorithmSpec, CELFComputationRuntime, CELFConfig, CELFResult};
pub use closeness::{
    ClosenessCentralityAlgorithmSpec, ClosenessCentralityComputationRuntime,
    ClosenessCentralityConfig, ClosenessCentralityResult, ClosenessCentralityStorageRuntime,
};
pub use degree_centrality::{
    DEGREE_CENTRALITYAlgorithmSpec, DegreeCentralityComputationRuntime, DegreeCentralityConfig,
    DegreeCentralityResult, DegreeCentralityStorageRuntime,
};
pub use delta_stepping::{
    DELTA_STEPPINGAlgorithmSpec, DeltaSteppingComputationRuntime, DeltaSteppingConfig,
    DeltaSteppingResult, DeltaSteppingStorageRuntime,
};
pub use dfs::{DFSAlgorithmSpec, DfsComputationRuntime, DfsConfig, DfsResult, DfsStorageRuntime};
pub use dijkstra::{
    AllTargets, DIJKSTRAAlgorithmSpec, DijkstraComputationRuntime, DijkstraConfig, DijkstraResult,
    DijkstraStorageRuntime, ManyTargets, PathFindingResult, SingleTarget, Targets, TraversalState,
};
pub use harmonic::{
    HarmonicAlgorithmSpec, HarmonicComputationRuntime, HarmonicConfig, HarmonicResult,
    HarmonicStorageRuntime,
};
pub use hits::{HITSAlgorithmSpec, HitsConfig, HitsResult, HitsRunResult, HitsStorageRuntime, run_hits};
pub use k1coloring::{
    K1ColoringAlgorithmSpec, K1ColoringComputationRuntime, K1ColoringConfig, K1ColoringResult,
    K1ColoringStorageRuntime,
};
pub use kcore::{
    KCoreAlgorithmSpec, KCoreComputationResult, KCoreComputationRuntime, KCoreConfig, KCoreResult,
    KCoreStorageRuntime,
};
pub use kmeans::{KMeansComputationRuntime, KMeansConfig, KMeansResult, KMeansSamplerType};
pub use kspanningtree::{
    KSpanningTreeAlgorithmSpec, KSpanningTreeComputationRuntime, KSpanningTreeConfig,
    KSpanningTreeResult, KSpanningTreeStorageRuntime,
};
pub use label_propagation::{
    LabelPropAlgorithmSpec, LabelPropComputationRuntime, LabelPropConfig, LabelPropResult,
    LabelPropStorageRuntime,
};
pub use local_clustering_coefficient::{
    LOCAL_CLUSTERING_COEFFICIENTAlgorithmSpec, LocalClusteringCoefficientComputationRuntime,
    LocalClusteringCoefficientConfig, LocalClusteringCoefficientResult,
    LocalClusteringCoefficientStorageRuntime,
};

// Embeddings
pub use embeddings::{FastRPAlgorithmSpec, FastRPConfig, FastRPResult};
pub use embeddings::{Node2VecAlgorithmSpec, Node2VecConfig, Node2VecResult};
pub use louvain::{
    LouvainAlgorithmSpec, LouvainComputationRuntime, LouvainConfig, LouvainResult,
    LouvainStorageRuntime,
};
pub use msbfs::{AggregatedNeighborProcessingMsBfs, OMEGA};
pub use pagerank::{
    estimate_pagerank_memory, run_pagerank, PageRankAlgorithmSpec, PageRankComputationResult,
    PageRankMemoryEstimation, PageRankRunResult,
};
pub use scc::{SCCAlgorithmSpec, SccComputationRuntime, SccConfig, SccResult, SccStorageRuntime};
pub use spanning_tree::{
    SPANNING_TREEAlgorithmSpec, SpanningGraph, SpanningTree, SpanningTreeComputationRuntime,
    SpanningTreeConfig, SpanningTreeResult, SpanningTreeStorageRuntime,
};
pub use traversal::{
    Aggregator, ExitPredicate, ExitPredicateResult, FollowExitPredicate, OneHopAggregator,
    TargetExitPredicate,
};
pub use triangle_count::{
    TriangleCountAlgorithmSpec, TriangleCountComputationRuntime, TriangleCountConfig,
    TriangleCountResult, TriangleCountStorageRuntime,
};
pub use wcc::{WccAlgorithmSpec, WccComputationRuntime, WccConfig, WccResult, WccStorageRuntime};
pub use yens::{
    CandidatePathsPriorityQueue, MutablePathResult, RelationshipFilterer, YENSAlgorithmSpec,
    YensComputationRuntime, YensConfig, YensResult, YensStorageRuntime,
};

// pub use algorithms::*;
pub use core::*;
