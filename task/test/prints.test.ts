import { describe, expect, it } from 'vitest';
import { readFileSync } from 'fs';
import { join } from 'path';
import { PrintEnvelopeSchema } from '../src/schema/prints.js';

const fixturesDir = join(__dirname, 'fixtures', 'prints');

describe('PrintEnvelope schema', () => {
  it('parses a kernel knowing print fixture', () => {
    const raw = JSON.parse(readFileSync(join(fixturesDir, 'knowing.json'), 'utf8'));
    const p = PrintEnvelopeSchema.parse(raw);

    expect(p.kind).toBe('knowing');
    expect(p.role).toBe('kernel');
    expect(p.payload).toBeDefined();
    expect((p as any).ontology).toBe('monadic');
  });

  it('parses a user conceiving print fixture and links derivedFrom', () => {
    const raw = JSON.parse(readFileSync(join(fixturesDir, 'conceiving.json'), 'utf8'));
    const conceiving = PrintEnvelopeSchema.parse(raw);

    expect(conceiving.kind).toBe('conceiving');
    expect(conceiving.derivedFrom).toContain('p-0001');
    expect((conceiving.payload as any).proof?.evidenceIds).toContain('p-0001');
    expect((conceiving as any).ontology).toBe('triadic');
  });

  it('allows epistemicLevel and confidence fields', () => {
    const withMeta = PrintEnvelopeSchema.parse({
      id: 'p-003',
      kind: 'knowing',
      role: 'kernel',
      timestamp: new Date().toISOString(),
      epistemicLevel: 'tacit',
      confidence: 0.77,
      payload: { modality: 'signal' },
    });

    expect(withMeta.epistemicLevel).toBe('tacit');
    expect(withMeta.confidence).toBeCloseTo(0.77);
  });
});
