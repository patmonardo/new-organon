import { describe, it, expect } from 'vitest';
import { FormEntity } from '../../../src/relative/form/entity';

describe('FormEntity (Principle of Entity in Forms)', () => {
  it('creates with defaults and exposes core fields', () => {
    const e = FormEntity.create({ type: 'system.Entity', name: 'A' });
    expect(e.id).toBeTruthy();
    expect(e.type).toBe('system.Entity');
    expect(e.name).toBe('A');
    const ref = e.toRef();
    expect(ref.id).toBe(e.id);
    expect(ref.type).toBe(e.type);
    expect(e.key).toBe(`${ref.type}:${ref.id}`);
  });

  it('updates via schema-safe mutators and bumps revision', () => {
    const e = FormEntity.create({ type: 'system.Entity', name: 'A' });
    const r0 = e.toSchema().revision ?? 0;

    e.setName('B').setDescription('desc');
    e.setState({ foo: 'bar' } as any);
    e.patchState({ more: true } as any);

    expect(e.name).toBe('B');
    expect(e.description).toBe('desc');
    expect((e.state as any).foo).toBe('bar');
    expect((e.state as any).more).toBe(true);
    expect(e.toSchema().revision ?? 0).toBeGreaterThan(r0);
  });

  it('supports signature and facets extension helpers', () => {
    const e = FormEntity.create({ type: 'system.Entity', name: 'Ext' });

    e.setSignature({ a: 1 });
    e.mergeSignature({ b: 2 });
    let sig = e.toSchema().shape.signature as Record<string, any> | undefined;
    expect(sig?.a).toBe(1);
    expect(sig?.b).toBe(2);

    e.setSignature(null); // clear
    expect(e.toSchema().shape.signature).toBeUndefined();

    e.setFacets({ ns: { foo: 'bar' } });
    e.mergeFacets({ other: { baz: true } });
    e.mergeFacet('ns', { qux: 3 });

    const facets = e.toSchema().shape.facets as Record<string, any>;
    expect(facets.other?.baz).toBe(true);
    expect(facets.ns?.foo).toBe('bar');
    expect(facets.ns?.qux).toBe(3);
  });

  it('toJSON returns the underlying schema doc', () => {
    const e = FormEntity.create({ type: 'system.Entity', name: 'A' });
    const json = e.toJSON();
    expect(json.shape.core.id).toBe(e.id);
    expect(json.shape.core.type).toBe(e.type);
  });
});
