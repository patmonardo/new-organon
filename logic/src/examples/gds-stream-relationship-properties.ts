/**
 * Example: GraphStore Catalog facade (stream_relationship_properties)
 *
 * Seeds a tiny graph with relationship properties and streams them back as rows.
 */
/// <reference types="node" />

import { execFileSync } from 'node:child_process';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { GdsTsjsonResponseSchema } from '@schema';

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
