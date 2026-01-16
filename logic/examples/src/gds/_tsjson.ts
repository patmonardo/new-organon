import { execFileSync } from 'node:child_process';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { GdsTsjsonResponseSchema } from '@schema';

// Resolve workspace root from src/examples to repo root (../../..)
const WORKSPACE_ROOT = resolve(
  fileURLToPath(new URL('../..', import.meta.url)),
);

/**
 * Invoke the Rust TSJSON CLI and Zod-validate the response.
 */
export function tsjsonInvoke(req: unknown): unknown {
  const requestJson = JSON.stringify(req);
  const out = execFileSync(
    'cargo',
    ['run', '-p', 'gds', '--bin', 'tsjson_cli', '--', requestJson],
    { cwd: WORKSPACE_ROOT, encoding: 'utf8' },
  );
  const parsed = JSON.parse(out) as unknown;
  if (Array.isArray(parsed))
    return parsed.map((x) => GdsTsjsonResponseSchema.parse(x));
  return GdsTsjsonResponseSchema.parse(parsed);
}
