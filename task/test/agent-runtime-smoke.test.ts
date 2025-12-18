import { describe, expect, it } from 'vitest';

import { RootAgent } from '../src/agent/RootAgent';

describe('agent runtime (RootAgent)', () => {
  it('runs one step and absorbs trace into next context', async () => {
    const boot = {
      context: {
        id: 'ctx-1',
        timestamp: new Date().toISOString(),
        facts: [],
        schema: {
          id: 'schema-1',
          fieldCount: 0,
          requiredFields: [],
          optionalFields: [],
        },
        constraints: [],
        dependencies: [],
        goal: { id: 'g1', type: 'test', description: 'Test goal' },
      },
      intent: {
        kind: 'taw.intent',
        payload: {
          goal: { id: 'g1', type: 'test', description: 'Test goal' },
        },
      },
      planPromptText: 'Make a plan',
      syscalls: {
        syscalls: [],
      },
    };

    const ra = new RootAgent(boot);

    const { absorb, turn } = await ra.step(async ({ state }) => {
      return {
        plan: {
          kind: 'taw.plan',
          payload: { goalId: state.intent.payload.goal.id, steps: [{ id: 's1', description: 'Do it' }] },
        },
        act: {
          kind: 'taw.act',
          payload: { goalId: state.intent.payload.goal.id, stepId: 's1', action: 'noop', input: { a: 1 } },
        },
        result: {
          kind: 'taw.result',
          payload: { goalId: state.intent.payload.goal.id, stepId: 's1', ok: true, output: { ok: true } },
        },
        traceDelta: [
          {
            kind: 'print',
            payload: { text: 'hello' },
            meta: { factStore: { op: 'assert', ids: ['p1'], kind: 'print' } },
          },
        ],
      };
    });

    expect(turn.intent.kind).toBe('taw.intent');
    expect(absorb.absorbedCount).toBe(1);
    expect(ra.getState().context.facts.length).toBe(1);
  });
});
