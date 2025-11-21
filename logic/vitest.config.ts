import { defineConfig } from 'vitest/config';
import tsconfig from './tsconfig.json';
import path from 'path';

// Defensive: gracefully handle missing/empty paths
const rawPaths = tsconfig.compilerOptions?.paths ?? {};
const alias = Object.fromEntries(
  Object.entries(rawPaths)
    .filter(([k, v]) => !!k && Array.isArray(v) && typeof v[0] === 'string')
    .map(([key, value]) => [
      key.replace(/\/\*$/, ''), // strip wildcard
      path.resolve(__dirname, (value as string[])[0].replace(/\/\*$/, '')),
    ]),
);

if (Object.keys(alias).length === 0) {
  // optional: small debug line so CI/editor logs the fact there are no aliases
  // console.info('vitest: no tsconfig paths found — running without aliases');
}

export default defineConfig({
  resolve: { alias },
  test: {
    environment: 'node',
    include: ['test/**/*.test.ts', 'test/**/*.spec.ts'],
    exclude: ['test/absolute', 'test/relative'],
    globals: true,
  },
});
