/**
 * Example: Algorithms facade (pagerank_stream)
 *
 * - Seeds a tiny graph
 * - Runs PageRank via ApplicationForm `facade: "algorithms"`
 * - Prints per-node scores (mapped back to original node ids)
 */
/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function pagerankStreamDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `pr-${Date.now()}`;

  const batch = [
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user,
      databaseId,
      graphName,
      snapshot: {
        nodes: [1, 2, 3],
        relationships: [
          { type: 'KNOWS', source: 1, target: 2, properties: { weight: 1.0 } },
          { type: 'KNOWS', source: 2, target: 3, properties: { weight: 1.0 } },
          { type: 'KNOWS', source: 3, target: 1, properties: { weight: 1.0 } },
        ],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'algorithms',
      op: 'pagerank',
      mode: 'stream',
      user,
      databaseId,
      graphName,
      relationshipTypes: ['KNOWS'],
      // Wiring: select relationship property to act as "weight" (pagerank still unweighted today,
      // but this proves the selector path is connected).
      weightProperty: 'weight',
      config: { maxIterations: 20, dampingFactor: 0.85, tolerance: 1e-9 },
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
  pagerankStreamDemo();
}
