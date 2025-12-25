import { describe, it, expect } from 'vitest';

import {
  planTextToTawPlanEvent,
  kernelRunRequestToTawActEvent,
  contextDocumentToTawIntentEvent,
} from '../src/sdsl/agent-to-taw';

import { contextFromFactTrace, type FactTraceEvent } from '../src/sdsl/fact-trace';

describe('demo-loop', () => {
  it('builds taw.intent from a context derived from trace', () => {
    const trace: FactTraceEvent[] = [
      { kind: 'shape.create', payload: { id: 'shape-1' } },
      {
        kind: 'entity.assert',
        payload: { id: 'e1', type: 'Thing' },
        meta: { factStore: { op: 'assert', kind: 'entity', ids: ['e1'] } },
      },
    ];

    const goal = { id: 'g1', type: 'demo', description: 'Demonstrate the loop' };
    const context = contextFromFactTrace(trace, { schema: { id: 'trace:demo', name: 'Demo' }, goal });
    const intent = contextDocumentToTawIntentEvent(context, { goal, source: 'test', correlationId: 'c1' });

    expect(intent.kind).toBe('taw.intent');
    expect(intent.payload.goal.id).toBe('g1');
    expect(context.goal?.id).toBe('g1');
  });

  it('parses planner output into taw.plan', () => {
    const planText = ['1. First step', '2. Second step'].join('\n');
    const evt = planTextToTawPlanEvent(planText, { goalId: 'g1', source: 'test' });

    expect(evt.kind).toBe('taw.plan');
    expect(evt.payload.goalId).toBe('g1');
    expect(evt.payload.steps).toHaveLength(2);
    expect(evt.payload.steps[0]?.description).toBe('First step');
  });

  it('maps a chosen action into taw.act (kernel.run)', () => {
    const evt = kernelRunRequestToTawActEvent(
      { model: { id: 'kernel.demo' }, input: { a: 1 } },
      { goalId: 'g1', stepId: 's2', source: 'test' },
    );

    expect(evt.kind).toBe('taw.act');
    expect(evt.payload.action).toBe('kernel.run');
    expect(evt.payload.goalId).toBe('g1');
    expect(evt.payload.stepId).toBe('s2');
    expect(evt.payload.input.model.id).toBe('kernel.demo');
  });
});
