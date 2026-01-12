/**
 * Example: BFS algorithm in stats mode
 *
 * Demonstrates running BFS in stats mode to get computation statistics
 * instead of the actual path results.
 */

/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function bfsStatsDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `bfs-stats-${Date.now()}`;

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
      mode: 'stats',
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

  // eslint-disable-next-line no-console
  console.log('BFS Stats Request:');
  // eslint-disable-next-line no-console
  console.log(JSON.stringify(batch[1], null, 2));

  const resp = tsjsonInvoke(batch) as any[];

  // eslint-disable-next-line no-console
  console.log('\nBFS Stats Response:');
  const statsResp = resp[1];
  if (statsResp && statsResp.ok) {
    // eslint-disable-next-line no-console
    console.log(JSON.stringify(statsResp.data, null, 2));
  } else {
    // eslint-disable-next-line no-console
    console.error('Error:', statsResp?.error);
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  bfsStatsDemo();
}
