import fs from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL, fileURLToPath } from 'node:url';

import { DialecticIRSchema, type DialecticIR } from '@schema/dialectic';

type ExportSpec = {
  key: string;
  exportName: string;
  moduleSpecifier: string;
  sourceFileAbs: string;
  sourceFileRelToSrc: string;
  localName: string;
  irId: string;
  title: string;
  section: string;
};

function isDialecticIR(value: unknown): value is DialecticIR {
  return DialecticIRSchema.safeParse(value).success;
}

function formatZodIssues(
  issues: Array<{ path: Array<string | number | symbol>; message: string }>
): string {
  return issues
    .map(i => `${i.path.map(String).join('.') || '<root>'}: ${i.message}`)
    .join('\n');
}

async function walk(dirAbs: string): Promise<string[]> {
  const out: string[] = [];
  const entries = await fs.readdir(dirAbs, { withFileTypes: true });
  for (const entry of entries) {
    const abs = path.join(dirAbs, entry.name);
    if (entry.isDirectory()) {
      out.push(...(await walk(abs)));
      continue;
    }
    if (entry.isFile() && entry.name.endsWith('-ir.ts')) out.push(abs);
  }
  return out;
}

function toModuleSpecifierFromSrcRelative(srcRelativePath: string): string {
  // src/relative/... => @relative/...
  const normalized = srcRelativePath.split(path.sep).join('/');
  const withoutPrefix = normalized.replace(/^relative\//, '');
  const withoutExt = withoutPrefix.replace(/\.ts$/, '');
  return `@relative/${withoutExt}`;
}

function toSafeIdentifier(raw: string): string {
  let ident = raw.replace(/[^A-Za-z0-9_]/g, '_');
  if (/^[0-9]/.test(ident)) ident = `_${ident}`;
  return ident;
}

async function main() {
  const here = path.dirname(fileURLToPath(import.meta.url));
  const packageRoot = path.resolve(here, '..', '..');
  const srcRoot = path.join(packageRoot, 'src');
  const relativeRoot = path.join(srcRoot, 'relative');

  const irFilesAbs = (await walk(relativeRoot)).sort();

  const exportSpecs: ExportSpec[] = [];

  for (const fileAbs of irFilesAbs) {
    const relToSrc = path.relative(srcRoot, fileAbs).split(path.sep).join('/');
    const relWithinRelative = relToSrc.replace(/^relative\//, '');
    const moduleSpecifier = toModuleSpecifierFromSrcRelative(relToSrc);

    const source = await fs.readFile(fileAbs, 'utf8');
    const exportRegex = /export\s+const\s+([A-Za-z0-9_]+IR)\s*:\s*DialecticIR\s*=/g;
    const exportNames: string[] = [];
    for (;;) {
      const match = exportRegex.exec(source);
      if (!match) break;
      exportNames.push(match[1]!);
    }

    if (exportNames.length === 0) {
      // Not necessarily an error, but this is unexpected for *-ir.ts.
      continue;
    }

    const mod = await import(pathToFileURL(fileAbs).href);

    for (const exportName of exportNames) {
      const value = (mod as any)[exportName] as unknown;
      const parsed = DialecticIRSchema.safeParse(value);
      if (!parsed.success) {
        throw new Error(
          `Invalid DialecticIR export ${exportName} in ${relToSrc} (failed DialecticIRSchema)\n${formatZodIssues(
            parsed.error.issues
          )}`
        );
      }

      const ir = parsed.data;

      const sphere = relWithinRelative.split('/')[0] ?? 'relative';
      const localName = toSafeIdentifier(
        `${sphere}_${relWithinRelative.replace(/\//g, '_').replace(/\.ts$/, '')}_${exportName}`
      );

      // Registry key must be globally unique; IR ids are allowed to collide across spheres.
      const key = `${moduleSpecifier}#${exportName}`;

      exportSpecs.push({
        key,
        exportName,
        moduleSpecifier,
        sourceFileAbs: fileAbs,
        sourceFileRelToSrc: relToSrc,
        localName,
        irId: ir.id,
        title: ir.title,
        section: ir.section,
      });
    }
  }

  // Ensure unique local names
  const byLocal = new Map<string, ExportSpec[]>();
  for (const spec of exportSpecs) {
    const list = byLocal.get(spec.localName) ?? [];
    list.push(spec);
    byLocal.set(spec.localName, list);
  }
  const dupLocal = [...byLocal.entries()].filter(([, specs]) => specs.length > 1);
  if (dupLocal.length > 0) {
    const detail = dupLocal
      .map(([name, specs]) => `${name}: ${specs.map(s => s.sourceFileRelToSrc).join(', ')}`)
      .join('\n');
    throw new Error(`Duplicate generated local names found:\n${detail}`);
  }

  exportSpecs.sort((a, b) => a.key.localeCompare(b.key));

  const outDir = path.join(srcRoot, 'relative', 'form', 'dialectic', 'generated');
  const outFile = path.join(outDir, 'registry.ts');
  await fs.mkdir(outDir, { recursive: true });

  const entries = exportSpecs
    .map(
      s =>
        `  { key: '${s.key}', id: '${s.irId}', title: ${JSON.stringify(
          s.title
        )}, section: ${JSON.stringify(s.section)}, module: '${s.moduleSpecifier}', export: '${s.exportName}' },`
    )
    .join('\n');

  const idIndexEntries = exportSpecs
    .reduce((acc, s) => {
      const list = acc.get(s.irId) ?? [];
      list.push(s.key);
      acc.set(s.irId, list);
      return acc;
    }, new Map<string, string[]>())
    .entries();

  const idIndex = [...idIndexEntries]
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([id, keys]) => `  ${JSON.stringify(id)}: ${JSON.stringify(keys.sort())},`)
    .join('\n');

  const content = `/*
 * GENERATED FILE — DO NOT EDIT.
 *
 * Source: src/tools/codegen-dialectic-registry.ts
 *
 * This registry links Dialectic IR ("true IR") modules into the active form layer
 * by providing a stable lookup table consumable from relative/form.
 */

import type { DialecticIR } from '@schema/dialectic';

export type DialecticIRRegistryEntry = {
  key: string;
  id: string;
  title: string;
  section: string;
  module: string;
  export: string;
};

export const dialecticIRRegistryEntries: DialecticIRRegistryEntry[] = [
${entries}
];

export const dialecticIRRegistry: Record<string, DialecticIRRegistryEntry> = Object.fromEntries(
  dialecticIRRegistryEntries.map(e => [e.key, e])
);

export const dialecticIRIdIndex: Record<string, string[]> = {
${idIndex}
};

export function getDialecticIRMeta(key: string): DialecticIRRegistryEntry | undefined {
  return dialecticIRRegistry[key];
}

export function getDialecticIRKeysById(id: string): string[] {
  return dialecticIRIdIndex[id] ?? [];
}

/**
 * Load a DialecticIR by registry key.
 *
 * Note: This uses dynamic import of the source module specifier recorded in the registry.
 * In built JS output, resolution depends on your bundler/aliasing strategy.
 */
export async function loadDialecticIR(key: string): Promise<DialecticIR> {
  const meta = getDialecticIRMeta(key);
  if (!meta) throw new Error('Unknown DialecticIR key: ' + key);
  const mod = await import(meta.module);
  const ir = (mod as any)[meta.export] as unknown;
  return ir as DialecticIR;
}
`;

  await fs.writeFile(outFile, content, 'utf8');

  // eslint-disable-next-line no-console
  console.log(
    `Generated ${path.relative(packageRoot, outFile)} with ${exportSpecs.length} IR exports.`
  );
}

await main();
