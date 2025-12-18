import { describe, expect, it } from 'vitest';
import { translatePrincipleToConcept } from '../src/translator/principle-to-concept.js';

describe('translatePrincipleToConcept', () => {
  it('converts a principle knowing print into a concept conceiving partial print', () => {
    const principle = {
      id: 'p-principle-001',
      kind: 'knowing',
      role: 'kernel',
      timestamp: new Date().toISOString(),
      provenance: { id: 'prov-p-1', origin: 'empirical', createdAt: new Date().toISOString() },
      payload: { modality: 'signal', summary: 'spike in activity on node X', trace: { node: 'node:X', metric: 'pagerank', value: 0.62 } },
    } as any;

    const j = translatePrincipleToConcept(principle as any);

    expect(j.kind).toBe('conceiving');
    expect(j.derivedFrom).toContain('p-principle-001');
    expect((j.payload as any).proof.evidenceIds).toContain('p-principle-001');
  });
});
