import { describe, it, expect } from 'vitest';

import { DemoKernelPort, type KernelRunRequest } from '../src/sdsl';

describe('kernel-port', () => {
  it('runs demo gds.pregel.rank deterministically', async () => {
    const port = new DemoKernelPort();

    const request: KernelRunRequest = {
      model: { id: 'gds.pregel.rank' },
      input: { seed: ['e1'] },
    };

    const result = await port.run(request);
    expect(result.ok).toBe(true);
    expect((result.output as any).scores.e1).toBe(0.73);
  });
});
