import {
  KERNEL_ACTIONS,
  KernelModelRefSchema,
  KernelRunRequestSchema,
  KernelRunResultSchema,
} from '../src/kernel-api';

describe('kernel-api', () => {
  it('parses KernelModelRef', () => {
    expect(KernelModelRefSchema.parse({ id: 'gds.pregel.rank' })).toEqual({
      id: 'gds.pregel.rank',
    });
  });

  it('parses KernelRunRequest', () => {
    expect(
      KernelRunRequestSchema.parse({
        model: { id: 'gds.pregel.rank', kind: 'pregel' },
        input: { seed: ['e1'] },
        params: { maxIters: 10 },
      }),
    ).toMatchObject({
      model: { id: 'gds.pregel.rank', kind: 'pregel' },
      input: { seed: ['e1'] },
      params: { maxIters: 10 },
    });
  });

  it('parses KernelRunResult', () => {
    expect(KernelRunResultSchema.parse({ ok: true, output: { scores: { e1: 1 } } })).toEqual({
      ok: true,
      output: { scores: { e1: 1 } },
    });
  });

  it('exposes stable action ids', () => {
    expect(KERNEL_ACTIONS.run).toBe('kernel.run');
  });
});
