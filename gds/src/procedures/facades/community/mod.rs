//! Community detection algorithm facades
//!
//! Identifies groups of related nodes within graphs.
//! Algorithms in this module find dense subgraphs or communities.

pub mod triangle_count;
pub mod local_clustering_coefficient;
pub mod scc;
pub mod label_propagation;
pub mod wcc;
pub mod louvain;
pub mod leiden;
pub mod kmeans;
pub mod k1coloring;
pub mod kcore;
pub mod conductance;
pub mod approx_max_k_cut;
pub mod modularity;

pub use triangle_count::{TriangleCountBuilder, TriangleCountRow, TriangleCountStats};
pub use local_clustering_coefficient::{
	LocalClusteringCoefficientBuilder,
	LocalClusteringCoefficientRow,
	LocalClusteringCoefficientStats,
};

pub use scc::{SccBuilder, SccRow, SccStats};

pub use label_propagation::{
	LabelPropagationBuilder,
	LabelPropagationRow,
	LabelPropagationStats,
};

pub use wcc::{WccBuilder, WccRow, WccStats};
pub use louvain::{LouvainBuilder, LouvainRow, LouvainStats};
pub use kmeans::{KMeansBuilder, KMeansRow, KMeansStats};
pub use k1coloring::{K1ColoringBuilder, K1ColoringRow, K1ColoringStats};
pub use kcore::{KCoreBuilder, KCoreRow, KCoreStats};
pub use conductance::{ConductanceBuilder, ConductanceRow, ConductanceStats};
pub use approx_max_k_cut::{ApproxMaxKCutBuilder, ApproxMaxKCutRow, ApproxMaxKCutStats};
pub use modularity::{ModularityBuilder, ModularityRow, ModularityStats};

