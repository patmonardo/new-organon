import { describe, it, expect } from 'vitest';
import { createShape, updateShape, ShapeSchema } from '../../src/schema/shape';

describe('ShapeSchema', () => {
  it('createShape produces a valid document with defaults', () => {
    const doc = createShape({ type: 'system.Shape', name: 'S' });
    // validates against schema
    expect(() => ShapeSchema.parse(doc)).not.toThrow();

    expect(doc.shape.core.type).toBe('system.Shape');
    expect(doc.shape.core.name).toBe('S');
    expect(doc.shape.core.id).toMatch(/^shape:/);
    // BaseState may include defaults (e.g., status, tags). Just ensure it's an object.
    expect(typeof doc.shape.state).toBe('object');
    expect(doc.shape.state).not.toHaveProperty('a');
    expect(doc.shape.facets).toEqual({});
    expect(doc.shape.signature).toBeUndefined();
  });

  it('updateShape merges core/state, replaces facets, controls signature, bumps revision', () => {
    const base = createShape({ type: 'system.Shape', name: 'S' });
    const baseRev = base.revision ?? 0;

    const doc2 = updateShape(base, {
      core: { name: 'S2', type: 'system.Shape.Updated' },
      state: { status: 'active' },
      facets: { x: 1 },
      signature: { sig: true },
      version: 'v2',
      ext: { note: 'ok' },
    });

    expect(doc2.shape.core.name).toBe('S2');
    expect(doc2.shape.core.type).toBe('system.Shape.Updated');
    expect(doc2.shape.state as any).toMatchObject({ status: 'active' });
    expect(doc2.shape.facets).toEqual({ x: 1 });
    expect(doc2.shape.signature).toEqual({ sig: true });
    expect(doc2.version).toBe('v2');
    expect(doc2.ext).toEqual({ note: 'ok' });
    expect(doc2.revision).toBe(baseRev + 1);

    // state merge (shallow)
    const doc3 = updateShape(doc2, { state: { tags: ['x'] } });
    expect((doc3.shape.state as any).status).toBe('active');
    expect((doc3.shape.state as any).tags).toEqual(['x']);
    expect(doc3.revision).toBe((doc2.revision ?? 0) + 1);

    // signature preserved when undefined
    const doc4 = updateShape(doc3, { signature: undefined });
    expect(doc4.shape.signature).toEqual(doc3.shape.signature);

    // signature cleared when null
    const doc5 = updateShape(doc4, { signature: null });
    expect(doc5.shape.signature).toBeUndefined();

    // facets replaced (not merged)
    const doc6 = updateShape(doc5, { facets: { y: 2 } });
    expect(doc6.shape.facets).toEqual({ y: 2 });
  });
});
