import { describe, expect, it, vi } from 'vitest';
import { InMemoryRealityPipe } from '../src/sdsl/reality-pipe';
import { publishTaw, subscribeTaw, type TawKind, isTawKind } from '../src/sdsl/taw';

describe('TAW concept surface', () => {
  it('publishes intent and delivers to kind-filtered subscribers', () => {
    const bus = new InMemoryRealityPipe<TawKind, any, any>();

    const seen = vi.fn();
    subscribeTaw(bus, seen, { kind: 'taw.intent' });

    publishTaw(bus, {
      kind: 'taw.intent',
      payload: {
        goal: { id: 'g1', type: 'seed', description: 'Seed cube' },
        constraints: ['internal-only'],
      },
      correlationId: 'c0',
      source: 'test',
    });

    publishTaw(bus, {
      kind: 'taw.plan',
      payload: {
        goalId: 'g1',
        steps: [{ id: 's1', description: 'Do the thing' }],
      },
    });

    expect(seen).toHaveBeenCalledTimes(1);
    expect(seen.mock.calls[0][0].kind).toBe('taw.intent');
    expect(seen.mock.calls[0][0].correlationId).toBe('c0');
  });

  it('recognizes TAW kinds', () => {
    expect(isTawKind('taw.intent')).toBe(true);
    expect(isTawKind('nope')).toBe(false);
  });
});

