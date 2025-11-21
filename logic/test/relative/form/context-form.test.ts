import { describe, it, expect } from 'vitest';
import { FormContext } from '../../../src/relative/form/context';

describe('FormContext (principle)', () => {
  it('creates with defaults', () => {
    const ctx = FormContext.create({ type: 'system.Context', name: 'World' });
    expect(ctx.id).toBeTruthy();
    expect(ctx.type).toBe('system.Context');
    expect(ctx.name).toBe('World');
    expect(ctx.revision).toBe(0);
  });

  it('updates core/state via schema-safe mutators', () => {
    const ctx = FormContext.create({ type: 'system.Context', name: 'C' });
    const r0 = ctx.revision;

    ctx.setName('C2').setDescription('desc');
    ctx.setState({ foo: 'bar' } as any);
    ctx.patchState({ more: true } as any);

    expect(ctx.name).toBe('C2');
    expect(ctx.description).toBe('desc');
    expect((ctx.state as any).foo).toBe('bar');
    expect((ctx.state as any).more).toBe(true);
    expect(ctx.revision).toBeGreaterThan(r0);
  });
});
