/**
 * Example: GraphStore Catalog facade (graph_exists)
 *
 * Demonstrates the tiny predicate op we’ll use everywhere for Java parity validations.
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


