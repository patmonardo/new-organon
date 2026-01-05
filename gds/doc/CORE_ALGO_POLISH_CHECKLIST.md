# Core algo polish checklist (non-ML)

Date: 2026-01-02

Intent: tomorrow’s work is a focused “polish pass” over core algorithms (exclude ML/embeddings). This checklist captures current estimation surface and placeholders so the sweep is mechanical.

## Application handler surface (mode tokens)

Target: standardize estimation to one token (likely `estimate_memory`) across core handlers.

| Group | Op | Handler | Estimation token(s) |
|---|---|---|---|
| centrality | articulation_points | gds/src/applications/algorithms/centrality/articulation_points.rs | estimate_memory |
| centrality | betweenness | gds/src/applications/algorithms/centrality/betweenness.rs | estimate_memory |
| centrality | bridges | gds/src/applications/algorithms/centrality/bridges.rs | estimate_memory |
| centrality | celf | gds/src/applications/algorithms/centrality/celf.rs | estimate_memory |
| centrality | closeness | gds/src/applications/algorithms/centrality/closeness.rs | estimate_memory |
| centrality | degree_centrality | gds/src/applications/algorithms/centrality/degree_centrality.rs | estimate_memory |
| centrality | harmonic | gds/src/applications/algorithms/centrality/harmonic.rs | estimate_memory |
| centrality | hits | gds/src/applications/algorithms/centrality/hits.rs | estimate_memory |
| centrality | pagerank | gds/src/applications/algorithms/centrality/pagerank.rs | estimate_memory |
| community | approxMaxKCut | gds/src/applications/algorithms/community/approx_max_kcut.rs |  |
| community | conductance | gds/src/applications/algorithms/community/conductance.rs | estimate_memory |
| community | k1coloring | gds/src/applications/algorithms/community/k1coloring.rs | estimate_memory |
| community | kcore | gds/src/applications/algorithms/community/kcore.rs | estimate_memory |
| community | kmeans | gds/src/applications/algorithms/community/kmeans.rs |  |
| community | labelPropagation | gds/src/applications/algorithms/community/label_propagation.rs | estimate_memory |
| community | leiden | gds/src/applications/algorithms/community/leiden.rs |  |
| community | localClusteringCoefficient | gds/src/applications/algorithms/community/local_clustering_coefficient.rs | estimate_memory |
| community | louvain | gds/src/applications/algorithms/community/louvain.rs | estimate_memory |
| community | modularity | gds/src/applications/algorithms/community/modularity.rs | estimate_memory |
| community | scc | gds/src/applications/algorithms/community/scc.rs | estimate_memory |
| community | triangleCount | gds/src/applications/algorithms/community/triangle_count.rs | estimate_memory |
| community | wcc | gds/src/applications/algorithms/community/wcc.rs | estimate_memory |
| miscellaneous | collapse_path | gds/src/applications/algorithms/miscellaneous/collapse_path.rs |  |
| miscellaneous | index_inverse | gds/src/applications/algorithms/miscellaneous/index_inverse.rs |  |
| miscellaneous | scale_properties | gds/src/applications/algorithms/miscellaneous/scale_properties.rs |  |
| miscellaneous | to_undirected | gds/src/applications/algorithms/miscellaneous/to_undirected.rs |  |
| pathfinding | allShortestPaths | gds/src/applications/algorithms/pathfinding/all_shortest_paths.rs |  |
| pathfinding | astar | gds/src/applications/algorithms/pathfinding/astar.rs | estimate |
| pathfinding | bellman_ford | gds/src/applications/algorithms/pathfinding/bellman_ford.rs | estimate |
| pathfinding | bfs | gds/src/applications/algorithms/pathfinding/bfs.rs | estimate |
| pathfinding | dag_longest_path | gds/src/applications/algorithms/pathfinding/dag_longest_path.rs |  |
| pathfinding | delta_stepping | gds/src/applications/algorithms/pathfinding/delta_stepping.rs | estimate |
| pathfinding | dfs | gds/src/applications/algorithms/pathfinding/dfs.rs | estimate |
| pathfinding | dijkstra | gds/src/applications/algorithms/pathfinding/dijkstra.rs | estimate |
| pathfinding | kspanningtree | gds/src/applications/algorithms/pathfinding/kspanningtree.rs |  |
| pathfinding | prize_collecting_steiner_tree | gds/src/applications/algorithms/pathfinding/prize_collecting_steiner_tree.rs |  |
| pathfinding | random_walk | gds/src/applications/algorithms/pathfinding/random_walk.rs |  |
| pathfinding | spanning_tree | gds/src/applications/algorithms/pathfinding/spanning_tree.rs |  |
| pathfinding | steiner_tree | gds/src/applications/algorithms/pathfinding/steiner_tree.rs |  |
| pathfinding | topological_sort | gds/src/applications/algorithms/pathfinding/topological_sort.rs |  |
| pathfinding | yens | gds/src/applications/algorithms/pathfinding/yens.rs | estimate |
| similarity | filtered_knn | gds/src/applications/algorithms/similarity/filtered_knn.rs |  |
| similarity | filtered_node_similarity | gds/src/applications/algorithms/similarity/filtered_node_similarity.rs |  |
| similarity | knn | gds/src/applications/algorithms/similarity/knn.rs |  |
| similarity | node_similarity | gds/src/applications/algorithms/similarity/node_similarity.rs |  |

## Procedure layer coverage (`estimate_memory()`)

Target: ensure every core procedure has an `estimate_memory()` and eliminate obvious placeholders where feasible.

| Group | Procedure file | Placeholder? |
|---|---|---:|
| centrality | gds/src/procedures/centrality/articulation_points.rs | no |
| centrality | gds/src/procedures/centrality/betweenness.rs | no |
| centrality | gds/src/procedures/centrality/bridges.rs | no |
| centrality | gds/src/procedures/centrality/celf.rs | no |
| centrality | gds/src/procedures/centrality/closeness.rs | no |
| centrality | gds/src/procedures/centrality/degree_centrality.rs | no |
| centrality | gds/src/procedures/centrality/harmonic.rs | no |
| centrality | gds/src/procedures/centrality/hits.rs | no |
| centrality | gds/src/procedures/centrality/pagerank.rs | no |
| community | gds/src/procedures/community/approx_max_kcut.rs | yes |
| community | gds/src/procedures/community/conductance.rs | yes |
| community | gds/src/procedures/community/k1coloring.rs | yes |
| community | gds/src/procedures/community/kcore.rs | yes |
| community | gds/src/procedures/community/label_propagation.rs | yes |
| community | gds/src/procedures/community/leiden.rs | yes |
| community | gds/src/procedures/community/local_clustering_coefficient.rs | yes |
| community | gds/src/procedures/community/louvain.rs | no |
| community | gds/src/procedures/community/modularity.rs | yes |
| community | gds/src/procedures/community/scc.rs | yes |
| community | gds/src/procedures/community/triangle_count.rs | yes |
| community | gds/src/procedures/community/wcc.rs | no |
| miscellaneous | gds/src/procedures/miscellaneous/scale_properties.rs | no |
| pathfinding | gds/src/procedures/pathfinding/all_shortest_paths.rs | no |
| pathfinding | gds/src/procedures/pathfinding/astar.rs | no |
| pathfinding | gds/src/procedures/pathfinding/bellman_ford.rs | no |
| pathfinding | gds/src/procedures/pathfinding/bfs.rs | no |
| pathfinding | gds/src/procedures/pathfinding/dag_longest_path.rs | no |
| pathfinding | gds/src/procedures/pathfinding/delta_stepping.rs | no |
| pathfinding | gds/src/procedures/pathfinding/dfs.rs | no |
| pathfinding | gds/src/procedures/pathfinding/dijkstra.rs | no |
| pathfinding | gds/src/procedures/pathfinding/prize_collecting_steiner_tree.rs | no |
| pathfinding | gds/src/procedures/pathfinding/random_walk.rs | no |
| pathfinding | gds/src/procedures/pathfinding/spanning_tree.rs | no |
| pathfinding | gds/src/procedures/pathfinding/steiner_tree.rs | no |
| pathfinding | gds/src/procedures/pathfinding/topological_sort.rs | no |
| pathfinding | gds/src/procedures/pathfinding/yens.rs | no |

## Known placeholder sentinel

Currently treated as placeholder by the probe: `MemoryRange::of_range(0, 1024 * 1024)`.
