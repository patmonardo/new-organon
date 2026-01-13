/**
 * Example: Degree Centrality algorithm in estimate mode with memory submode
 *
 * Demonstrates running Degree Centrality in estimate mode with memory submode to get memory estimation.
 */

/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function degreeCentralityEstimateDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `degree-estimate-${Date.now()}`;

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
      op: 'degree_centrality',
      mode: 'estimate',
      submode: 'memory',
      user,
      databaseId,
      graphName,
      orientation: 'undirected',
      concurrency: 1,
    },
  ];

  // eslint-disable-next-line no-console
  console.log('Degree Centrality Estimate Request:');
  // eslint-disable-next-line no-console
  console.log(JSON.stringify(batch[1], null, 2));

  const resp = tsjsonInvoke(batch) as any[];

  // eslint-disable-next-line no-console
  console.log('\nDegree Centrality Estimate Response:');
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
  degreeCentralityEstimateDemo();
}
