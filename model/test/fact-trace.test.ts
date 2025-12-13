import { describe, it, expect } from 'vitest';

import { contextFromFactTrace, type FactTraceEvent } from '../src/sdsl';

describe('Fact trace bridge (Logic → Agent ContextDocument)', () => {
  it('renders a ContextDocument from events with factStore meta', () => {
    const events: FactTraceEvent[] = [
      {
        kind: 'shape.create',
        payload: { id: 'shape:1', type: 'system.Form', name: 'Invoice' },
        meta: {
          factStore: { mode: 'reflection', store: 'FormDB', op: 'assert', kind: 'FormShape', ids: ['shape:1'] },
          dialectic: { tags: [{ layer: 'shape', rule: 'posting' }] },
        },
      },
      {
        kind: 'dialectic.invariant.violated',
        payload: { stateId: 'entity:2', invariant: 'inv.statusActive', reason: 'missing' },
        meta: {
          factStore: { mode: 'logic', store: 'FormDB', op: 'revise', kind: 'Entity', ids: ['entity:2'] },
          dialectic: { tags: [{ layer: 'entity', rule: 'thing' }] },
        },
      },
    ];

    const ctx = contextFromFactTrace(events, {
      schema: { id: 'trace:test', name: 'Trace', description: 'Event trace' },
      goal: { id: 'g1', type: 'validate', description: 'Check invariants' },
    });

    expect(ctx.schema.id).toBe('trace:test');
    expect(ctx.goal?.type).toBe('validate');
    expect(ctx.facts.length).toBe(2);
    expect(ctx.facts[0].provenance).toBe('asserted');
    expect(ctx.facts[1].provenance).toBe('inferred');
    expect(ctx.facts[0].id).toContain('shape.create');
  });
});
