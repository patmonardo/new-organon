//! Algorithms Procedure Facade
//!
//! Thin, Java-shaped aggregator that groups procedure facades by category.
//! This keeps the surface linear and discoverable while delegating to the
//! per-algorithm facades defined elsewhere in `procedures/`.

use std::sync::Arc;

use crate::procedures::centrality::{
    ArticulationPointsFacade, BetweennessCentralityFacade, BridgesFacade, CELFFacade,
    ClosenessCentralityFacade, DegreeCentralityFacade, HarmonicCentralityFacade,
    HitsCentralityFacade, PageRankFacade,
};
use crate::procedures::community::{
    ApproxMaxKCutFacade, ConductanceFacade, K1ColoringFacade, KCoreFacade, KMeansFacade,
    LabelPropagationFacade, LeidenFacade, LouvainFacade, ModularityFacade, SccFacade,
    TriangleFacade, WccFacade,
};
use crate::procedures::embeddings::{
    FastRPBuilder, GATBuilder, GraphSageBuilder, HashGNNBuilder, Node2VecBuilder,
};
use crate::procedures::miscellaneous::{
    CollapsePathFacade, IndexInverseFacade, IndirectExposureFacade, ScalePropertiesFacade,
    ToUndirectedFacade,
};
use crate::procedures::pathfinding::{
    AStarBuilder, AllShortestPathsBuilder, BellmanFordBuilder, BfsBuilder, DagLongestPathBuilder,
    DeltaSteppingBuilder, DfsBuilder, DijkstraBuilder, KSpanningTreeBuilder, PCSTreeBuilder,
    RandomWalkBuilder, SpanningTreeBuilder, SteinerTreeBuilder, TopologicalSortBuilder,
    YensBuilder,
};
use crate::procedures::similarity::{
    FilteredKnnFacade, FilteredNodeSimilarityFacade, KnnFacade, NodeSimilarityFacade,
};
use crate::types::prelude::DefaultGraphStore;

/// Top-level aggregator for algorithm procedure facades.
#[derive(Clone)]
pub struct AlgorithmsProcedureFacade {
    centrality: CentralityProcedureFacade,
    community: CommunityProcedureFacade,
    embeddings: NodeEmbeddingsProcedureFacade,
    miscellaneous: MiscellaneousProcedureFacade,
    pathfinding: PathFindingProcedureFacade,
    similarity: SimilarityProcedureFacade,
}

impl AlgorithmsProcedureFacade {
    pub fn new(
        centrality: CentralityProcedureFacade,
        community: CommunityProcedureFacade,
        embeddings: NodeEmbeddingsProcedureFacade,
        miscellaneous: MiscellaneousProcedureFacade,
        pathfinding: PathFindingProcedureFacade,
        similarity: SimilarityProcedureFacade,
    ) -> Self {
        Self {
            centrality,
            community,
            embeddings,
            miscellaneous,
            pathfinding,
            similarity,
        }
    }

    /// Convenience constructor for store-backed procedure facades.
    pub fn from_store(store: Arc<DefaultGraphStore>) -> Self {
        Self {
            centrality: CentralityProcedureFacade::new(Arc::clone(&store)),
            community: CommunityProcedureFacade::new(Arc::clone(&store)),
            embeddings: NodeEmbeddingsProcedureFacade::new(Arc::clone(&store)),
            miscellaneous: MiscellaneousProcedureFacade::new(Arc::clone(&store)),
            pathfinding: PathFindingProcedureFacade::new(Arc::clone(&store)),
            similarity: SimilarityProcedureFacade::new(store),
        }
    }

    pub fn centrality(&self) -> CentralityProcedureFacade {
        self.centrality.clone()
    }

    pub fn community(&self) -> CommunityProcedureFacade {
        self.community.clone()
    }

    pub fn embeddings(&self) -> NodeEmbeddingsProcedureFacade {
        self.embeddings.clone()
    }

    pub fn miscellaneous(&self) -> MiscellaneousProcedureFacade {
        self.miscellaneous.clone()
    }

    pub fn pathfinding(&self) -> PathFindingProcedureFacade {
        self.pathfinding.clone()
    }

    pub fn similarity(&self) -> SimilarityProcedureFacade {
        self.similarity.clone()
    }
}

/// Centrality category procedure facade.
#[derive(Clone)]
pub struct CentralityProcedureFacade {
    store: Arc<DefaultGraphStore>,
}

impl CentralityProcedureFacade {
    pub fn new(store: Arc<DefaultGraphStore>) -> Self {
        Self { store }
    }

    pub fn degree_centrality(&self) -> DegreeCentralityFacade {
        DegreeCentralityFacade::new(Arc::clone(&self.store))
    }

    pub fn closeness(&self) -> ClosenessCentralityFacade {
        ClosenessCentralityFacade::new(Arc::clone(&self.store))
    }

    pub fn harmonic(&self) -> HarmonicCentralityFacade {
        HarmonicCentralityFacade::new(Arc::clone(&self.store))
    }

    pub fn betweenness(&self) -> BetweennessCentralityFacade {
        BetweennessCentralityFacade::new(Arc::clone(&self.store))
    }

    pub fn pagerank(&self) -> PageRankFacade {
        PageRankFacade::new(Arc::clone(&self.store))
    }

    pub fn hits(&self) -> HitsCentralityFacade {
        HitsCentralityFacade::new(Arc::clone(&self.store))
    }

    pub fn articulation_points(&self) -> ArticulationPointsFacade {
        ArticulationPointsFacade::new(Arc::clone(&self.store))
    }

    pub fn bridges(&self) -> BridgesFacade {
        BridgesFacade::new(Arc::clone(&self.store))
    }

    pub fn celf(&self) -> CELFFacade {
        CELFFacade::new(Arc::clone(&self.store))
    }
}

/// Community detection category procedure facade.
#[derive(Clone)]
pub struct CommunityProcedureFacade {
    store: Arc<DefaultGraphStore>,
}

impl CommunityProcedureFacade {
    pub fn new(store: Arc<DefaultGraphStore>) -> Self {
        Self { store }
    }

    pub fn approx_max_k_cut(&self) -> ApproxMaxKCutFacade {
        ApproxMaxKCutFacade::new(Arc::clone(&self.store))
    }

    pub fn conductance(&self, community_property: impl Into<String>) -> ConductanceFacade {
        ConductanceFacade::new(Arc::clone(&self.store), community_property.into())
    }

    pub fn k1_coloring(&self) -> K1ColoringFacade {
        K1ColoringFacade::new(Arc::clone(&self.store))
    }

    pub fn kcore(&self) -> KCoreFacade {
        KCoreFacade::new(Arc::clone(&self.store))
    }

    pub fn kmeans(&self) -> KMeansFacade {
        KMeansFacade::new(Arc::clone(&self.store))
    }

    pub fn label_propagation(&self) -> LabelPropagationFacade {
        LabelPropagationFacade::new(Arc::clone(&self.store))
    }

    pub fn leiden(&self) -> LeidenFacade {
        LeidenFacade::new(Arc::clone(&self.store))
    }

    pub fn louvain(&self) -> LouvainFacade {
        LouvainFacade::new(Arc::clone(&self.store))
    }

    pub fn modularity(&self, community_property: impl Into<String>) -> ModularityFacade {
        ModularityFacade::new(Arc::clone(&self.store), community_property.into())
    }

    pub fn scc(&self) -> SccFacade {
        SccFacade::new(Arc::clone(&self.store))
    }

    pub fn triangle(&self) -> TriangleFacade {
        TriangleFacade::new(Arc::clone(&self.store))
    }

    pub fn wcc(&self) -> WccFacade {
        WccFacade::new(Arc::clone(&self.store))
    }
}

/// Node embeddings category procedure facade.
#[derive(Clone)]
pub struct NodeEmbeddingsProcedureFacade {
    store: Arc<DefaultGraphStore>,
}

impl NodeEmbeddingsProcedureFacade {
    pub fn new(store: Arc<DefaultGraphStore>) -> Self {
        Self { store }
    }

    pub fn fast_rp(&self) -> FastRPBuilder {
        FastRPBuilder::new(Arc::clone(&self.store))
    }

    pub fn hash_gnn(&self) -> HashGNNBuilder {
        HashGNNBuilder::new(Arc::clone(&self.store))
    }

    pub fn gat(&self) -> GATBuilder {
        GATBuilder::new(Arc::clone(&self.store))
    }

    pub fn graphsage(&self) -> GraphSageBuilder {
        GraphSageBuilder::new(Arc::clone(&self.store))
    }

    pub fn node2vec(&self) -> Node2VecBuilder {
        Node2VecBuilder::new(Arc::clone(&self.store))
    }
}

/// Miscellaneous utilities category procedure facade.
#[derive(Clone)]
pub struct MiscellaneousProcedureFacade {
    store: Arc<DefaultGraphStore>,
}

impl MiscellaneousProcedureFacade {
    pub fn new(store: Arc<DefaultGraphStore>) -> Self {
        Self { store }
    }

    pub fn collapse_path(&self) -> CollapsePathFacade {
        CollapsePathFacade::new(Arc::clone(&self.store))
    }

    pub fn index_inverse(&self) -> IndexInverseFacade {
        IndexInverseFacade::new(Arc::clone(&self.store))
    }

    pub fn indirect_exposure(&self) -> IndirectExposureFacade {
        IndirectExposureFacade::new(Arc::clone(&self.store))
    }

    pub fn scale_properties(&self) -> ScalePropertiesFacade {
        ScalePropertiesFacade::new(Arc::clone(&self.store))
    }

    pub fn to_undirected(&self) -> ToUndirectedFacade {
        ToUndirectedFacade::new(Arc::clone(&self.store))
    }
}

/// Pathfinding category procedure facade.
#[derive(Clone)]
pub struct PathFindingProcedureFacade {
    store: Arc<DefaultGraphStore>,
}

impl PathFindingProcedureFacade {
    pub fn new(store: Arc<DefaultGraphStore>) -> Self {
        Self { store }
    }

    pub fn bfs(&self) -> BfsBuilder {
        BfsBuilder::new(Arc::clone(&self.store))
    }

    pub fn dfs(&self) -> DfsBuilder {
        DfsBuilder::new(Arc::clone(&self.store))
    }

    pub fn dijkstra(&self) -> DijkstraBuilder {
        DijkstraBuilder::new(Arc::clone(&self.store))
    }

    pub fn astar(&self) -> AStarBuilder {
        AStarBuilder::new(Arc::clone(&self.store))
    }

    pub fn bellman_ford(&self) -> BellmanFordBuilder {
        BellmanFordBuilder::new(Arc::clone(&self.store))
    }

    pub fn delta_stepping(&self) -> DeltaSteppingBuilder {
        DeltaSteppingBuilder::new(Arc::clone(&self.store))
    }

    pub fn yens(&self) -> YensBuilder {
        YensBuilder::new(Arc::clone(&self.store))
    }

    pub fn all_shortest_paths(&self) -> AllShortestPathsBuilder {
        AllShortestPathsBuilder::new(Arc::clone(&self.store))
    }

    pub fn spanning_tree(&self) -> SpanningTreeBuilder {
        SpanningTreeBuilder::new(Arc::clone(&self.store))
    }

    pub fn kspanning_tree(&self) -> KSpanningTreeBuilder {
        KSpanningTreeBuilder::new(Arc::clone(&self.store))
    }

    pub fn steiner_tree(&self) -> SteinerTreeBuilder {
        SteinerTreeBuilder::new(Arc::clone(&self.store))
    }

    pub fn prize_collecting_steiner_tree(&self) -> PCSTreeBuilder {
        PCSTreeBuilder::new(Arc::clone(&self.store))
    }

    pub fn topological_sort(&self) -> TopologicalSortBuilder {
        TopologicalSortBuilder::new(Arc::clone(&self.store))
    }

    pub fn dag_longest_path(&self) -> DagLongestPathBuilder {
        DagLongestPathBuilder::new(Arc::clone(&self.store))
    }

    pub fn random_walk(&self) -> RandomWalkBuilder {
        RandomWalkBuilder::new(Arc::clone(&self.store))
    }
}

/// Similarity category procedure facade.
#[derive(Clone)]
pub struct SimilarityProcedureFacade {
    store: Arc<DefaultGraphStore>,
}

impl SimilarityProcedureFacade {
    pub fn new(store: Arc<DefaultGraphStore>) -> Self {
        Self { store }
    }

    pub fn node_similarity(&self) -> NodeSimilarityFacade {
        NodeSimilarityFacade::new(Arc::clone(&self.store))
    }

    pub fn knn(&self, node_property: impl Into<String>) -> KnnFacade {
        KnnFacade::new(Arc::clone(&self.store), node_property)
    }

    pub fn filtered_knn(&self, node_property: impl Into<String>) -> FilteredKnnFacade {
        FilteredKnnFacade::new(Arc::clone(&self.store), node_property)
    }

    pub fn filtered_node_similarity(&self) -> FilteredNodeSimilarityFacade {
        FilteredNodeSimilarityFacade::new(Arc::clone(&self.store))
    }
}
