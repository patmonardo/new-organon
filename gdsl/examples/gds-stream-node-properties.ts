/**
 * Example: GraphStore Catalog facade (stream_node_properties)
 *
 * Seeds a tiny graph with numeric node properties and streams them back as rows.
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
      op: 'stream_node_properties',
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


