//! Community detection algorithm facades
//!
//! Identifies groups of related nodes within graphs.
//! Algorithms in this module find dense subgraphs or communities.

pub mod approx_max_kcut;
pub mod conductance;
pub mod k1coloring;
pub mod kcore;
pub mod kmeans;
pub mod label_propagation;
pub mod leiden;
pub mod louvain;
pub mod modularity;
pub mod scc;
pub mod triangle;
pub mod wcc;

pub use triangle::{TriangleFacade, TriangleRow, TriangleStats};

pub use scc::{SccFacade, SccRow, SccStats};

pub use label_propagation::{LabelPropagationFacade, LabelPropagationRow, LabelPropagationStats};

pub use approx_max_kcut::{ApproxMaxKCutFacade, ApproxMaxKCutRow, ApproxMaxKCutStats};
pub use conductance::{ConductanceFacade, ConductanceRow, ConductanceStats};
pub use k1coloring::{K1ColoringFacade, K1ColoringRow, K1ColoringStats};
pub use kcore::{KCoreFacade, KCoreRow, KCoreStats};
pub use kmeans::{KMeansFacade, KMeansRow, KMeansStats};
pub use leiden::{LeidenFacade, LeidenRow, LeidenStats};
pub use louvain::{LouvainFacade, LouvainRow, LouvainStats};
pub use modularity::{ModularityFacade, ModularityRow, ModularityStats};
pub use wcc::{WccFacade, WccRow, WccStats};
