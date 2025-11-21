import { describe, it, expect } from 'vitest';
import { JudgmentSchema, createJudgment } from '../../src/schema/judgment';

describe('schema/judgment — happy path', () => {
  it('creates an S–P judgment with default polarity/modality', () => {
    const j = createJudgment({
      type: 'system.Judgment',
      subject: { id: 'S1', type: 'system.Thing' },
      predicate: 'likes',
    });

    const parsed = JudgmentSchema.parse(j);
    expect(parsed.shape.core.type).toBe('system.Judgment');
    expect(parsed.shape.subject.id).toBe('S1');
    expect(parsed.shape.predicate).toBe('likes');
    expect(parsed.shape.object).toBeUndefined();
    expect(parsed.shape.polarity).toBe('affirm');
    expect(parsed.shape.modality).toBe('actual');
    expect(parsed.shape.state).toBeDefined();
    expect(parsed.shape.facets).toEqual({});
  });

  it('creates an S-P-O judgment with explicit fields', () => {
    const j = createJudgment({
      type: 'system.Judgment',
      name: 'deny-inclusion',
      subject: { id: 'S', type: 'system.Thing' },
      predicate: 'subset_of',
      object: { id: 'P', type: 'system.Thing' },
      polarity: 'deny',
      modality: 'necessary',
      agentId: 'agent:1',
      warrant: { proof: 'sketch' },
      facets: { a: 1 },
      state: { status: 'active', tags: ['t1'], meta: { note: 'ok' } },
    });

    const parsed = JudgmentSchema.parse(j);
    expect(parsed.shape.core.name).toBe('deny-inclusion');
    expect(parsed.shape.subject.id).toBe('S');
    expect(parsed.shape.object?.id).toBe('P');
    expect(parsed.shape.polarity).toBe('deny');
    expect(parsed.shape.modality).toBe('necessary');
    expect(parsed.shape.agentId).toBe('agent:1');
    expect((parsed.shape.warrant as any).proof).toBe('sketch');
    expect(parsed.shape.facets).toEqual({ a: 1 });
    expect(parsed.shape.state.status).toBe('active');
  });
});
