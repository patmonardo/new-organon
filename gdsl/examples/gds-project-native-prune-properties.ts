/**
 * Example: GraphStore Catalog facade (project_native) with property pruning
 *
 * Demonstrates that projection can *physically prune* property stores in the projected graph.
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

export function projectNativePrunePropertiesDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const sourceGraphName = `srcprops-${Date.now()}`;
  const projectedGraphName = `projprops-${Date.now()}`;

  const batch = [
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user,
      databaseId,
      graphName: sourceGraphName,
      snapshot: {
        nodes: [10, 11, 12],
        relationships: [
          { type: 'KNOWS', source: 10, target: 11, properties: { weight: 1.5, hops: 1 } },
          { type: 'KNOWS', source: 11, target: 12, properties: { weight: 2.25, hops: 2 } },
        ],
        nodeProperties: {
          score: [1, 2, 3],
          weight: [0.1, 0.2, 0.3],
        },
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'graph_store_catalog',
      op: 'subGraphProject',
      user,
      databaseId,
      graphName: projectedGraphName,
      originGraphName: sourceGraphName,
      nodeFilter: '*',
      relationshipFilter: 'KNOWS',
      configuration: {},
    },
    {
      kind: 'ApplicationForm',
      facade: 'graph_store_catalog',
      op: 'streamNodeProperties',
      user,
      databaseId,
      graphName: projectedGraphName,
      nodeProperties: ['score', 'weight'],
      listNodeLabels: false,
      nodeLabels: ['*'],
    },
    {
      kind: 'ApplicationForm',
      facade: 'graph_store_catalog',
      op: 'streamRelationshipProperties',
      user,
      databaseId,
      graphName: projectedGraphName,
      relationshipTypes: ['KNOWS'],
      relationshipProperties: ['weight', 'hops'],
    },
  ];

  const resp = tsjsonInvoke(batch) as any[];
  // eslint-disable-next-line no-console
  console.log('batch.response:', resp);

  const nodeRows = resp?.[2]?.data?.rows ?? [];
  const relRows = resp?.[3]?.data?.rows ?? [];
  // eslint-disable-next-line no-console
  console.dir({ sourceGraphName, projectedGraphName, nodeRows, relRows }, { depth: null });
}

if (import.meta.url === `file://${process.argv[1]}`) {
  projectNativePrunePropertiesDemo();
}


