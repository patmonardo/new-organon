/**
 * Example: Dijkstra shortest path via Algorithms ApplicationForm (dijkstra_stream)
 *
 * - Seeds a weighted graph
 * - Runs Dijkstra via `facade: "algorithms"`, `op: "dijkstra_stream"`
 * - Prints streamed path rows with costs
 */
/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function dijkstraStreamDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `dijkstra-${Date.now()}`;

  // Weighted graph:
  //   0 --1.0--> 1 --2.0--> 3
  //   |          |
  //  5.0        1.0
  //   |          |
  //   v          v
  //   2 --1.0--> 4
  //
  // Shortest 0→3: 0→1→3 (cost 3.0)
  // Shortest 0→4: 0→1→4 (cost 2.0)
  const batch = [
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user,
      databaseId,
      graphName,
      snapshot: {
        nodes: [0, 1, 2, 3, 4],
        relationships: [
          { type: 'ROAD', source: 0, target: 1, properties: { weight: 1.0 } },
          { type: 'ROAD', source: 1, target: 3, properties: { weight: 2.0 } },
          { type: 'ROAD', source: 0, target: 2, properties: { weight: 5.0 } },
          { type: 'ROAD', source: 1, target: 4, properties: { weight: 1.0 } },
          { type: 'ROAD', source: 2, target: 4, properties: { weight: 1.0 } },
        ],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'algorithms',
      op: 'dijkstra',
      mode: 'stream',
      user,
      databaseId,
      graphName,
      sourceNode: 0,
      targetNode: 3, // Single target
      weightProperty: 'weight',
      direction: 'outgoing',
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
  dijkstraStreamDemo();
}
