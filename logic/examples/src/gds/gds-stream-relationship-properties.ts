/**
 * Example: GraphStore Catalog facade (stream_relationship_properties)
 *
 * Seeds a tiny graph with relationship properties and streams them back as rows.
 */
/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function streamRelationshipPropertiesDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `relprops-${Date.now()}`;

  const batch = [
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user,
      databaseId,
      graphName,
      snapshot: {
        nodes: [0, 1, 2],
        relationships: [
          {
            type: 'KNOWS',
            source: 0,
            target: 1,
            properties: { weight: 1.5, hops: 1 },
          },
          {
            type: 'KNOWS',
            source: 1,
            target: 2,
            properties: { weight: 2.25, hops: 2 },
          },
          {
            type: 'LIKES',
            source: 0,
            target: 2,
            properties: { weight: 0.5, hops: 1 },
          },
        ],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'graph_store_catalog',
      op: 'streamRelationshipProperties',
      user,
      databaseId,
      graphName,
      // Java parity knob: filter by relationship types (supports "*" for all).
      relationshipTypes: ['KNOWS'],
      relationshipProperties: ['weight', 'hops'],
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
  streamRelationshipPropertiesDemo();
}
