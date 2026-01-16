/**
 * Example: BFS algorithm in estimate mode
 *
 * Demonstrates running BFS in estimate mode to get memory estimation
 * without actually executing the algorithm.
 */

/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function bfsEstimateDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `bfs-estimate-${Date.now()}`;

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
      mode: 'estimate',
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
  console.log('BFS Estimate Request:');
  // eslint-disable-next-line no-console
  console.log(JSON.stringify(batch[1], null, 2));

  const resp = tsjsonInvoke(batch) as any[];

  // eslint-disable-next-line no-console
  console.log('\nBFS Estimate Response:');
  const estimateResp = resp[1];
  if (estimateResp && estimateResp.ok) {
    // eslint-disable-next-line no-console
    console.log(JSON.stringify(estimateResp.data, null, 2));
  } else {
    // eslint-disable-next-line no-console
    console.error('Error:', estimateResp?.error);
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  bfsEstimateDemo();
}
