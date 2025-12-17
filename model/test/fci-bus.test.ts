import { describe, expect, it, vi } from 'vitest';
import { InMemoryRealityPipe } from '../src/sdsl/reality-pipe';

describe('RealityPipe', () => {
  it('publishes envelopes and supports kind filtering', () => {
    const bus = new InMemoryRealityPipe<'a' | 'b', { n: number }>();

    const seen: Array<{ kind: 'a' | 'b'; n: number }> = [];

    bus.subscribe((e) => {
      seen.push({ kind: e.kind, n: e.payload.n });
    }, { kind: 'a' });

    bus.publish({ kind: 'a', payload: { n: 1 } });
    bus.publish({ kind: 'b', payload: { n: 2 } });

    expect(seen).toEqual([{ kind: 'a', n: 1 }]);
  });

  it('returns a normalized envelope with id/ts', () => {
    const bus = new InMemoryRealityPipe<'x', { ok: true }>();
    const spy = vi.fn();
    bus.subscribe(spy);

    const env = bus.publish({ kind: 'x', payload: { ok: true }, source: 'test' });

    expect(env.id).toBeTruthy();
    expect(typeof env.ts).toBe('number');
    expect(env.kind).toBe('x');
    expect(env.payload).toEqual({ ok: true });
    expect(env.source).toBe('test');

    expect(spy).toHaveBeenCalledTimes(1);
    expect(spy.mock.calls[0][0].payload).toEqual({ ok: true });
  });
});
