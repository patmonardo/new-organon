/**
 * Example: Pathfinding facade via Algorithms ApplicationForm (bfs_stream)
 *
 * - Seeds a tiny graph
 * - Runs BFS via `facade: "algorithms"`, `op: "bfs_stream"`
 * - Prints streamed path rows
 */
/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function bfsStreamDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `bfs-${Date.now()}`;

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
          { type: 'REL', source: 0, target: 1 },
          { type: 'REL', source: 1, target: 2 },
          { type: 'REL', source: 2, target: 3 },
          { type: 'REL', source: 1, target: 4 },
        ],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'algorithms',
      op: 'bfs',
      mode: 'stream',
      user,
      databaseId,
      graphName,
      sourceNode: 0,
      targetNodes: [3],
      maxDepth: 10,
      trackPaths: true,
      concurrency: 1,
      delta: 64,
    },
  ];

  const resp = tsjsonInvoke(batch) as any[];
  // eslint-disable-next-line no-console
  console.log('batch.response:', resp);

  const rows = resp?.[1]?.data?.rows ?? [];
  // eslint-disable-next-line no-console
  console.dir({ graphName, rows }, { depth: null });
}

if (import.meta.url === `file://${process.argv[1]}`) {
  bfsStreamDemo();
}
