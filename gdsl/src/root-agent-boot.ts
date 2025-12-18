import { z } from 'zod';

import { ContextDocumentSchema } from './sdk/agent-context';
import { TawIntentEventSchema } from './root-agent';
import { SyscallTableSchema } from './syscalls';

/**
 * RootAgent boot envelope (schema-first)
 *
 * This is the “PID 1” starting artifact: the minimal validated state required
 * to start the RootAgent loop.
 */

export const RootAgentBootEnvelopeSchema = z
  .object({
    context: ContextDocumentSchema,

    /** Root positing of a goal (taw.intent). */
    intent: TawIntentEventSchema,

    /** Optional deterministic plan prompt text to hand to an external planner. */
    planPromptText: z.string().optional(),

    /** Declared action surface available to this agent/container. */
    syscalls: SyscallTableSchema.optional(),

    meta: z.record(z.string(), z.unknown()).optional(),
  })
  .strict();

export type RootAgentBootEnvelope = z.infer<typeof RootAgentBootEnvelopeSchema>;

export function parseRootAgentBootEnvelope(input: unknown): RootAgentBootEnvelope {
  return RootAgentBootEnvelopeSchema.parse(input);
}
