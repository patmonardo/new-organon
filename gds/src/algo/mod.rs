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
/// Common utilities from Java GDS algo-common
/// - Result builders and statistics (centrality, community, similarity)
/// - Feature scaling for ML pipelines
/// - Common algorithm utilities
pub mod common;

/// Algorithm infrastructure (Genera)
/// - Centrality algorithm utilities
/// - Community detection utilities
/// - Algorithm-specific result types and transformations
pub mod algorithms;

// Module structure

pub mod all_shortest_paths;
pub mod approx_max_kcut;
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
pub mod embeddings;
pub mod harmonic;
pub mod hits;
pub mod index_inverse;
pub mod indirect_exposure;
pub mod k1coloring;
pub mod kcore;
pub mod kmeans;
pub mod kspanningtree;
pub mod label_propagation;
pub mod leiden;
pub mod louvain;
pub mod modularity;
pub mod modularity_optimization;
pub mod msbfs;
pub mod pagerank;
pub mod prize_collecting_steiner_tree;
pub mod random_walk;
pub mod scale_properties;
pub mod scc;
pub mod similarity;
pub mod spanning_tree;
pub mod steiner_tree;
pub mod topological_sort;
pub mod traversal;
pub mod triangle;
pub mod undirected;
pub mod walking;
pub mod wcc;
pub mod yens;

// pub use algorithms::*;
pub use common::*;
