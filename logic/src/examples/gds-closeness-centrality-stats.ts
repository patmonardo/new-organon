/**
 * Example: Closeness Centrality algorithm in stats mode
 *
 * Demonstrates running Closeness Centrality in stats mode to get execution statistics.
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

export function closenessCentralityStatsDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `closeness-stats-${Date.now()}`;

  const batch = [
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user,
      databaseId,
      graphName,
      snapshot: {
        nodes: [0, 1, 2, 3, 4],
        relationships: [
          { type: 'CONNECTS', source: 0, target: 1 },
          { type: 'CONNECTS', source: 1, target: 2 },
          { type: 'CONNECTS', source: 2, target: 3 },
          { type: 'CONNECTS', source: 1, target: 3 },
          { type: 'CONNECTS', source: 3, target: 4 },
          { type: 'CONNECTS', source: 0, target: 4 },
        ],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'algorithms',
      op: 'closeness',
      mode: 'stats',
      user,
      databaseId,
      graphName,
      concurrency: 1,
    },
  ];

  // eslint-disable-next-line no-console
  console.log('Closeness Centrality Stats Request:');
  // eslint-disable-next-line no-console
  console.log(JSON.stringify(batch[1], null, 2));

  const resp = tsjsonInvoke(batch) as any[];

  // eslint-disable-next-line no-console
  console.log('\nCloseness Centrality Stats Response:');
  const statsResp = resp[1];
  if (statsResp && statsResp.ok) {
    // eslint-disable-next-line no-console
    console.log(JSON.stringify(statsResp.data, null, 2));
  } else {
    // eslint-disable-next-line no-console
    console.error('Error:', statsResp?.error);
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  closenessCentralityStatsDemo();
}
