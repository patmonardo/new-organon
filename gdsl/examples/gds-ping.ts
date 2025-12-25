/**
 * Example: GDS kernel TS-JSON ping
 *
 * GDSL should stay a thin “GDS Link” vocabulary. This example exercises the
 * lowest-level kernel boundary: `invoke(json)` → `{"ok", "op", "data|error"}`.
 *
 * It runs the Rust CLI wrapper (`gds/src/bin/tsjson_cli.rs`) so this can execute
 * against the real kernel without requiring a Node NAPI addon.
 */

/// <reference types="node" />

import { execFileSync } from 'node:child_process';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { GdsTsjsonResponseSchema } from '@organon/gdsl/schema';

function repoRoot(): string {
  // gdsl/examples → repo root
  const here = dirname(fileURLToPath(import.meta.url));
  return resolve(here, '../..');
}

export function pingGdsKernel(): void {
  const req = { op: 'ping', nonce: `n-${Date.now()}` };
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
  pingGdsKernel();
}


