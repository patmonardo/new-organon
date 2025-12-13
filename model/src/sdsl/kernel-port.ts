import type { KernelRunRequest, KernelRunResult } from './kernel';

/**
 * KernelPort (execution boundary)
 *
 * Model owns the representational contract (KernelRunRequest/Result).
 * A KernelPort is the execution adapter that can actually run a kernel.
 */
export interface KernelPort {
  readonly name: string;

  run(request: KernelRunRequest): Promise<KernelRunResult>;
}

/**
 * DemoKernelPort
 *
 * Deterministic in-process kernel used for demos/tests.
 * This is NOT meant to be a real executor.
 */
export class DemoKernelPort implements KernelPort {
  readonly name = 'demo';

  async run(request: KernelRunRequest): Promise<KernelRunResult> {
    if (!request?.model?.id) {
      return { ok: false, error: { message: 'Missing model.id' } };
    }

    if (request.model.id === 'gds.pregel.rank') {
      const input = request.input as any;
      const seed: string[] = Array.isArray(input?.seed) ? input.seed : [];

      // Deterministic toy output.
      const scores: Record<string, number> = {};
      for (const [i, id] of seed.entries()) scores[id] = Number((0.73 - i * 0.11).toFixed(2));
      if (!scores.e2) scores.e2 = 0.12;

      return { ok: true, output: { scores } };
    }

    return {
      ok: false,
      error: {
        message: `No demo kernel implementation for model: ${request.model.id}`,
      },
    };
  }
}
