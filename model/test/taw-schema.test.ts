import { describe, expect, it } from 'vitest';
import { TawEventSchema } from '../src/sdsl/taw-schema';

describe('TAW schemas', () => {
  it('parses an intent event', () => {
    const parsed = TawEventSchema.parse({
      kind: 'taw.intent',
      payload: {
        goal: { id: 'g1', type: 'seed', description: 'Seed cube' },
        constraints: ['internal-only'],
      },
      meta: { dialectic: { note: 'concept' } },
      correlationId: 'c0',
      source: 'test',
    });

    expect(parsed.kind).toBe('taw.intent');
    expect(parsed.payload.goal.id).toBe('g1');
  });

  it('rejects unknown kinds', () => {
    expect(() =>
      TawEventSchema.parse({
        // @ts-expect-error - runtime check
        kind: 'taw.nope',
        payload: { goalId: 'g1', steps: [] },
      }),
    ).toThrow();
  });
});
