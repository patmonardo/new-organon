/**
 * Example: Closeness Centrality algorithm in stats mode
 *
 * Demonstrates running Closeness Centrality in stats mode to get execution statistics.
 */

/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function closenessCentralityStatsDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `closeness-stats-${Date.now()}`;

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
          { type: 'CONNECTS', source: 0, target: 1 },
          { type: 'CONNECTS', source: 1, target: 2 },
          { type: 'CONNECTS', source: 2, target: 3 },
          { type: 'CONNECTS', source: 1, target: 3 },
          { type: 'CONNECTS', source: 3, target: 4 },
          { type: 'CONNECTS', source: 0, target: 4 },
        ],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'algorithms',
      op: 'closeness',
      mode: 'stats',
      user,
      databaseId,
      graphName,
      concurrency: 1,
    },
  ];

  // eslint-disable-next-line no-console
  console.log('Closeness Centrality Stats Request:');
  // eslint-disable-next-line no-console
  console.log(JSON.stringify(batch[1], null, 2));

  const resp = tsjsonInvoke(batch) as any[];

  // eslint-disable-next-line no-console
  console.log('\nCloseness Centrality Stats Response:');
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
  closenessCentralityStatsDemo();
}
