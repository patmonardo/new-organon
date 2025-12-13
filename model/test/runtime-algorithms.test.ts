import { describe, expect, it } from 'vitest';
import {
  createRuntimeState,
  recordEvent,
  snapshotTrace,
  toContextFromState,
} from '../src/sdsl/runtime-algorithms';

describe('runtime algorithms', () => {
  it('records and trims to maxEvents', () => {
    let state = createRuntimeState();
    state = recordEvent(state, { kind: 'e1', payload: 1 }, { maxEvents: 2 });
    state = recordEvent(state, { kind: 'e2', payload: 2 }, { maxEvents: 2 });
    state = recordEvent(state, { kind: 'e3', payload: 3 }, { maxEvents: 2 });

    const snap = snapshotTrace(state);
    expect(snap.map((e) => e.kind)).toEqual(['e2', 'e3']);
  });

  it('produces a ContextDocument from state', () => {
    const state = recordEvent(createRuntimeState(), { kind: 'dialectic.test', payload: { ok: true } });
    const ctx = toContextFromState(state);
    expect(ctx.facts.length).toBe(1);
    expect(ctx.facts[0].type).toBe('dialectic.test');
  });
});
