/**
 * Example: Bellman-Ford shortest path via Algorithms ApplicationForm (bellman_ford_stream)
 *
 * - Seeds a weighted graph (including negative weights)
 * - Runs Bellman-Ford via `facade: "algorithms"`, `op: "bellman_ford_stream"`
 * - Prints streamed path rows with costs
 *
 * Bellman-Ford handles negative edge weights (unlike Dijkstra).
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

export function bellmanFordStreamDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `bellman-ford-${Date.now()}`;

  // Graph with a negative-weight edge:
  //   0 --1.0--> 1 --(-2.0)--> 2 --1.0--> 3
  //
  // Shortest 0→3: 0→1→2→3 (cost: 1 + (-2) + 1 = 0)
  const batch = [
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user,
      databaseId,
      graphName,
      snapshot: {
        nodes: [0, 1, 2, 3],
        relationships: [
          { type: 'ROAD', source: 0, target: 1, properties: { weight: 1.0 } },
          { type: 'ROAD', source: 1, target: 2, properties: { weight: -2.0 } },
          { type: 'ROAD', source: 2, target: 3, properties: { weight: 1.0 } },
        ],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'algorithms',
      op: 'bellman_ford',
      mode: 'stream',
      user,
      databaseId,
      graphName,
      sourceNode: 0,
      weightProperty: 'weight',
      direction: 'outgoing',
      trackNegativeCycles: true,
      trackPaths: true,
      concurrency: 1,
    },
  ];

  const resp = tsjsonInvoke(batch) as any[];
  // eslint-disable-next-line no-console
  console.log('batch.response:', resp);

  const result = resp?.[1]?.data ?? {};
  // eslint-disable-next-line no-console
  console.dir(result, { depth: null });
}

if (import.meta.url === `file://${process.argv[1]}`) {
  bellmanFordStreamDemo();
}
