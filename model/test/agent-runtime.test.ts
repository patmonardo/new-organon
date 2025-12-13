import { describe, it, expect } from 'vitest';

import { EssenceAgentRuntime, type AgentBusEvent } from '../src/sdsl';

describe('EssenceAgentRuntime (Model middleware)', () => {
  it('records published events and produces ContextDocument', () => {
    const rt = new EssenceAgentRuntime({ maxEvents: 10 });

    const e1: AgentBusEvent = {
      kind: 'shape.create',
      payload: { id: 'shape:1', type: 'system.Form', name: 'Invoice' },
      meta: {
        factStore: { mode: 'reflection', store: 'FormDB', op: 'assert', kind: 'FormShape', ids: ['shape:1'] },
      },
    };

    rt.publish(e1);

    const snap = rt.snapshot();
    expect(snap.length).toBe(1);

    const ctx = rt.toContext({ schema: { id: 'trace:FormShape', name: 'Form Trace' } });
    expect(ctx.schema.id).toBe('trace:FormShape');
    expect(ctx.facts.length).toBe(1);
  });

  it('can produce multiple agent outputs (plural adapters)', () => {
    const rt = new EssenceAgentRuntime();

    rt.publish({
      kind: 'dialectic.invariant.violated',
      payload: { stateId: 'entity:2', invariant: 'inv.status', reason: 'missing' },
      meta: {
        factStore: { mode: 'logic', store: 'FormDB', op: 'revise', kind: 'Entity', ids: ['entity:2'] },
      },
    });

    const out = rt.toAgentOutputs({
      formats: ['context', 'prompt', 'function'],
      functionName: 'agent.revise',
      goal: { id: 'g1', type: 'validate', description: 'Repair invariant violations' },
    });

    expect(out.context?.facts.length).toBe(1);
    expect(out.prompt?.content).toContain('## Facts');
    expect(out.function?.name).toBe('agent.revise');
  });
});
