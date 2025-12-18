//! Centrality algorithm facades
//!
//! Provides user-friendly APIs for centrality algorithms:
//! - DegreeCentrality: Counts node connections
//! - PageRank: Importance based on link structure
//! - Betweenness: Frequency of appearance in shortest paths
//! - Closeness: Average distance to all other nodes
//! - Harmonic: Reciprocal distances
//! - HITS: Hub and authority scores

pub mod articulation_points;
pub mod betweenness;
pub mod bridges;
pub mod celf;
pub mod closeness;
pub mod degree_centrality;
pub mod harmonic;
pub mod hits_pregel;
pub mod pagerank;

// Re-export main facades
pub use articulation_points::{
    ArticulationPointRow, ArticulationPointsFacade, ArticulationPointsStats,
};
pub use betweenness::BetweennessCentralityFacade;
pub use bridges::{BridgeRow, BridgesFacade, BridgesStats};
pub use celf::{CELFFacade, CELFRow, CELFStats};
pub use closeness::ClosenessCentralityFacade;
pub use degree_centrality::DegreeCentralityFacade;
pub use harmonic::HarmonicCentralityFacade;
pub use hits_pregel::{HitsPregelBuilder, HitsPregelRow, HitsPregelStats};
pub use pagerank::PageRankBuilder;
