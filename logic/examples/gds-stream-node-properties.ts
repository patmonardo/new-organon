/**
 * Example: GraphStore Catalog facade (stream_node_properties)
 *
 * Seeds a tiny graph with numeric node properties and streams them back as rows.
 */
/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function streamNodePropertiesDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `nodeprops-${Date.now()}`;

  const batch = [
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user,
      databaseId,
      graphName,
      snapshot: {
        nodes: [100, 101, 102],
        relationships: [{ type: 'KNOWS', source: 100, target: 101 }],
        nodeProperties: {
          score: [1, 2, 3],
          weight: [0.1, 0.2, 0.3],
        },
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'graph_store_catalog',
      op: 'streamNodeProperties',
      user,
      databaseId,
      graphName,
      nodeProperties: ['score', 'weight'],
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
  streamNodePropertiesDemo();
}
