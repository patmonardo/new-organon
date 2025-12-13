import { describe, expect, it } from 'vitest';
import { TawEventSchema, TAW_KINDS } from '../src/schema/taw.js';

describe('TAW concept surface schemas', () => {
  it('accepts intent/plan/act/result events', () => {
    const [intentKind, planKind, actKind, resultKind] = TAW_KINDS;

    const intent = TawEventSchema.parse({
      kind: intentKind,
      payload: {
        goal: { id: 'g1', type: 'seed', description: 'Seed dialectical cube' },
        constraints: ['barely-prescribed'],
      },
      correlationId: 'c1',
      source: 'test',
    });

    const plan = TawEventSchema.parse({
      kind: planKind,
      payload: {
        goalId: 'g1',
        steps: [{ id: 's1', description: 'Write schema-first surface' }],
      },
    });

    const act = TawEventSchema.parse({
      kind: actKind,
      payload: {
        goalId: 'g1',
        stepId: 's1',
        action: 'schema.emit',
        input: { note: 'minimal' },
      },
    });

    const result = TawEventSchema.parse({
      kind: resultKind,
      payload: {
        goalId: 'g1',
        stepId: 's1',
        ok: true,
        output: { done: true },
      },
      meta: { layer: 'concept' },
    });

    expect(intent.kind).toBe('taw.intent');
    expect(plan.kind).toBe('taw.plan');
    expect(act.kind).toBe('taw.act');
    expect(result.kind).toBe('taw.result');
  });
});
