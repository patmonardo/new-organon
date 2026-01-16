/**
 * Example: A* shortest path via Algorithms ApplicationForm (astar_stream)
 *
 * - Seeds a weighted graph
 * - Runs A* via `facade: "algorithms"`, `op: "astar_stream"`
 * - Prints streamed path rows with costs
 *
 * A* uses heuristic guidance to find optimal paths faster than Dijkstra.
 */
/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function astarStreamDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `astar-${Date.now()}`;

  // Weighted graph (simple grid-like structure for A* demonstration):
  //   0 --1.0--> 1 --1.0--> 2
  //   |          |          |
  //  2.0        2.0        2.0
  //   |          |          |
  //   v          v          v
  //   3 --1.0--> 4 --1.0--> 5
  //
  // Shortest 0→5: 0→1→2→5 (cost 4.0) or 0→1→4→5 (cost 4.0)
  const batch = [
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user,
      databaseId,
      graphName,
      snapshot: {
        nodes: [0, 1, 2, 3, 4, 5],
        relationships: [
          // Row 1
          { type: 'ROAD', source: 0, target: 1, properties: { weight: 1.0 } },
          { type: 'ROAD', source: 1, target: 2, properties: { weight: 1.0 } },
          // Down from row 1
          { type: 'ROAD', source: 0, target: 3, properties: { weight: 2.0 } },
          { type: 'ROAD', source: 1, target: 4, properties: { weight: 2.0 } },
          { type: 'ROAD', source: 2, target: 5, properties: { weight: 2.0 } },
          // Row 2
          { type: 'ROAD', source: 3, target: 4, properties: { weight: 1.0 } },
          { type: 'ROAD', source: 4, target: 5, properties: { weight: 1.0 } },
        ],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'algorithms',
      op: 'astar',
      mode: 'stream',
      user,
      databaseId,
      graphName,
      sourceNode: 0,
      targetNode: 5,
      weightProperty: 'weight',
      heuristic: 'manhattan',
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
  astarStreamDemo();
}
