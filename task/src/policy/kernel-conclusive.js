import { PrintEnvelopeSchema } from '../schema/prints.js';
export function assertKernelConclusiveAllowed(candidate, opts = {}) {
    const print = PrintEnvelopeSchema.parse(candidate);
    const allowed = !!opts.kernelConclusiveAllowed;
    if (print.role === 'kernel') {
        if (print.epistemicLevel === 'conclusive') {
            if (!allowed) {
                throw new Error('Kernel-sourced prints MUST NOT be marked as epistemicLevel: "conclusive" unless kernelConclusiveAllowed is enabled');
            }
        }
    }
}
//# sourceMappingURL=kernel-conclusive.js.map