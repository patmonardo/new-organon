import fs from 'fs/promises';
import { ProtoEngine } from './proto-engine';
import { PrototypeProcessor } from './proto-processor';
import type { EmpowermentLike } from '../empowerment-core';

function parseArgs(argv: string[]) {
  const out: Record<string, string> = {};
  for (let i = 2; i < argv.length; i++) {
    const a = argv[i];
    if (a === '--json') out.json = '1';
    else if (a === '--help' || a === '-h') out.help = '1';
    else if (a.startsWith('--')) {
      const [k, v] = a.slice(2).split('=');
      out[k] = v ?? 'true';
    } else {
      // positional: subject action resource
      if (!out.subj) out.subj = a;
      else if (!out.act) out.act = a;
      else if (!out.res) out.res = a;
    }
  }
  return out;
}

async function parseInitiation(
  raw?: string,
): Promise<EmpowermentLike | undefined> {
  if (!raw) return undefined;
  try {
    if (raw.startsWith('@')) {
      const path = raw.slice(1);
      const data = await fs.readFile(path, 'utf8');
      return JSON.parse(data) as EmpowermentLike;
    }
    // allow single-quoted CLI JSON by replacing single to double quotes if necessary
    const normalized = raw.trim().startsWith('{')
      ? raw
      : raw.replace(/'/g, '"');
    return JSON.parse(normalized) as EmpowermentLike;
  } catch (err) {
    console.error('Failed to parse initiation JSON:', String(err));
    return undefined;
  }
}

async function demo() {
  const argv = parseArgs(process.argv);
  if (argv.help) {
    console.log(
      'Usage: node demo.js [subject] [action] [resource] [--privilegeBoost=NUM] [--initiation=JSON|@file] [--json]',
    );
    console.log(
      'Example: node demo.js alice read resource:doc1 --privilegeBoost=2 --initiation=\'{"id":"root","weight":100,"certainty":1}\'',
    );
    console.log('Or load file: --initiation=@./shaktipat.json');
    return;
  }

  const subject = argv.subj ?? 'alice';
  const action = argv.act ?? 'read';
  const resource = argv.res ?? 'resource:doc1';

  // parse privilegeBoost robustly; accept numeric string, fallback to 1 on invalid input
  let boost = 1;
  if (typeof argv.privilegeBoost === 'string') {
    const n = Number(argv.privilegeBoost);
    boost = Number.isFinite(n) && !Number.isNaN(n) ? n : 1;
  }

  const outJson = !!argv.json;
  const initiation = await parseInitiation(argv.initiation);

  // simple logger for diagnostics
  const logger = (msg: string, ...args: any[]) => {
    const ts = new Date().toISOString();
    console.log(`[proto ${ts}] ${msg}`, ...args);
  };

  // engine A grants read on resource:doc1
  const engineA = new ProtoEngine('engine-a', [
    {
      id: 'emp:a:read',
      subject: '*',
      actions: ['read'],
      weight: 2,
      certainty: 0.5,
      facets: ['read', 'resource:doc1'],
      signatures: [{ id: 's1', issuer: 'a', signature: 'ok' }],
    },
  ]);

  // engine B grants wide bootstrap (higher weight)
  const engineB = new ProtoEngine('engine-b', [
    {
      id: 'emp:b:root',
      subject: '*',
      actions: ['*'],
      weight: 10,
      certainty: 1,
      facets: ['*'],
      signatures: [{ id: 's2', issuer: 'b', signature: 'ok' }],
    },
  ]);

  // construct processor with optional initiation (root empowerment)
  const processor = new PrototypeProcessor([engineA, engineB], {
    logger,
    initiation,
  });

  try {
    const result = await processor.run(subject, action, resource, {
      privilegeBoost: boost,
    });

    if (outJson) {
      console.log(JSON.stringify(result, null, 2));
      return;
    }

    console.log('prototype pipeline result:');
    console.log(`  subject: ${subject}`);
    console.log(`  action:  ${action}`);
    console.log(`  resource: ${resource}`);
    console.log(`  privilegeBoost: ${boost}`);
    if (initiation) {
      console.log('  initiation id:', initiation.id);
    }
    console.log('');
    console.log(`  total: ${result.total}`);
    console.log('  scores:');
    for (const s of result.scores) {
      console.log(`    - id: ${s.id}`);
      console.log(`      subject: ${s.subject}`);
      console.log(`      score: ${s.score}`);
      if ((s as any).provenance !== undefined)
        console.log(`      provenance: ${(s as any).provenance}`);
    }
  } catch (err) {
    console.error('Prototype demo failed:', err);
    process.exit(2);
  }
}

if (require.main === module) {
  demo().catch((e) => {
    console.error(e);
    process.exit(1);
  });
}
