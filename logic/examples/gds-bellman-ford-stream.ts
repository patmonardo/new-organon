/**
 * Example: Bellman-Ford shortest path via Algorithms ApplicationForm (bellman_ford_stream)
 *
 * - Seeds a weighted graph (including negative weights)
 * - Runs Bellman-Ford via `facade: "algorithms"`, `op: "bellman_ford_stream"`
 * - Prints streamed path rows with costs
 *
 * Bellman-Ford handles negative edge weights (unlike Dijkstra).
 */
/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function bellmanFordStreamDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `bellman-ford-${Date.now()}`;

  // Graph with a negative-weight edge:
  //   0 --1.0--> 1 --(-2.0)--> 2 --1.0--> 3
  //
  // Shortest 0→3: 0→1→2→3 (cost: 1 + (-2) + 1 = 0)
  const batch = [
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user,
      databaseId,
      graphName,
      snapshot: {
        nodes: [0, 1, 2, 3],
        relationships: [
          { type: 'ROAD', source: 0, target: 1, properties: { weight: 1.0 } },
          { type: 'ROAD', source: 1, target: 2, properties: { weight: -2.0 } },
          { type: 'ROAD', source: 2, target: 3, properties: { weight: 1.0 } },
        ],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'algorithms',
      op: 'bellman_ford',
      mode: 'stream',
      user,
      databaseId,
      graphName,
      sourceNode: 0,
      weightProperty: 'weight',
      direction: 'outgoing',
      trackNegativeCycles: true,
      trackPaths: true,
      concurrency: 1,
    },
  ];

  const resp = tsjsonInvoke(batch) as any[];
  // eslint-disable-next-line no-console
  console.log('batch.response:', resp);

  const result = resp?.[1]?.data ?? {};
  // eslint-disable-next-line no-console
  console.dir(result, { depth: null });
}

if (import.meta.url === `file://${process.argv[1]}`) {
  bellmanFordStreamDemo();
}
