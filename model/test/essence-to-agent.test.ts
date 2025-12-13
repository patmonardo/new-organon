import { describe, it, expect } from 'vitest';

import { essenceToAgentOutputs, type FactTraceEvent } from '../src/sdsl';

describe('Essence → View → Agent pipeline (Model middle term)', () => {
  it('produces multiple agent outputs from a fact trace', () => {
    const events: FactTraceEvent[] = [
      {
        kind: 'shape.create',
        payload: { id: 'shape:1', type: 'system.Form', name: 'Invoice' },
        meta: {
          factStore: { mode: 'reflection', store: 'FormDB', op: 'assert', kind: 'FormShape', ids: ['shape:1'] },
          dialectic: { tags: [{ layer: 'shape', rule: 'posting' }] },
        },
      },
    ];

    const out = essenceToAgentOutputs(events, {
      formats: ['context', 'prompt', 'function', 'jsonld'],
      functionName: 'agent.apply',
      schema: { id: 'trace:FormShape', name: 'Form Trace' },
      goal: { id: 'g1', type: 'summarize', description: 'Summarize the trace' },
    });

    expect(out.context?.facts.length).toBe(1);
    expect(out.prompt?.content).toContain('## Facts');
    expect(out.function?.name).toBe('agent.apply');
    expect(out.jsonld?.['@type']).toBeDefined();
  });
});
