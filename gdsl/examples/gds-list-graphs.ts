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

import { execFileSync } from 'node:child_process';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { GdsTsjsonResponseSchema } from '@organon/gdsl/schema';

function repoRoot(): string {
  const here = dirname(fileURLToPath(import.meta.url));
  return resolve(here, '../..');
}

function tsjsonInvoke(req: unknown): unknown {
  const requestJson = JSON.stringify(req);
  const out = execFileSync(
    'cargo',
    ['run', '-p', 'gds', '--bin', 'tsjson_cli', '--', requestJson],
    { cwd: repoRoot(), encoding: 'utf8' },
  );
  const parsed = JSON.parse(out) as unknown;
  if (Array.isArray(parsed)) {
    return parsed.map((x) => GdsTsjsonResponseSchema.parse(x));
  }
  return GdsTsjsonResponseSchema.parse(parsed);
}

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
    op: 'list_graphs',
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

  // Pretty-print the list_graphs entries, including optional degreeDistribution.
  const listResp = Array.isArray(resp) ? resp[1] : resp;
  if (listResp && (listResp as any).ok && (listResp as any).op === 'list_graphs') {
    const entries = (listResp as any).data?.entries;
    // eslint-disable-next-line no-console
    console.dir({ entries }, { depth: null });
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  listGraphsDemo();
}


