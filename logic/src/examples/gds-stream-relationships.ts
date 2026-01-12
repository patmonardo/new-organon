/**
 * Example: GraphStore Catalog facade (stream_relationships)
 *
 * Seeds a tiny graph and then streams topology rows (Java parity `TopologyResult`).
 */
/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function streamRelationshipsDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `stream-${Date.now()}`;

  const batch = [
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user,
      databaseId,
      graphName,
      snapshot: {
        nodes: [10, 11, 12],
        relationships: [
          { type: 'KNOWS', source: 10, target: 11 },
          { type: 'KNOWS', source: 11, target: 12 },
          { type: 'LIKES', source: 10, target: 12 },
        ],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'graph_store_catalog',
      op: 'streamRelationships',
      user,
      databaseId,
      graphName,
      relationshipTypes: ['KNOWS'],
    },
  ];

  const resp = tsjsonInvoke(batch) as any[];
  // eslint-disable-next-line no-console
  console.log('batch.response:', resp);

  const rows = resp?.[1]?.data?.relationships ?? [];
  // eslint-disable-next-line no-console
  console.dir({ graphName, rows }, { depth: null });
}

if (import.meta.url === `file://${process.argv[1]}`) {
  streamRelationshipsDemo();
}
