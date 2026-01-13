/**
 * Example: GDS kernel TS-JSON version
 *
 * Like `gds-ping.ts`, but exercises the `"version"` op.
 */

/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function versionGdsKernel(): void {
  const req = { op: 'version' };
  const parsed = tsjsonInvoke(req);
  // eslint-disable-next-line no-console
  console.log('request:', req);
  // eslint-disable-next-line no-console
  console.log('response:', parsed);
}

if (import.meta.url === `file://${process.argv[1]}`) {
  versionGdsKernel();
}
