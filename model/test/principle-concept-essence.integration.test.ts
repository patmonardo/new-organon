import { describe, expect, it } from 'vitest';
import { InMemoryRealityPipe } from '../src/sdsl/reality-pipe';
import { translatePrincipleToConcept } from '../../logic/src/translator/principle-to-concept.js';

describe('Integration: Principle -> Concept -> Essence', () => {
  it('publishes principle, translator emits concept, aggregated essence picks conclusive concept', () => {
    const pipe = new InMemoryRealityPipe<string, any, any>();

    // translator subscribes and republishes converted prints
    pipe.subscribe((env) => {
      if (env.kind === 'knowing') {
        const j = translatePrincipleToConcept(env as any) as any;
        pipe.publish(j as any);
      }
    });

    // publish principle
    const principle = {
      id: 'p-principle-001',
      kind: 'knowing',
      role: 'kernel',
      ts: Date.now(),
      timestamp: new Date().toISOString(),
      provenance: { id: 'prov-p-1', origin: 'empirical', createdAt: new Date().toISOString() },
      payload: { modality: 'signal', summary: 'spike in activity on node X', trace: { node: 'node:X', metric: 'pagerank', value: 0.62 } },
    } as any;

    pipe.publish(principle);

    // now read aggregated essence for subject 'node:X'
    const view = pipe.read({ kind: 'conceiving', aggregate: { groupBy: (e) => (e as any).payload?.subject ?? 'node:X', reducer: 'conclusive-latest' } });

    const ag = view.aggregated!['node:X'];
    expect(ag).toBeDefined();
    expect((ag as any).kind).toBe('conceiving');
    expect((ag as any).derivedFrom).toContain('p-principle-001');
  });
});
