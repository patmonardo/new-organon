import {
  contextFromFactTrace,
  ContextDocumentSchema,
  DIALECTICAL_TRIADS,
  FACT_STORE_MODES,
  type FactTraceEvent,
} from '../src/index';

describe('gdsl sdk: fact-trace + terminology', () => {
  it('builds a ContextDocument from trace events', () => {
    const events: FactTraceEvent[] = [
      {
        kind: 'kernel.run.request',
        payload: { model: { id: 'm1' }, input: { seed: ['e1'] } },
        meta: { factStore: { kind: 'kernel.run', op: 'index', ids: ['r1'] } },
      },
    ];

    const ctx = contextFromFactTrace(events, { schema: { id: 'trace:kernel' } });
    expect(ContextDocumentSchema.parse(ctx).schema.id).toBe('trace:kernel');
    expect(ctx.facts.length).toBe(1);
  });

  it('exports dialectical seeds', () => {
    expect(FACT_STORE_MODES.length).toBeGreaterThan(0);
    expect(DIALECTICAL_TRIADS.logic).toEqual(['identity', 'difference', 'contradiction']);
  });
});
