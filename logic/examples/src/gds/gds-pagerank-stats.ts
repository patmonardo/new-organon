/**
 * Example: PageRank algorithm in stats mode
 *
 * Demonstrates running PageRank in stats mode to get computation statistics.
 */

/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function pagerankStatsDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `pagerank-stats-${Date.now()}`;

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
          { type: 'FOLLOWS', source: 0, target: 1 },
          { type: 'FOLLOWS', source: 1, target: 2 },
          { type: 'FOLLOWS', source: 2, target: 0 },
          { type: 'FOLLOWS', source: 1, target: 3 },
          { type: 'FOLLOWS', source: 3, target: 2 },
        ],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'algorithms',
      op: 'pagerank',
      mode: 'stats',
      user,
      databaseId,
      graphName,
      maxIterations: 20,
      dampingFactor: 0.85,
      tolerance: 0.0001,
      concurrency: 1,
    },
  ];

  // eslint-disable-next-line no-console
  console.log('PageRank Stats Request:');
  // eslint-disable-next-line no-console
  console.log(JSON.stringify(batch[1], null, 2));

  const resp = tsjsonInvoke(batch) as any[];

  // eslint-disable-next-line no-console
  console.log('\nPageRank Stats Response:');
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
  pagerankStatsDemo();
}
