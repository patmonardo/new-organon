# Estimation handler matrix (applications)

Date: 2026-01-02

This table scans `gds/src/applications/algorithms/**` request handlers for `mode` dispatch and records whether an estimation mode is exposed, and under which token(s).

| Group | Op | Handler | Supports estimation? | Token(s) |
|---|---|---|---:|---|
| centrality | articulation_points | gds/src/applications/algorithms/centrality/articulation_points.rs | yes | estimate_memory |
| centrality | betweenness | gds/src/applications/algorithms/centrality/betweenness.rs | yes | estimate_memory |
| centrality | bridges | gds/src/applications/algorithms/centrality/bridges.rs | yes | estimate_memory |
| centrality | celf | gds/src/applications/algorithms/centrality/celf.rs | yes | estimate_memory |
| centrality | closeness | gds/src/applications/algorithms/centrality/closeness.rs | yes | estimate_memory |
| centrality | degree_centrality | gds/src/applications/algorithms/centrality/degree_centrality.rs | yes | estimate_memory |
| centrality | harmonic | gds/src/applications/algorithms/centrality/harmonic.rs | yes | estimate_memory |
| centrality | hits | gds/src/applications/algorithms/centrality/hits.rs | yes | estimate_memory |
| centrality | pagerank | gds/src/applications/algorithms/centrality/pagerank.rs | yes | estimate_memory |
| community | approxMaxKCut | gds/src/applications/algorithms/community/approx_max_kcut.rs | no |  |
| community | conductance | gds/src/applications/algorithms/community/conductance.rs | yes | estimate_memory |
| community | k1coloring | gds/src/applications/algorithms/community/k1coloring.rs | yes | estimate_memory |
| community | kcore | gds/src/applications/algorithms/community/kcore.rs | yes | estimate_memory |
| community | kmeans | gds/src/applications/algorithms/community/kmeans.rs | no |  |
| community | labelPropagation | gds/src/applications/algorithms/community/label_propagation.rs | yes | estimate_memory |
| community | leiden | gds/src/applications/algorithms/community/leiden.rs | no |  |
| community | localClusteringCoefficient | gds/src/applications/algorithms/community/local_clustering_coefficient.rs | yes | estimate_memory |
| community | louvain | gds/src/applications/algorithms/community/louvain.rs | yes | estimate_memory |
| community | modularity | gds/src/applications/algorithms/community/modularity.rs | yes | estimate_memory |
| community | scc | gds/src/applications/algorithms/community/scc.rs | yes | estimate_memory |
| community | triangleCount | gds/src/applications/algorithms/community/triangle_count.rs | yes | estimate_memory |
| community | wcc | gds/src/applications/algorithms/community/wcc.rs | yes | estimate_memory |
| embeddings | fast_rp | gds/src/applications/algorithms/embeddings/fast_rp.rs | no |  |
| embeddings | gat | gds/src/applications/algorithms/embeddings/gat.rs | no |  |
| embeddings | graphsage | gds/src/applications/algorithms/embeddings/graphsage.rs | no |  |
| embeddings | hash_gnn | gds/src/applications/algorithms/embeddings/hash_gnn.rs | no |  |
| embeddings | node2vec | gds/src/applications/algorithms/embeddings/node2vec.rs | no |  |
| machine_learning | kge | gds/src/applications/algorithms/machine_learning/kge.rs | no | explicitly unsupported |
| machine_learning | split_relationships | gds/src/applications/algorithms/machine_learning/split_relationships.rs | no | explicitly unsupported |
| miscellaneous | collapse_path | gds/src/applications/algorithms/miscellaneous/collapse_path.rs | no |  |
| miscellaneous | index_inverse | gds/src/applications/algorithms/miscellaneous/index_inverse.rs | no |  |
| miscellaneous | scale_properties | gds/src/applications/algorithms/miscellaneous/scale_properties.rs | no |  |
| miscellaneous | to_undirected | gds/src/applications/algorithms/miscellaneous/to_undirected.rs | no |  |
| pathfinding | allShortestPaths | gds/src/applications/algorithms/pathfinding/all_shortest_paths.rs | no |  |
| pathfinding | astar | gds/src/applications/algorithms/pathfinding/astar.rs | yes | estimate |
| pathfinding | bellman_ford | gds/src/applications/algorithms/pathfinding/bellman_ford.rs | yes | estimate |
| pathfinding | bfs | gds/src/applications/algorithms/pathfinding/bfs.rs | yes | estimate |
| pathfinding | dag_longest_path | gds/src/applications/algorithms/pathfinding/dag_longest_path.rs | no |  |
| pathfinding | delta_stepping | gds/src/applications/algorithms/pathfinding/delta_stepping.rs | yes | estimate |
| pathfinding | dfs | gds/src/applications/algorithms/pathfinding/dfs.rs | yes | estimate |
| pathfinding | dijkstra | gds/src/applications/algorithms/pathfinding/dijkstra.rs | yes | estimate |
| pathfinding | kspanningtree | gds/src/applications/algorithms/pathfinding/kspanningtree.rs | no |  |
| pathfinding | prize_collecting_steiner_tree | gds/src/applications/algorithms/pathfinding/prize_collecting_steiner_tree.rs | no |  |
| pathfinding | random_walk | gds/src/applications/algorithms/pathfinding/random_walk.rs | no |  |
| pathfinding | spanning_tree | gds/src/applications/algorithms/pathfinding/spanning_tree.rs | no |  |
| pathfinding | steiner_tree | gds/src/applications/algorithms/pathfinding/steiner_tree.rs | no |  |
| pathfinding | topological_sort | gds/src/applications/algorithms/pathfinding/topological_sort.rs | no |  |
| pathfinding | yens | gds/src/applications/algorithms/pathfinding/yens.rs | yes | estimate |
| similarity | filtered_knn | gds/src/applications/algorithms/similarity/filtered_knn.rs | no |  |
| similarity | filtered_node_similarity | gds/src/applications/algorithms/similarity/filtered_node_similarity.rs | no |  |
| similarity | knn | gds/src/applications/algorithms/similarity/knn.rs | no |  |
| similarity | node_similarity | gds/src/applications/algorithms/similarity/node_similarity.rs | no |  |

## Summary

- centrality: 9 expose estimation, 0 do not
- community: 10 expose estimation, 3 do not
- embeddings: 0 expose estimation, 5 do not
- machine_learning: 0 expose estimation, 2 do not
- miscellaneous: 0 expose estimation, 4 do not
- pathfinding: 7 expose estimation, 8 do not
- similarity: 0 expose estimation, 4 do not

## Token distribution

- estimate_memory: 19 handlers
- estimate: 7 handlers
