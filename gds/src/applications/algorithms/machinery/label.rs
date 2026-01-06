//! Label modules (Java parity).
//!
//! Java references:
//! - `Label` (interface)
//! - `StandardLabel` (record)
//! - `AlgorithmLabel` (enum with mapping from Algorithm)

use crate::applications::algorithms::metadata::Algorithm;

/// A human-readable identifier used for computation, memory guard, metrics, etc.
///
/// This exists specifically to avoid passing strings everywhere.
pub trait Label {
    fn as_string(&self) -> &str;
}

impl std::fmt::Display for dyn Label + '_ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string())
    }
}

/// Java-parity: `StandardLabel(String value)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StandardLabel(pub String);

impl StandardLabel {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl Label for StandardLabel {
    fn as_string(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for StandardLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string())
    }
}

/// Java-parity: `AlgorithmLabel`.
///
/// In Java this is an enum of friendly names. In Rust we already have the authoritative
/// `Algorithm` enum, so `AlgorithmLabel` is a thin wrapper that provides the same mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlgorithmLabel(pub Algorithm);

impl AlgorithmLabel {
    /// Lookup powered by enum so the compiler tells us what to remember.
    pub fn from(algorithm: Algorithm) -> Self {
        Self(algorithm)
    }
}

impl Label for AlgorithmLabel {
    fn as_string(&self) -> &str {
        match self.0 {
            Algorithm::AllShortestPaths => "All Shortest Paths",
            Algorithm::ApproximateMaximumKCut => "ApproxMaxKCut",
            Algorithm::ArticleRank => "ArticleRank",
            Algorithm::ArticulationPoints => "Articulation Points",
            Algorithm::AStar => "AStar",
            Algorithm::BellmanFord => "Bellman-Ford",
            Algorithm::BetaClosenessCentrality => "Closeness Centrality (beta)",
            Algorithm::BetweennessCentrality => "Betweenness Centrality",
            Algorithm::BFS => "BFS",
            Algorithm::Bridges => "Bridges",
            Algorithm::CELF => "CELF",
            Algorithm::ClosenessCentrality => "Closeness Centrality",
            Algorithm::CollapsePath => "CollapsePath",
            Algorithm::Conductance => "Conductance",
            Algorithm::DegreeCentrality => "DegreeCentrality",
            Algorithm::DeltaStepping => "Delta Stepping",
            Algorithm::DFS => "DFS",
            Algorithm::Dijkstra => "Dijkstra",
            Algorithm::EigenVector => "EigenVector",
            Algorithm::FastRP => "FastRP",
            Algorithm::FilteredKNN => "Filtered K-Nearest Neighbours",
            Algorithm::FilteredNodeSimilarity => "Filtered Node Similarity",
            Algorithm::GraphSage => "GraphSage",
            Algorithm::GraphSageTrain => "GraphSageTrain",
            Algorithm::HarmonicCentrality => "HarmonicCentrality",
            Algorithm::HashGNN => "HashGNN",
            Algorithm::HITS => "HITS",
            Algorithm::IndexInverse => "IndexInverse",
            Algorithm::K1Coloring => "K1Coloring",
            Algorithm::KCore => "KCoreDecomposition",
            Algorithm::KGE => "KGE",
            Algorithm::KMeans => "K-Means",
            Algorithm::KNN => "K-Nearest Neighbours",
            Algorithm::KSpanningTree => "K Spanning Tree",
            Algorithm::LabelPropagation => "Label Propagation",
            Algorithm::Leiden => "Leiden",
            Algorithm::Louvain => "Louvain",
            Algorithm::LongestPath => "LongestPath",
            Algorithm::Modularity => "Modularity",
            Algorithm::ModularityOptimization => "ModularityOptimization",
            Algorithm::NodeSimilarity => "Node Similarity",
            Algorithm::Node2Vec => "Node2Vec",
            Algorithm::PageRank => "PageRank",
            Algorithm::PCST => "PrizeCollectingSteinerTree",
            Algorithm::RandomWalk => "RandomWalk",
            Algorithm::ScaleProperties => "ScaleProperties",
            Algorithm::SCC => "SCC",
            Algorithm::SingleSourceDijkstra => "All Shortest Paths",
            Algorithm::SLLPA => "SpeakerListenerLPA",
            Algorithm::SpanningTree => "SpanningTree",
            Algorithm::SplitRelationships => "SplitRelationships",
            Algorithm::SteinerTree => "SteinerTree",
            Algorithm::TopologicalSort => "TopologicalSort",
            Algorithm::ToUndirected => "ToUndirected",
            Algorithm::Triangles => "Triangle",
            Algorithm::WCC => "WCC",
            Algorithm::Yens => "Yens",
        }
    }
}

impl std::fmt::Display for AlgorithmLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string())
    }
}
