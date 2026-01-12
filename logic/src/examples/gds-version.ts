/**
 * Example: GDS kernel TS-JSON version
 *
 * Like `gds-ping.ts`, but exercises the `"version"` op.
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

export function versionGdsKernel(): void {
  const req = { op: 'version' };
  const requestJson = JSON.stringify(req);

  const out = execFileSync(
    'cargo',
    ['run', '-p', 'gds', '--bin', 'tsjson_cli', '--', requestJson],
    { cwd: repoRoot(), encoding: 'utf8' },
  );

  const parsed = GdsTsjsonResponseSchema.parse(JSON.parse(out));
  // eslint-disable-next-line no-console
  console.log('request:', req);
  // eslint-disable-next-line no-console
  console.log('response:', parsed);
}

if (import.meta.url === `file://${process.argv[1]}`) {
  versionGdsKernel();
}
