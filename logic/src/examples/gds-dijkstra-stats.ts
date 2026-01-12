/**
 * Example: Dijkstra algorithm in stats mode
 *
 * Demonstrates running Dijkstra in stats mode to get execution statistics.
 */

/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function dijkstraStatsDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `dijkstra-stats-${Date.now()}`;

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
          {
            type: 'CONNECTS',
            source: 0,
            target: 1,
            properties: { weight: 1.0 },
          },
          {
            type: 'CONNECTS',
            source: 1,
            target: 2,
            properties: { weight: 2.0 },
          },
          {
            type: 'CONNECTS',
            source: 2,
            target: 3,
            properties: { weight: 1.0 },
          },
          {
            type: 'CONNECTS',
            source: 1,
            target: 3,
            properties: { weight: 4.0 },
          },
          {
            type: 'CONNECTS',
            source: 3,
            target: 4,
            properties: { weight: 2.0 },
          },
          {
            type: 'CONNECTS',
            source: 0,
            target: 4,
            properties: { weight: 5.0 },
          },
        ],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'algorithms',
      op: 'dijkstra',
      mode: 'stats',
      user,
      databaseId,
      graphName,
      sourceNode: 0,
      weightProperty: 'weight',
      concurrency: 1,
    },
  ];

  // eslint-disable-next-line no-console
  console.log('Dijkstra Stats Request:');
  // eslint-disable-next-line no-console
  console.log(JSON.stringify(batch[1], null, 2));

  const resp = tsjsonInvoke(batch) as any[];

  // eslint-disable-next-line no-console
  console.log('\nDijkstra Stats Response:');
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
  dijkstraStatsDemo();
}
