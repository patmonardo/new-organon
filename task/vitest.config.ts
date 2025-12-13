import { defineConfig } from 'vitest/config';
import tsconfig from './tsconfig.json';
import path from 'path';

// Create an alias object from the paths in tsconfig.json
const tsPaths = (tsconfig as any)?.compilerOptions?.paths as
  | Record<string, string[]>
  | undefined;

const alias = Object.fromEntries(
  // For Each Path in tsconfig.json
  Object.entries(tsPaths ?? {}).map(([key, values]) => {
    const value = values?.[0];
    if (!value) return null;
    // Remove the "/*" from the key and resolve the path
    return [
      key.replace('/*', ''),
      // Remove the "/*" from the value Resolve the relative path
      path.resolve(__dirname, value.replace('/*', '')),
    ] as const;
  }).filter((x): x is readonly [string, string] => x !== null),
);

export default defineConfig({
  resolve: {
    alias,
  },
  test: {
    environment: 'node',
    include: ['test/**/*.test.ts', 'test/**/*.spec.ts'],
    // exclude: ['node_modules', 'dist'],
    globals: true,
  },
});
