/**
 * Example: PageRank algorithm in estimate mode with memory submode
 *
 * Demonstrates running PageRank in estimate mode with memory submode to get memory estimation.
 */

/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function pagerankEstimateDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `pagerank-estimate-${Date.now()}`;

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
      mode: 'estimate',
      estimateSubmode: 'memory',
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
  console.log('PageRank Estimate Request:');
  // eslint-disable-next-line no-console
  console.log(JSON.stringify(batch[1], null, 2));

  const resp = tsjsonInvoke(batch) as any[];

  // eslint-disable-next-line no-console
  console.log('\nPageRank Estimate Response:');
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
  pagerankEstimateDemo();
}
