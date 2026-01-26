use std::sync::Arc;

use crate::types::prelude::DefaultGraphStore;

use super::pathfinding::{
    AStarBuilder, AllShortestPathsBuilder, BellmanFordBuilder, BfsBuilder, DagLongestPathBuilder,
    DeltaSteppingBuilder, DfsBuilder, DijkstraBuilder, KSpanningTreeBuilder, RandomWalkBuilder,
    SpanningTreeBuilder, SteinerTreeBuilder, TopologicalSortBuilder, YensBuilder,
};

use super::centrality::{
    ArticulationPointsFacade, BetweennessCentralityFacade, BridgesFacade, CELFFacade,
    ClosenessCentralityFacade, DegreeCentralityFacade, HarmonicCentralityFacade,
    HitsCentralityFacade, PageRankFacade,
};

use super::community::{
    ApproxMaxKCutFacade, ConductanceFacade, K1ColoringFacade, KCoreFacade, KMeansFacade,
    LabelPropagationFacade, LeidenFacade, LouvainFacade, ModularityFacade, SccFacade,
    TriangleFacade, WccFacade,
};

use super::embeddings::{
    FastRPBuilder, GATBuilder, GraphSageBuilder, HashGNNBuilder, Node2VecBuilder,
};

use super::miscellaneous::{
    CollapsePathFacade, IndexInverseFacade, ScalePropertiesFacade, ToUndirectedFacade,
};

use super::similarity::NodeSimilarityFacade;

/// User-facing graph handle for running algorithms against a live `DefaultGraphStore`.
///
/// This is the main entrypoint for the facade layer.
#[derive(Clone)]
pub struct GraphFacade {
    store: Arc<DefaultGraphStore>,
}

impl GraphFacade {
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

    pub fn node2vec(&self) -> Node2VecBuilder {
        Node2VecBuilder::new(Arc::clone(&self.store))
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

    /// K-spanning tree (prune MST to exactly k nodes).
    pub fn kspanning_tree(&self) -> KSpanningTreeBuilder {
        KSpanningTreeBuilder::new(Arc::clone(&self.store))
    }

    /// Steiner tree (minimum tree connecting source to terminals).
    pub fn steiner_tree(&self) -> SteinerTreeBuilder {
        SteinerTreeBuilder::new(Arc::clone(&self.store))
    }

    /// Topological sort for directed acyclic graphs (DAG).
    pub fn topological_sort(&self) -> TopologicalSortBuilder {
        TopologicalSortBuilder::new(Arc::clone(&self.store))
    }

    /// Longest path in directed acyclic graphs (DAG).
    pub fn dag_longest_path(&self) -> DagLongestPathBuilder {
        DagLongestPathBuilder::new(Arc::clone(&self.store))
    }

    pub fn random_walk(&self) -> RandomWalkBuilder {
        RandomWalkBuilder::new(Arc::clone(&self.store))
    }

    /// FastRP node embeddings.
    pub fn fast_rp(&self) -> FastRPBuilder {
        FastRPBuilder::new(Arc::clone(&self.store))
    }

    /// HashGNN node embeddings.
    pub fn hash_gnn(&self) -> HashGNNBuilder {
        HashGNNBuilder::new(Arc::clone(&self.store))
    }

    /// GAT node embeddings (Graph Attention Network).
    pub fn gat(&self) -> GATBuilder {
        GATBuilder::new(Arc::clone(&self.store))
    }

    /// GraphSAGE node embeddings (inductive representation learning).
    pub fn graphsage(&self) -> GraphSageBuilder {
        GraphSageBuilder::new(Arc::clone(&self.store))
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
    pub fn pagerank(&self) -> PageRankFacade {
        PageRankFacade::new(Arc::clone(&self.store))
    }

    /// HITS (bidirectional authority/hub scoring).
    ///
    /// Implemented using bidirectional Pregel: authority from incoming neighbors, hubs from outgoing.
    pub fn hits(&self) -> HitsCentralityFacade {
        HitsCentralityFacade::new(Arc::clone(&self.store))
    }

    /// Articulation Points (cut vertices) for undirected connectivity.
    pub fn articulation_points(&self) -> ArticulationPointsFacade {
        ArticulationPointsFacade::new(Arc::clone(&self.store))
    }

    /// Bridges (cut edges) for undirected graphs.
    pub fn bridges(&self) -> BridgesFacade {
        BridgesFacade::new(Arc::clone(&self.store))
    }

    /// CELF (Cost-Effective Lazy Forward) influence maximization.
    pub fn celf(&self) -> CELFFacade {
        CELFFacade::new(Arc::clone(&self.store))
    }

    /// Node Similarity (Jaccard, Cosine, Overlap).
    pub fn node_similarity(&self) -> NodeSimilarityFacade {
        NodeSimilarityFacade::new(Arc::clone(&self.store))
    }

    /// Triangle Count (per-node triangles + global triangle count).
    pub fn triangle(&self) -> TriangleFacade {
        TriangleFacade::new(Arc::clone(&self.store))
    }

    /// Strongly Connected Components (directed graph SCCs).
    pub fn scc(&self) -> SccFacade {
        SccFacade::new(Arc::clone(&self.store))
    }

    /// Label Propagation (community detection via label voting).
    pub fn label_propagation(&self) -> LabelPropagationFacade {
        LabelPropagationFacade::new(Arc::clone(&self.store))
    }

    /// Weakly Connected Components (undirected connectivity).
    pub fn wcc(&self) -> WccFacade {
        WccFacade::new(Arc::clone(&self.store))
    }

    /// Louvain community detection (modularity optimization).
    pub fn louvain(&self) -> LouvainFacade {
        LouvainFacade::new(Arc::clone(&self.store))
    }

    /// K-Means clustering (community detection on feature vectors).
    pub fn kmeans(&self) -> KMeansFacade {
        KMeansFacade::new(Arc::clone(&self.store))
    }

    /// Leiden community detection (modularity refinement with connected components).
    pub fn leiden(&self) -> LeidenFacade {
        LeidenFacade::new(Arc::clone(&self.store))
    }

    /// K1-Coloring (greedy graph coloring).
    pub fn k1coloring(&self) -> K1ColoringFacade {
        K1ColoringFacade::new(Arc::clone(&self.store))
    }

    /// K-Core Decomposition (core values).
    pub fn kcore(&self) -> KCoreFacade {
        KCoreFacade::new(Arc::clone(&self.store))
    }

    /// Conductance community quality metric.
    pub fn conductance(&self, community_property: String) -> ConductanceFacade {
        ConductanceFacade::new(Arc::clone(&self.store), community_property)
    }

    /// Approximate maximum k-cut partitioning.
    pub fn approx_max_kcut(&self) -> ApproxMaxKCutFacade {
        ApproxMaxKCutFacade::new(Arc::clone(&self.store))
    }

    /// Modularity community quality metric.
    pub fn modularity(&self, community_property: String) -> ModularityFacade {
        ModularityFacade::new(Arc::clone(&self.store), community_property)
    }

    /// Converts the graph to an undirected projection (utility).
    pub fn to_undirected(&self) -> ToUndirectedFacade {
        ToUndirectedFacade::new(Arc::clone(&self.store))
    }

    /// Scales a numeric node property (utility).
    pub fn scale_properties(&self) -> ScalePropertiesFacade {
        ScalePropertiesFacade::new(Arc::clone(&self.store))
    }

    /// Builds inverse indices for relationships (placeholder utility).
    pub fn index_inverse(&self) -> IndexInverseFacade {
        IndexInverseFacade::new(Arc::clone(&self.store))
    }

    /// Collapses paths into relationships (placeholder utility).
    pub fn collapse_path(&self) -> CollapsePathFacade {
        CollapsePathFacade::new(Arc::clone(&self.store))
    }
}
