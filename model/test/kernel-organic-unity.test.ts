import { describe, expect, it } from 'vitest';

import {
  DemoKernelPort,
  runKernelOrganicUnity,
  type KernelRunRequest,
} from '../src/sdsl';

describe('Kernel organic unity (Knowing → Print)', () => {
  it('runs a kernel and returns taw + trace prints', async () => {
    const port = new DemoKernelPort();

    const request: KernelRunRequest = {
      model: { id: 'gds.pregel.rank' },
      input: { seed: ['e1', 'e2'] },
    };

    const unity = await runKernelOrganicUnity(port, request, {
      goalId: 'g1',
      stepId: 's1',
      correlationId: 'c1',
      source: 'test',
      meta: { dialectic: { note: 'unit-test' } },
    });

    expect(unity.result.ok).toBe(true);
    expect(unity.taw.act.kind).toBe('taw.act');
    expect(unity.taw.act.payload.action).toBe('kernel.run');
    expect(unity.taw.result.kind).toBe('taw.result');
    expect(unity.taw.result.payload.ok).toBe(true);

    expect(unity.trace).toHaveLength(2);
    expect(unity.trace[0].kind).toBe('kernel.run.request');
    expect(unity.trace[1].kind).toBe('kernel.run.result');

    // runId is derived deterministically from correlationId + stepId
    expect(unity.runId).toBe('c1:s1:kernel.run');
  });

  it('propagates failure as a taw.result ok=false', async () => {
    const port = new DemoKernelPort();

    const request: KernelRunRequest = {
      model: { id: 'gds.unknown' },
      input: { seed: ['e1'] },
    };

    const unity = await runKernelOrganicUnity(port, request, {
      goalId: 'g1',
      correlationId: 'c0',
      source: 'test',
    });

    expect(unity.result.ok).toBe(false);
    expect(unity.taw.result.payload.ok).toBe(false);
    expect(unity.taw.result.payload.error).toBeDefined();
  });
});
