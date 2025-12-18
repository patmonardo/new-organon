import { PrintEnvelopeSchema } from '../schema/prints.js';
import type { PrintEnvelope } from '../schema/prints.js';

export function assertKernelConclusiveAllowed(
  candidate: unknown,
  opts: { kernelConclusiveAllowed?: boolean } = {},
): asserts candidate is PrintEnvelope {
  const print = PrintEnvelopeSchema.parse(candidate);
  const allowed = !!opts.kernelConclusiveAllowed;

  if (print.role === 'kernel') {
    if ((print as any).epistemicLevel === 'conclusive') {
      if (!allowed) {
        throw new Error(
          'Kernel-sourced prints MUST NOT be marked as epistemicLevel: "conclusive" unless kernelConclusiveAllowed is enabled',
        );
      }
    }
  }
}
