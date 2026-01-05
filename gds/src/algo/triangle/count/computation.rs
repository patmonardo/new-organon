use super::spec::TriangleCountResult;
use crate::algo::triangle::intersect::{
    AdjacencyProvider, GraphIntersect, RelationshipIntersect, RelationshipIntersectConfig,
};

pub struct TriangleCountComputationRuntime {}

impl TriangleCountComputationRuntime {
    pub fn new() -> Self {
        Self {}
    }

    /// Counts triangles using neighbor-set intersections.
    ///
    /// Contract:
    /// - `get_neighbors(i)` must return a sorted, deduplicated list of neighbor node indices.
    /// - The graph is assumed to be undirected (i.e. if `u` lists `v`, then `v` lists `u`).
    ///
    /// Counting rule:
    /// - Each triangle is counted exactly once by enforcing ordering $a < b < c$.
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> TriangleCountResult {
        self.compute_with_max_degree(node_count, get_neighbors, u64::MAX)
    }

    pub fn compute_with_max_degree(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
        max_degree: u64,
    ) -> TriangleCountResult {
        if node_count == 0 {
            return TriangleCountResult {
                local_triangles: Vec::new(),
                global_triangles: 0,
            };
        }

        let mut neighbors_cache: Vec<Vec<usize>> =
            (0..node_count).map(|n| get_neighbors(n)).collect();
        for n in 0..node_count {
            neighbors_cache[n].sort_unstable();
            neighbors_cache[n].dedup();
        }

        struct CacheProvider<'a> {
            adj: &'a [Vec<usize>],
        }

        impl<'a> AdjacencyProvider for CacheProvider<'a> {
            fn degree(&self, node: usize) -> usize {
                self.adj[node].len()
            }

            fn neighbors(&self, node: usize) -> &[usize] {
                &self.adj[node]
            }
        }

        let provider = CacheProvider {
            adj: &neighbors_cache,
        };

        let mut local = vec![0u64; node_count];
        let mut global = 0u64;

        let mut intersect = GraphIntersect::new(&provider, RelationshipIntersectConfig { max_degree });
        for a in 0..node_count {
            let mut consumer = |c: usize, b: usize, a: usize| {
                if a < node_count && b < node_count && c < node_count {
                    local[a] += 1;
                    local[b] += 1;
                    local[c] += 1;
                    global += 1;
                }
            };
            intersect.intersect_all(a, &mut consumer);
        }

        TriangleCountResult {
            local_triangles: local,
            global_triangles: global,
        }
    }
}

impl Default for TriangleCountComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}
