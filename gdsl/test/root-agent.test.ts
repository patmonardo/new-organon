import { describe, expect, it } from 'vitest';

import { parseRootAgentLoopTurn, parseRootAgentKernelTurn } from '../src/root-agent';

describe('RootAgent container surface (GDSL)', () => {
  it('parses a minimal loop turn (intent + context + traceDelta)', () => {
    const turn = parseRootAgentLoopTurn({
      meta: { loopId: 'loop-1' },
      context: {
        id: 'ctx-1',
        timestamp: new Date().toISOString(),
        facts: [],
        schema: { id: 's', fieldCount: 0, requiredFields: [], optionalFields: [] },
      },
      intent: {
        kind: 'taw.intent',
        payload: { goal: { id: 'g1', type: 'demo', description: 'Do the thing' } },
        source: 'test',
      },
      traceDelta: [{ kind: 'shape.create', payload: { id: 'shape-1' } }],
    });

    expect(turn.intent.kind).toBe('taw.intent');
    expect(turn.context.id).toBe('ctx-1');
    expect(turn.traceDelta).toHaveLength(1);
  });

  it('enforces strict kernel result when using RootAgentKernelTurn', () => {
    expect(() =>
      parseRootAgentKernelTurn({
        context: {
          id: 'ctx-1',
          timestamp: new Date().toISOString(),
          facts: [],
          schema: { id: 's', fieldCount: 0, requiredFields: [], optionalFields: [] },
        },
        intent: {
          kind: 'taw.intent',
          payload: { goal: { id: 'g1', type: 'demo', description: 'Do the thing' } },
        },
        traceDelta: [],
        kernelResult: { ok: false },
      }),
    ).toThrow(/ok=false/i);

    const ok = parseRootAgentKernelTurn({
      context: {
        id: 'ctx-1',
        timestamp: new Date().toISOString(),
        facts: [],
        schema: { id: 's', fieldCount: 0, requiredFields: [], optionalFields: [] },
      },
      intent: {
        kind: 'taw.intent',
        payload: { goal: { id: 'g1', type: 'demo', description: 'Do the thing' } },
      },
      traceDelta: [],
      kernelResult: { ok: true, output: { scores: { a: 0.1 } } },
    });

    expect(ok.kernelResult.ok).toBe(true);
  });
});
