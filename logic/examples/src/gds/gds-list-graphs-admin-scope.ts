/**
 * Example: per-user/per-db catalog scoping + admin merged view
 *
 * This demonstrates the current **mock Java parity** behavior:
 * - Non-admin users see only their own (username, databaseId) catalog.
 * - Admin users get a merged per-db view (can see other users' graphs on that database).
 */
/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function adminScopeDemo(): void {
  const db1 = 'db1';
  const alice = { username: 'alice', isAdmin: false };
  const bob = { username: 'bob', isAdmin: false };
  const admin = { username: 'root', isAdmin: true };

  const aliceGraph = `alice-${Date.now()}`;
  const bobGraph = `bob-${Date.now()}`;

  const batch = [
    // Alice seeds a graph into her catalog scope.
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user: alice,
      databaseId: db1,
      graphName: aliceGraph,
      snapshot: {
        nodes: [0, 1],
        relationships: [{ type: 'KNOWS', source: 0, target: 1 }],
      },
    },
    // Bob seeds a different graph into his scope.
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user: bob,
      databaseId: db1,
      graphName: bobGraph,
      snapshot: {
        nodes: [0, 1, 2],
        relationships: [{ type: 'KNOWS', source: 0, target: 1 }],
      },
    },
    // Alice lists: should only see aliceGraph.
    {
      kind: 'ApplicationForm',
      facade: 'graph_store_catalog',
      op: 'listGraphs',
      user: alice,
      databaseId: db1,
    },
    // Bob lists: should only see bobGraph.
    {
      kind: 'ApplicationForm',
      facade: 'graph_store_catalog',
      op: 'listGraphs',
      user: bob,
      databaseId: db1,
    },
    // Admin lists: should see both (merged per-db view).
    {
      kind: 'ApplicationForm',
      facade: 'graph_store_catalog',
      op: 'listGraphs',
      user: admin,
      databaseId: db1,
    },
  ];

  // eslint-disable-next-line no-console
  console.log('batch.request:', batch);
  const resp = tsjsonInvoke(batch) as any[];
  // eslint-disable-next-line no-console
  console.log('batch.response:', resp);

  const aliceEntries = resp[2]?.data?.entries ?? [];
  const bobEntries = resp[3]?.data?.entries ?? [];
  const adminEntries = resp[4]?.data?.entries ?? [];

  // eslint-disable-next-line no-console
  console.dir(
    {
      aliceEntries: aliceEntries.map((e: any) => e.name),
      bobEntries: bobEntries.map((e: any) => e.name),
      adminEntries: adminEntries.map((e: any) => e.name),
    },
    { depth: null },
  );
}

if (import.meta.url === `file://${process.argv[1]}`) {
  adminScopeDemo();
}
