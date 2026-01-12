/**
 * Example: GraphStore Catalog facade (list_graphs)
 *
 * This stays **Rajasic / GDS-L**: we exercise a single real facade op using the
 * TS-JSON boundary against the Rust kernel CLI (`tsjson_cli`).
 *
 * Flow:
 * - graph_store.put (seed a tiny graph into the shared catalog)
 * - graph_store_catalog.list_graphs (observe the catalog entries)
 */

/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function listGraphsDemo(): void {
  const user = { username: 'alice', isAdmin: true };

  const putReq = {
    kind: 'ApplicationForm',
    facade: 'graph_store',
    op: 'put',
    user,
    databaseId: 'db1',
    graphName: `demo-${Date.now()}`,
    snapshot: {
      nodes: [0, 1, 2],
      relationships: [
        { type: 'KNOWS', source: 0, target: 1 },
        { type: 'KNOWS', source: 1, target: 2 },
      ],
    },
  };

  const listReq = {
    kind: 'ApplicationForm',
    facade: 'graph_store_catalog',
    op: 'listGraphs',
    user,
    databaseId: 'db1',
    graphName: putReq.graphName,
    includeDegreeDistribution: true,
  };

  // Batch both calls in one process so the in-memory catalog persists across ops.
  const batch = [putReq, listReq];

  // eslint-disable-next-line no-console
  console.log('batch.request:', batch);
  const resp = tsjsonInvoke(batch);
  // eslint-disable-next-line no-console
  console.log('batch.response:', resp);

  // Pretty-print the listGraphs entries, including optional degreeDistribution.
  const listResp = Array.isArray(resp) ? resp[1] : resp;
  if (
    listResp &&
    (listResp as any).ok &&
    (listResp as any).op === 'listGraphs'
  ) {
    const entries = (listResp as any).data?.entries;
    // eslint-disable-next-line no-console
    console.dir({ entries }, { depth: null });
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  listGraphsDemo();
}
