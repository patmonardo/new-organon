import {
  InMemoryRealityPipe,
  publishTaw,
  defineModel,
  sum,
  count,
  dimension,
  modelAndViewToFactTraceEvents,
} from '../sdsl/index';

import {
  DemoKernelPort,
  seedDemoLoopFromTrace,
  plannerTextToTawPlan,
  kernelRunRequestToTawActEvent,
  kernelRunResultToTawResultEvent,
  kernelRunToTraceEvents,
  contextFromFactTrace,
  type FactTraceEvent,
  type KernelRunRequest,
  type KernelRunResult,
  type EventMeta,
} from '@organon/gdsl';

import type { TawKind, TawPayload } from '@organon/task';

// NOTE: This demo runner intentionally imports the kernel/demo-loop primitives from
// GDSL, not Model SDSL.

function printJson(label: string, value: unknown) {
  // eslint-disable-next-line no-console
  console.log(`\n=== ${label} ===`);
  // eslint-disable-next-line no-console
  console.log(JSON.stringify(value, null, 2));
}

async function main() {
  const goal = { id: 'g-demo-1', type: 'demo', description: 'Run the Model→TAW demo loop' };
  const schema = { id: 'trace:demo-loop', name: 'Demo Loop Trace' };

  // 0) Real “logical model” + “view” inside the agent’s context
  const Sales = defineModel({
    name: 'Sales',
    source: 'demo.sales',
    dimensions: {
      day: dimension('created_at', 'day'),
      customer: 'customer',
    },
    measures: {
      revenue: sum('amount'),
      orders: count(),
    },
  });

  const SalesByDay = Sales.view({
    group_by: ['day'],
    aggregate: ['revenue', 'orders'],
    limit: 10,
  });

  const baseTrace: FactTraceEvent[] = [
    { kind: 'shape.create', payload: { id: 'shape-1', name: 'DemoShape' } },
    {
      kind: 'entity.assert',
      payload: { id: 'e1', type: 'Thing', label: 'Node A' },
      meta: { factStore: { op: 'assert', kind: 'entity', ids: ['e1'] } },
    },
    {
      kind: 'relation.assert',
      payload: { from: 'e1', to: 'e2', type: 'LINKS_TO' },
      meta: { factStore: { op: 'assert', kind: 'relation', ids: ['r1'] } },
    },
  ];

  const logicalTrace = modelAndViewToFactTraceEvents(Sales, SalesByDay, {
    modelId: 'm-sales',
    viewId: 'v-sales-by-day',
  });

  const trace: FactTraceEvent[] = [...baseTrace, ...logicalTrace];

  // 1) Trace → Context → taw.intent + plan prompt
  const seed = seedDemoLoopFromTrace(trace, {
    schema,
    goal,
    planPrompt: { maxSteps: 5, style: 'numbered' },
    promptText: '## Goal\nRun the Model→TAW demo loop',
    intent: { source: 'demo-loop-runner', correlationId: 'corr-demo-1' },
  });

  printJson('Context (from trace)', seed.context);
  printJson('TAW intent event (Model→TAW)', seed.intentEvent);
  // Keep plan prompt readable as text
  // eslint-disable-next-line no-console
  console.log(`\n=== Plan prompt text ===\n${seed.planPromptText}\n`);

  // 2) Create a bus and publish the events (this is the “machine motion”)
  const bus = new InMemoryRealityPipe<TawKind, TawPayload, EventMeta>();
  bus.subscribe((env) => {
    // eslint-disable-next-line no-console
    console.log(`[bus] ${env.kind} id=${env.id} corr=${env.correlationId ?? '-'} source=${env.source ?? '-'}`);
  });

  const intentEnvelope = publishTaw(bus, seed.intentEvent);

  // 3) Simulate external planner output → taw.plan
  const plannerText = ['1. Identify kernel run', '2. Execute kernel.run with graph input', '3. Record result'].join(
    '\n',
  );
  const planEvent = plannerTextToTawPlan(plannerText, {
    goalId: goal.id,
    source: 'demo-loop-runner',
    correlationId: intentEnvelope.correlationId ?? intentEnvelope.id,
  });
  const planEnvelope = publishTaw(bus, planEvent);

  // 4) Choose an action → taw.act (kernel.run)
  const kernelRequest: KernelRunRequest = {
    model: { id: 'gds.pregel.rank', kind: 'gds', version: '1' },
    input: { graph: 'g://demo', seed: ['e1'], viewPlan: SalesByDay.toPlan() },
    params: { iterations: 10 },
  };

  const actEvent = kernelRunRequestToTawActEvent(kernelRequest, {
    goalId: goal.id,
    stepId: 's2',
    source: 'demo-loop-runner',
    correlationId: planEnvelope.correlationId ?? planEnvelope.id,
  });
  const actEnvelope = publishTaw(bus, actEvent);

  // 5) Kernel execution via port → taw.result
  const kernelPort = new DemoKernelPort();
  const kernelResult: KernelRunResult = await kernelPort.run(kernelRequest);

  const resultEvent = kernelRunResultToTawResultEvent(kernelResult, {
    goalId: goal.id,
    stepId: 's2',
    source: 'demo-loop-runner',
    correlationId: actEnvelope.correlationId ?? actEnvelope.id,
  });
  const resultEnvelope = publishTaw(bus, resultEvent);

  // 6) Close the loop: kernel request+result → FactTrace → new Context
  const kernelTrace = kernelRunToTraceEvents(kernelRequest, kernelResult, {
    runId: resultEnvelope.id,
  }) as FactTraceEvent[];

  const nextTrace = [...trace, ...kernelTrace];
  const nextContext = contextFromFactTrace(nextTrace, {
    schema,
    goal,
  });

  printJson('Kernel trace events (re-absorption)', kernelTrace);
  printJson('Context (after kernel result re-absorption)', {
    id: nextContext.id,
    timestamp: nextContext.timestamp,
    goal: nextContext.goal,
    facts: nextContext.facts.slice(-5),
  });
}

main().catch((err) => {
  // eslint-disable-next-line no-console
  console.error(err);
  process.exitCode = 1;
});
