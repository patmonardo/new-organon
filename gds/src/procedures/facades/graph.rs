use std::sync::Arc;

use crate::types::prelude::DefaultGraphStore;

use super::pathfinding::{
    AStarBuilder, AllShortestPathsBuilder, BellmanFordBuilder, BfsBuilder, DeltaSteppingBuilder,
    DfsBuilder, DijkstraBuilder, SpanningTreeBuilder, YensBuilder,
};

use super::centrality::{
    BetweennessCentralityFacade, ClosenessCentralityFacade, DegreeCentralityFacade,
    HarmonicCentralityFacade, HitsPregelBuilder, PageRankBuilder,
};

use super::community::{
    K1ColoringBuilder,
    KCoreBuilder,
    KMeansBuilder,
    LabelPropagationBuilder,
    LouvainBuilder,
    LocalClusteringCoefficientBuilder,
    SccBuilder,
    TriangleCountBuilder,
    WccBuilder,
};

/// User-facing graph handle for running algorithms against a live `DefaultGraphStore`.
///
/// This is the main entrypoint for the facade layer.
#[derive(Clone)]
pub struct Graph {
    store: Arc<DefaultGraphStore>,
}

impl Graph {
    /// Create a new facade graph handle from an in-memory graph store.
    pub fn new(store: Arc<DefaultGraphStore>) -> Self {
        Self { store }
    }

    /// Access the underlying graph store.
    pub fn store(&self) -> &Arc<DefaultGraphStore> {
        &self.store
    }

    /// Breadth-first search traversal.
    pub fn bfs(&self) -> BfsBuilder {
        BfsBuilder::new(Arc::clone(&self.store))
    }

    /// Depth-first search traversal.
    pub fn dfs(&self) -> DfsBuilder {
        DfsBuilder::new(Arc::clone(&self.store))
    }

    /// Dijkstra shortest-paths.
    pub fn dijkstra(&self) -> DijkstraBuilder {
        DijkstraBuilder::new(Arc::clone(&self.store))
    }

    /// A* shortest-path (heuristic-guided).
    pub fn astar(&self) -> AStarBuilder {
        AStarBuilder::new(Arc::clone(&self.store))
    }

    /// Bellman-Ford shortest-paths (supports negative weights; detects negative cycles).
    pub fn bellman_ford(&self) -> BellmanFordBuilder {
        BellmanFordBuilder::new(Arc::clone(&self.store))
    }

    /// Delta Stepping shortest-paths (binning strategy).
    pub fn delta_stepping(&self) -> DeltaSteppingBuilder {
        DeltaSteppingBuilder::new(Arc::clone(&self.store))
    }

    /// Yen's K-shortest simple paths (single-pair).
    pub fn yens(&self) -> YensBuilder {
        YensBuilder::new(Arc::clone(&self.store))
    }

    /// All-pairs shortest path distances.
    pub fn all_shortest_paths(&self) -> AllShortestPathsBuilder {
        AllShortestPathsBuilder::new(Arc::clone(&self.store))
    }

    /// Spanning tree via Prim's algorithm.
    pub fn spanning_tree(&self) -> SpanningTreeBuilder {
        SpanningTreeBuilder::new(Arc::clone(&self.store))
    }

    /// Degree centrality (counts connections per node).
    pub fn degree_centrality(&self) -> DegreeCentralityFacade {
        DegreeCentralityFacade::new(Arc::clone(&self.store))
    }

    /// Closeness centrality (distance-based centrality).
    pub fn closeness(&self) -> ClosenessCentralityFacade {
        ClosenessCentralityFacade::new(Arc::clone(&self.store))
    }

    /// Harmonic centrality (reciprocal distances).
    pub fn harmonic(&self) -> HarmonicCentralityFacade {
        HarmonicCentralityFacade::new(Arc::clone(&self.store))
    }

    /// Betweenness centrality (Brandes shortest-path dependency).
    pub fn betweenness(&self) -> BetweennessCentralityFacade {
        BetweennessCentralityFacade::new(Arc::clone(&self.store))
    }

    /// PageRank (delta-based, Java GDS aligned).
    pub fn pagerank(&self) -> PageRankBuilder {
        PageRankBuilder::new(Arc::clone(&self.store))
    }

    /// HITS Pregel (bidirectional authority/hub scoring).
    ///
    /// Demonstrates bidirectional Pregel: authority from incoming neighbors, hubs from outgoing.
    pub fn hits_pregel(&self) -> HitsPregelBuilder {
        HitsPregelBuilder::new(Arc::clone(&self.store))
    }

    /// Node Similarity (Jaccard, Cosine, Overlap).
    pub fn node_similarity(&self) -> crate::procedures::facades::similarity::SimilarityBuilder {
        crate::procedures::facades::similarity::SimilarityBuilder::new(Arc::clone(&self.store))
    }

    /// Triangle Count (per-node triangles + global triangle count).
    pub fn triangle_count(&self) -> TriangleCountBuilder {
        TriangleCountBuilder::new(Arc::clone(&self.store))
    }

    /// Local Clustering Coefficient (per-node coefficient + average).
    pub fn local_clustering_coefficient(&self) -> LocalClusteringCoefficientBuilder {
        LocalClusteringCoefficientBuilder::new(Arc::clone(&self.store))
    }

    /// Strongly Connected Components (directed graph SCCs).
    pub fn scc(&self) -> SccBuilder {
        SccBuilder::new(Arc::clone(&self.store))
    }

    /// Label Propagation (community detection via label voting).
    pub fn label_propagation(&self) -> LabelPropagationBuilder {
        LabelPropagationBuilder::new(Arc::clone(&self.store))
    }

    /// Weakly Connected Components (undirected connectivity).
    pub fn wcc(&self) -> WccBuilder {
        WccBuilder::new(Arc::clone(&self.store))
    }

    /// Louvain community detection (modularity optimization).
    pub fn louvain(&self) -> LouvainBuilder {
        LouvainBuilder::new(Arc::clone(&self.store))
    }

    /// K-Means clustering (community detection on feature vectors).
    pub fn kmeans(&self) -> KMeansBuilder {
        KMeansBuilder::new(Arc::clone(&self.store))
    }

    /// K1-Coloring (greedy graph coloring).
    pub fn k1coloring(&self) -> K1ColoringBuilder {
        K1ColoringBuilder::new(Arc::clone(&self.store))
    }

    /// K-Core Decomposition (core values).
    pub fn kcore(&self) -> KCoreBuilder {
        KCoreBuilder::new(Arc::clone(&self.store))
    }
}
