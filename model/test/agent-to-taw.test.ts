import { describe, expect, it } from 'vitest';

import {
  contextDocumentToTawIntentEvent,
  functionCallOutputToTawActEvent,
  kernelRunRequestToTawActEvent,
  kernelRunResultToTawResultEvent,
  planTextToTawPlanEvent,
  promptOutputToPlanPromptText,
  type FunctionCallOutput,
} from '../src/sdsl';

import { KERNEL_TAW_ACTIONS } from '@organon/gdsl';

describe('Model → TAW bridge', () => {
  it('maps FunctionCallOutput into a validated taw.act event', () => {
    const call: FunctionCallOutput = {
      type: 'function',
      name: 'agent.act',
      arguments: { x: 1, y: 'two' },
      schema: {
        type: 'object',
        properties: {
          x: { type: 'number' },
          y: { type: 'string' },
        },
        required: ['x', 'y'],
      },
    };

    const event = functionCallOutputToTawActEvent(call, {
      goalId: 'g1',
      stepId: 's1',
      correlationId: 'c0',
      source: 'test',
      meta: { dialectic: { note: 'bridge' } },
    });

    expect(event.kind).toBe('taw.act');
    expect(event.payload.goalId).toBe('g1');
    expect(event.payload.stepId).toBe('s1');
    expect(event.payload.action).toBe('agent.act');
    expect(event.payload.input).toEqual({ x: 1, y: 'two' });
    expect(event.correlationId).toBe('c0');
    expect(event.source).toBe('test');
  });

  it('parses a simple plan text into taw.plan', () => {
    const plan = `
1. Load context
2. Determine steps
- Act with tool
`;

    const event = planTextToTawPlanEvent(plan, {
      goalId: 'g1',
      correlationId: 'c0',
      source: 'test',
    });

    expect(event.kind).toBe('taw.plan');
    expect(event.payload.goalId).toBe('g1');
    expect(event.payload.steps.map((s) => s.id)).toEqual(['s1', 's2', 's3']);
    expect(event.payload.steps[0].description).toBe('Load context');
    expect(event.payload.steps[2].description).toBe('Act with tool');
  });

  it('builds taw.intent from ContextDocument.goal', () => {
    const context = {
      id: 'ctx-1',
      timestamp: new Date().toISOString(),
      facts: [],
      schema: {
        id: 's',
        name: 'Schema',
        fieldCount: 0,
        requiredFields: [],
        optionalFields: [],
      },
      goal: { id: 'g1', type: 'seed', description: 'Seed cube' },
    };

    const event = contextDocumentToTawIntentEvent(context, {
      constraints: ['internal-only'],
      correlationId: 'c0',
      source: 'test',
    });

    expect(event.kind).toBe('taw.intent');
    expect(event.payload.goal.id).toBe('g1');
    expect(event.payload.constraints).toEqual(['internal-only']);
  });

  it('adds deterministic formatting instructions for plan prompts', () => {
    const planPrompt = promptOutputToPlanPromptText(
      {
        type: 'prompt',
        content: '## Goal\nDo X',
        sections: { goal: '## Goal\nDo X' },
      },
      { style: 'numbered', maxSteps: 5 },
    );

    expect(planPrompt).toContain('## Response Format');
    expect(planPrompt).toContain('Return ONLY a numbered list');
    expect(planPrompt).toContain('Limit to at most 5 steps.');
  });

  it('maps a kernel run request to taw.act', () => {
    const event = kernelRunRequestToTawActEvent(
      {
        model: { id: 'gds.pagerank', kind: 'graph-kernel', version: '1' },
        input: { graphRef: 'graph:1' },
        params: { damping: 0.85 },
      },
      { goalId: 'g1', stepId: 's2', correlationId: 'c0', source: 'test' },
    );

    expect(event.kind).toBe('taw.act');
    expect(event.payload.goalId).toBe('g1');
    expect(event.payload.stepId).toBe('s2');
    expect(event.payload.action).toBe(KERNEL_TAW_ACTIONS.run);
    expect((event.payload.input as any).model.id).toBe('gds.pagerank');
  });

  it('maps a kernel result to taw.result', () => {
    const event = kernelRunResultToTawResultEvent(
      { ok: true, output: { scores: { a: 0.1 } } },
      { goalId: 'g1', stepId: 's2', correlationId: 'c0', source: 'test' },
    );

    expect(event.kind).toBe('taw.result');
    expect(event.payload.ok).toBe(true);
    expect(event.payload.output).toEqual({ scores: { a: 0.1 } });
  });
});
