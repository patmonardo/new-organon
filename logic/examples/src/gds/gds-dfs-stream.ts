/**
 * Example: Pathfinding facade via Algorithms ApplicationForm (dfs_stream)
 *
 * - Seeds a tiny graph
 * - Runs DFS via `facade: "algorithms"`, `op: "dfs_stream"`
 * - Prints streamed path rows
 */
/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function dfsStreamDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `dfs-${Date.now()}`;

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
      op: 'dfs',
      mode: 'stream',
      user,
      databaseId,
      graphName,
      sourceNode: 0,
      targetNodes: [3],
      maxDepth: 10,
      trackPaths: true,
      concurrency: 1,
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
  dfsStreamDemo();
}
