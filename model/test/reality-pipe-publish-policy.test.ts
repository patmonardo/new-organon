import { describe, expect, it, vi } from 'vitest';
import { InMemoryRealityPipe } from '../src/sdsl/reality-pipe';

describe('RealityPipe publish policy: kernel conclusive enforcement (warn-by-default)', () => {
  it('warns when publishing a kernel-sourced conclusive print by default and still publishes', () => {
    const pipe = new InMemoryRealityPipe<string, any, any>();
    const warnSpy = vi.spyOn(console, 'warn');

    const p = {
      id: 'pk-001',
      ts: Date.now(),
      kind: 'conceiving',
      role: 'kernel',
      epistemicLevel: 'conclusive',
      // Omit provenance so schema parsing doesn't fail for the policy check.
      payload: { proposition: 'x' },
    } as any;

    expect(() => pipe.publish(p)).not.toThrow();
    expect(warnSpy).toHaveBeenCalled();

    warnSpy.mockRestore();
  });

  it('allows kernel conclusive prints when opt-in is set (no warning)', () => {
    const pipe = new InMemoryRealityPipe<string, any, any>({ kernelConclusiveAllowed: true });
    const warnSpy = vi.spyOn(console, 'warn');
    const p = {
      id: 'pk-002',
      ts: Date.now(),
      kind: 'conceiving',
      role: 'kernel',
      epistemicLevel: 'conclusive',
      payload: { proposition: 'x' },
    } as any;

    expect(() => pipe.publish(p)).not.toThrow();
    expect(warnSpy).not.toHaveBeenCalled();

    warnSpy.mockRestore();
  });

  it('throws when policyWarnOnly is false and kernel conclusive is published without opt-in', () => {
    const pipe = new InMemoryRealityPipe<string, any, any>({ policyWarnOnly: false });
    const p = {
      id: 'pk-003',
      ts: Date.now(),
      kind: 'conceiving',
      role: 'kernel',
      epistemicLevel: 'conclusive',
      payload: { proposition: 'x' },
    } as any;

    expect(() => pipe.publish(p)).toThrow();
  });
});
