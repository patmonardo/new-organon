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

import { tsjsonInvoke } from './_tsjson';

export function pingGdsKernel(): void {
  const req = { op: 'ping', nonce: `n-${Date.now()}` };
  const parsed = tsjsonInvoke(req);
  // eslint-disable-next-line no-console
  console.log('request:', req);
  // eslint-disable-next-line no-console
  console.log('response:', parsed);
}

if (import.meta.url === `file://${process.argv[1]}`) {
  pingGdsKernel();
}
