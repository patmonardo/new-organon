import { describe, expect, it } from 'vitest';
import { assertKernelConclusiveAllowed } from '../src/policy/kernel-conclusive.js';

describe('kernel conclusive policy', () => {
  it('throws when a kernel-sourced print is conclusive and feature is not enabled', () => {
    const p = {
      id: 'pk-001',
      kind: 'conceiving',
      role: 'kernel',
      timestamp: new Date().toISOString(),
      provenance: { id: 'prov-k-1', origin: 'empirical', createdAt: new Date().toISOString() },
      epistemicLevel: 'conclusive',
      payload: { proposition: 'Kernel claims X is true', proof: { steps: ['x'], evidenceIds: [] } },
    } as any;

    expect(() => assertKernelConclusiveAllowed(p)).toThrowError();
  });

  it('does not throw when kernelConclusiveAllowed is true', () => {
    const p = {
      id: 'pk-002',
      kind: 'conceiving',
      role: 'kernel',
      timestamp: new Date().toISOString(),
      provenance: { id: 'prov-k-2', origin: 'empirical', createdAt: new Date().toISOString() },
      epistemicLevel: 'conclusive',
      payload: { proposition: 'Kernel claims X is true', proof: { steps: ['x'], evidenceIds: [] } },
    } as any;

    expect(() => assertKernelConclusiveAllowed(p, { kernelConclusiveAllowed: true })).not.toThrow();
  });

  it('does not throw for non-kernel prints', () => {
    const p = {
      id: 'pu-001',
      kind: 'conceiving',
      role: 'user',
      timestamp: new Date().toISOString(),
      provenance: { id: 'prov-u-1', origin: 'reflective', createdAt: new Date().toISOString() },
      epistemicLevel: 'conclusive',
      payload: { proposition: 'Agent claims Y is true', proof: { steps: ['y'], evidenceIds: [] } },
    } as any;

    expect(() => assertKernelConclusiveAllowed(p)).not.toThrow();
  });
});
