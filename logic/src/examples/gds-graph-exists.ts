/**
 * Example: GraphStore Catalog facade (graph_exists)
 *
 * Demonstrates the tiny predicate op we’ll use everywhere for Java parity validations.
 */
/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function graphExistsDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `exists-${Date.now()}`;

  // Batch in one process so the in-memory catalog persists across ops.
  const batch = [
    {
      kind: 'ApplicationForm',
      facade: 'graph_store_catalog',
      op: 'graphExists',
      user,
      databaseId,
      graphName,
    },
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user,
      databaseId,
      graphName,
      snapshot: {
        nodes: [0, 1],
        relationships: [{ type: 'KNOWS', source: 0, target: 1 }],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'graph_store_catalog',
      op: 'graphExists',
      user,
      databaseId,
      graphName,
    },
  ];

  const resp = tsjsonInvoke(batch) as any[];
  // eslint-disable-next-line no-console
  console.dir(
    {
      before: resp[0],
      put: resp[1],
      after: resp[2],
    },
    { depth: null },
  );
}

if (import.meta.url === `file://${process.argv[1]}`) {
  graphExistsDemo();
}
