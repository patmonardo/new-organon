import fs from 'fs/promises';
import { fileURLToPath } from 'url';
import { ProtoEngine } from './proto-engine';
import { AgentEngine } from './agent-engine';
import { TaskEngine } from './task-engine';
import { AgentProcessor } from './agent-processor';
import type { EmpowermentLike } from '../empowerment-core';
import path from 'path';
import { dirname } from 'path';
const scriptDir = dirname(fileURLToPath(import.meta.url));
// climb up from logic/src/relative/core/proto -> repo root (adjust if layout changes)
const repoRoot = path.resolve(scriptDir, '../../../../..');
const cacheDir = path.join(repoRoot, '.cache');

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
      if (!out.actor) out.actor = a;
      else if (!out.action) out.action = a;
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
      'Usage: demo-agent [actor] [action] [resource] [--privilegeBoost=NUM] [--initiation=JSON|@file] [--json]',
    );
    return;
  }

  const actor = argv.actor ?? 'alice';
  const action = argv.action ?? 'run';
  const resource = argv.res ?? 'resource:task1';

  let boost = 1;
  if (typeof argv.privilegeBoost === 'string') {
    const n = Number(argv.privilegeBoost);
    boost = Number.isFinite(n) && !Number.isNaN(n) ? n : 1;
  }

  // const outJson = !!argv.json;
  const initiation = await parseInitiation(argv.initiation);

  const logger = (msg: string, ...args: any[]) => {
    console.log(`[proto-agent ${new Date().toISOString()}] ${msg}`, ...args);
  };

  // Policy / identity engine that grants actor-level empowerments
  const policyEngine = new ProtoEngine('policy-engine', [
    {
      id: `emp:${actor}:run`,
      subject: actor,
      actions: [action],
      weight: 1,
      certainty: 1,
      facets: [action, resource],
      signatures: [{ id: 's-policy-1', issuer: 'policy', signature: 'ok' }],
    } as EmpowermentLike,
  ]);

  // Task engine: represents the Task-level grants (Goal → Method)
  const taskEngine = new TaskEngine('task-engine', [
    {
      id: `emp:task:run`,
      subject: 'task-system',
      actions: [action],
      weight: 3,
      certainty: 1,
      facets: [action, resource],
      signatures: [{ id: 's-task-1', issuer: 'task', signature: 'task-sig' }],
    } as EmpowermentLike,
  ]);

  // Agent engine: represents the actual agent/worker that will execute tasks
  const agentId = 'agent1';
  const agentEngine = new AgentEngine(agentId, [
    {
      id: `emp:${agentId}:exec`,
      subject: agentId,
      actions: [action],
      weight: 5,
      certainty: 1,
      facets: [action, resource],
      signatures: [
        { id: 's-agent-1', issuer: agentId, signature: 'agent-sig' },
      ],
    } as EmpowermentLike,
  ]);

  // persist each engine's graph for inspection / reload (optional)
  try {
    await policyEngine.saveGraph(path.join(cacheDir, 'policy-engine.graph.json'));
    await taskEngine.saveGraph(path.join(cacheDir, 'task-engine.graph.json'));
    await agentEngine.saveGraph(path.join(cacheDir, 'agent-engine.graph.json'));
    logger('saved engine graphs to ./.cache/*.graph.json');
  } catch {
    /* non-fatal */
  }

  // example: show loading into a fresh engine (demo of persistence)
  // const agentEngineReload = new AgentEngine(agentId, []);
  // await agentEngineReload.loadGraph('./.cache/agent-engine.graph.json', { overwrite: true });
  // logger('reloaded agent engine graph into new instance');

  // (AgentRunner not required here; AgentProcessor will run tasks)

  // AgentProcessor composes the prototype processor and manages a task queue
  const agentProcessor = new AgentProcessor(
    [policyEngine, taskEngine, agentEngine],
    { logger, initiation },
  );

  // create a proto task and submit it
  const task = {
    id: 'task-001',
    actor,
    action,
    resource,
    payload: { job: 'demo' },
  };

  logger(
    `submitting task=${task.id} actor=${actor} action=${action} resource=${resource} boost=${boost}`,
  );
  agentProcessor.submitTask(task);

  // run one cycle (demo) — in real use you might call startPolling()
  const outcome = await agentProcessor.pollAndRunOnce({
    privilegeBoost: boost,
  });

  if (!outcome) {
    logger('no outcome (queue empty or poll error)');
    return;
  }
  // narrow the unknown shape for demo usage
  const o = outcome as any;

  if (!o.ok) {
    logger(`task denied: ${o.reason}`);
    console.log('authorize result:', o.result);
    return;
  }

  logger(`task executed: ${task.id} score=${o.audit?.score}`);
  console.log('audit:', o.audit);
}

// ESM entry check (avoid using require)
const __filename = fileURLToPath(import.meta.url);
if (process.argv[1] === __filename) {
  demo().catch((e) => {
    console.error('demo-agent failed:', e);
    process.exit(1);
  });
}
