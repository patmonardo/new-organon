//! Community detection algorithm facades
//!
//! Identifies groups of related nodes within graphs.
//! Algorithms in this module find dense subgraphs or communities.

pub mod approx_max_k_cut;
pub mod conductance;
pub mod k1coloring;
pub mod kcore;
pub mod kmeans;
pub mod label_propagation;
pub mod leiden;
pub mod local_clustering_coefficient;
pub mod louvain;
pub mod modularity;
pub mod scc;
pub mod triangle_count;
pub mod wcc;

pub use local_clustering_coefficient::{
    LocalClusteringCoefficientFacade, LocalClusteringCoefficientRow,
    LocalClusteringCoefficientStats,
};
pub use triangle_count::{TriangleCountFacade, TriangleCountRow, TriangleCountStats};

pub use scc::{SccFacade, SccRow, SccStats};

pub use label_propagation::{LabelPropagationFacade, LabelPropagationRow, LabelPropagationStats};

pub use approx_max_k_cut::{ApproxMaxKCutFacade, ApproxMaxKCutRow, ApproxMaxKCutStats};
pub use conductance::{ConductanceFacade, ConductanceRow, ConductanceStats};
pub use k1coloring::{K1ColoringFacade, K1ColoringRow, K1ColoringStats};
pub use kcore::{KCoreFacade, KCoreRow, KCoreStats};
pub use kmeans::{KMeansFacade, KMeansRow, KMeansStats};
pub use leiden::{LeidenFacade, LeidenRow, LeidenStats};
pub use louvain::{LouvainFacade, LouvainRow, LouvainStats};
pub use modularity::{ModularityFacade, ModularityRow, ModularityStats};
pub use wcc::{WccFacade, WccRow, WccStats};
