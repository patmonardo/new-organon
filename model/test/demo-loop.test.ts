import { describe, it, expect } from 'vitest';

import {
  seedDemoLoopFromTrace,
  plannerTextToTawPlan,
  choiceToTawAct,
  type FactTraceEvent,
} from '../src/sdsl';

describe('demo-loop', () => {
  it('seeds intent + plan prompt from trace', () => {
    const trace: FactTraceEvent[] = [
      { kind: 'shape.create', payload: { id: 'shape-1' } },
      {
        kind: 'entity.assert',
        payload: { id: 'e1', type: 'Thing' },
        meta: { factStore: { op: 'assert', kind: 'entity', ids: ['e1'] } },
      },
    ];

    const seed = seedDemoLoopFromTrace(trace, {
      schema: { id: 'trace:demo', name: 'Demo' },
      goal: { id: 'g1', type: 'demo', description: 'Demonstrate the loop' },
      planPrompt: { maxSteps: 3, style: 'numbered' },
      intent: { source: 'test', correlationId: 'c1' },
    });

    expect(seed.intentEvent.kind).toBe('taw.intent');
    expect(seed.intentEvent.payload.goal.id).toBe('g1');
    expect(seed.planPromptText).toContain('## Response Format');
    expect(seed.planPromptText).toContain('Limit to at most 3 steps.');
    expect(seed.context.goal?.id).toBe('g1');
  });

  it('parses planner output into taw.plan', () => {
    const planText = ['1. First step', '2. Second step'].join('\n');
    const evt = plannerTextToTawPlan(planText, { goalId: 'g1', source: 'test' });

    expect(evt.kind).toBe('taw.plan');
    expect(evt.payload.goalId).toBe('g1');
    expect(evt.payload.steps).toHaveLength(2);
    expect(evt.payload.steps[0]?.description).toBe('First step');
  });

  it('maps a chosen action into taw.act (function)', () => {
    const evt = choiceToTawAct(
      {
        type: 'function',
        call: {
          type: 'function',
          name: 'tool.doThing',
          arguments: { x: 1 },
          schema: { type: 'object', properties: { x: { type: 'number' } }, required: ['x'] },
        },
      },
      { goalId: 'g1', stepId: 's1', source: 'test' },
    );

    expect(evt.kind).toBe('taw.act');
    expect(evt.payload.goalId).toBe('g1');
    expect(evt.payload.stepId).toBe('s1');
    expect(evt.payload.action).toBe('tool.doThing');
    expect(evt.payload.input).toEqual({ x: 1 });
  });

  it('maps a chosen action into taw.act (kernel.run)', () => {
    const evt = choiceToTawAct(
      {
        type: 'kernel.run',
        request: { model: { id: 'kernel.demo' }, input: { a: 1 } },
      },
      { goalId: 'g1', stepId: 's2', source: 'test' },
    );

    expect(evt.kind).toBe('taw.act');
    expect(evt.payload.action).toBe('kernel.run');
    expect(evt.payload.goalId).toBe('g1');
    expect(evt.payload.stepId).toBe('s2');
    expect(evt.payload.input.model.id).toBe('kernel.demo');
  });
});
