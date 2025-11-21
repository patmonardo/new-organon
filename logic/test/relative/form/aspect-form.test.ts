import { describe, it, expect } from 'vitest';
import { FormAspect } from '../../../src/relative/form/aspect';

describe('FormAspect (skeletal)', () => {
  it('creates with defaults and exposes core fields', () => {
    const a = FormAspect.create({ type: 'system.Aspect', name: 'A' });
    expect(a.id).toBeTruthy();
    expect(a.type).toBe('system.Aspect');
    expect(a.name).toBe('A');
  });

  it('updates core and state via schema-safe mutators', () => {
    const a = FormAspect.create({ type: 'system.Aspect', name: 'A' });

    a.setCore({ name: 'A2', type: 'system.Aspect.Updated' });
    expect(a.name).toBe('A2');
    expect(a.type).toBe('system.Aspect.Updated');

    a.setState({} as any);
    a.patchState({ meta: { ok: true } } as any);
    expect((a.state as any).meta?.ok).toBe(true);
  });

  it('supports signature and facets extension helpers', () => {
    const a = FormAspect.create({ type: 'system.Aspect', name: 'SigFacet' });

    a.setSignature({ s: 1 });
    a.mergeSignature({ t: 2 });
    let sig = a.toSchema().shape.signature as Record<string, any> | undefined;
    expect(sig?.s).toBe(1);
    expect(sig?.t).toBe(2);

    a.setSignature(null); // clear
    expect(a.toSchema().shape.signature).toBeUndefined();

    a.setFacets({ ns: { foo: 'bar' } });
    a.mergeFacets({ other: { baz: true } });
    const facets = a.toSchema().shape.facets as Record<string, any>;
    expect(facets.other?.baz).toBe(true);
    expect(facets.ns?.foo).toBe('bar');
  });

  it('toJSON returns the underlying schema doc', () => {
    const a = FormAspect.create({ type: 'system.Aspect', name: 'A' });
    const json = a.toJSON();
    expect(json.shape.core.id).toBe(a.id);
    expect(json.shape.core.type).toBe(a.type);
  });
});
