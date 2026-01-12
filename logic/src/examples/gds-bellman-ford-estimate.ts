/**
 * Example: Bellman-Ford algorithm in estimate mode
 *
 * Demonstrates running Bellman-Ford in estimate mode to get memory estimation.
 */

/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function bellmanFordEstimateDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `bellman-ford-estimate-${Date.now()}`;

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
      op: 'bellman_ford',
      mode: 'estimate',
      user,
      databaseId,
      graphName,
      sourceNode: 0,
      weightProperty: 'weight',
      concurrency: 1,
    },
  ];

  // eslint-disable-next-line no-console
  console.log('Bellman-Ford Estimate Request:');
  // eslint-disable-next-line no-console
  console.log(JSON.stringify(batch[1], null, 2));

  const resp = tsjsonInvoke(batch) as any[];

  // eslint-disable-next-line no-console
  console.log('\nBellman-Ford Estimate Response:');
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
  bellmanFordEstimateDemo();
}
