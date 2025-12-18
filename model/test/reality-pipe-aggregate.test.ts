import { describe, expect, it } from 'vitest';
import { InMemoryRealityPipe } from '../src/sdsl/reality-pipe';

describe('InMemoryRealityPipe aggregation (conclusive-latest default)', () => {
  it('prefers conclusive prints when available', () => {
    const pipe = new InMemoryRealityPipe<string, any, any>();

    // Aggregation defaults require: provenance.id, confidence>=0.8, epistemic>=inferred,
    // and payload.proof.evidenceIds length>=1.
    const p1 = {
      id: 'g1-1',
      ts: Date.now() - 3000,
      kind: 'conceiving',
      role: 'user',
      payload: { subject: 'node:A', proposition: 'p1', proof: { evidenceIds: ['e1'] } },
      provenance: { id: 'prov-1', sources: ['s1'] },
      epistemicLevel: 'inferred',
      confidence: 0.9,
    };
    const p2 = {
      id: 'g1-2',
      ts: Date.now() - 2000,
      kind: 'conceiving',
      role: 'user',
      payload: { subject: 'node:A', proposition: 'p2', proof: { evidenceIds: ['e2'] } },
      provenance: { id: 'prov-2', sources: ['s2'] },
      epistemicLevel: 'conclusive',
      confidence: 0.95,
    };
    const p3 = {
      id: 'g1-3',
      ts: Date.now() - 1000,
      kind: 'conceiving',
      role: 'user',
      payload: { subject: 'node:A', proposition: 'p3', proof: { evidenceIds: ['e3'] } },
      provenance: { id: 'prov-3', sources: ['s3'] },
      epistemicLevel: 'inferred',
      confidence: 0.99,
    };

    pipe.publish(p1 as any);
    pipe.publish(p2 as any);
    pipe.publish(p3 as any);

    const view = pipe.read({ kind: 'conceiving', aggregate: { reducer: 'conclusive-latest' } });

    expect(view.aggregated).toBeDefined();
    expect(view.aggregated!['node:A'].id).toBe('g1-2');
  });

  it('falls back to latest when no conclusive present', () => {
    const pipe = new InMemoryRealityPipe<string, any, any>();

    const p1 = {
      id: 'g2-1',
      ts: Date.now() - 3000,
      kind: 'conceiving',
      role: 'user',
      payload: { subject: 'node:B', proposition: 'p1', proof: { evidenceIds: ['e1'] } },
      provenance: { id: 'prov-1', sources: ['s1'] },
      epistemicLevel: 'inferred',
      confidence: 0.9,
    };
    const p2 = {
      id: 'g2-2',
      ts: Date.now() - 1000,
      kind: 'conceiving',
      role: 'user',
      payload: { subject: 'node:B', proposition: 'p2', proof: { evidenceIds: ['e2'] } },
      provenance: { id: 'prov-2', sources: ['s2'] },
      epistemicLevel: 'inferred',
      confidence: 0.95,
    };

    pipe.publish(p1 as any);
    pipe.publish(p2 as any);

    const view = pipe.read({ kind: 'conceiving', aggregate: { reducer: 'conclusive-latest' } });
    expect(view.aggregated!['node:B'].id).toBe('g2-2');
  });

  it('honors minConfidence and excludes low-confidence prints', () => {
    const pipe = new InMemoryRealityPipe<string, any, any>();

    const low = {
      id: 'g3-low',
      ts: Date.now() - 1000,
      kind: 'conceiving',
      role: 'user',
      payload: { subject: 'node:C', proposition: 'low', proof: { evidenceIds: ['e1'] } },
      provenance: { id: 'prov-1', sources: ['s1'] },
      epistemicLevel: 'conclusive',
      confidence: 0.5,
    };
    const other = {
      id: 'g3-other',
      ts: Date.now() - 500,
      kind: 'conceiving',
      role: 'user',
      payload: { subject: 'node:C', proposition: 'other', proof: { evidenceIds: ['e2'] } },
      provenance: { id: 'prov-2', sources: ['s2'] },
      epistemicLevel: 'inferred',
      confidence: 0.9,
    };

    pipe.publish(low as any);
    pipe.publish(other as any);

    const view = pipe.read({ kind: 'conceiving', aggregate: { reducer: 'conclusive-latest', minConfidence: 0.8 } });
    // low-confidence conclusive should be ignored; fallback selects other latest
    expect(view.aggregated!['node:C'].id).toBe('g3-other');
  });

  it('supports count reducer', () => {
    const pipe = new InMemoryRealityPipe<string, any, any>();

    for (let i = 0; i < 5; i++) {
      pipe.publish({
        id: `ct-${i}`,
        ts: Date.now() + i,
        kind: 'taw',
        role: 'system',
        payload: { subject: 's1', proof: { evidenceIds: [`e-${i}`] } },
        provenance: { id: `prov-${i}`, sources: ['s'] },
        epistemicLevel: 'inferred',
        confidence: 0.9,
      } as any);
    }

    const view = pipe.read({ kind: 'taw', aggregate: { reducer: 'count' } });
    expect(view.aggregated!['s1'].count).toBe(5);
  });
});
