import { z } from 'zod';

import { ContextDocumentSchema } from './sdk/agent-context';
import { TraceEventSchema } from './invariants';

/**
 * RootAgent absorption surface (schema-first)
 *
 * The OS loop closes when a trace delta (prints) is re-absorbed into a new
 * context for the next turn.
 *
 * This is intentionally structural:
 * - It describes the request/response artifact.
 * - Implementations can live in Agent/Model/Task runtimes.
 */

export const AbsorptionStrategySchema = z.enum(['append', 'recompute']);
export type AbsorptionStrategy = z.infer<typeof AbsorptionStrategySchema>;

export const RootAgentAbsorbRequestSchema = z
  .object({
    previous: ContextDocumentSchema,
    traceDelta: z.array(TraceEventSchema),

    /**
     * append: treat `traceDelta` as additional facts for the next context
     * recompute: rebuild context from a provided full trace (implementation-defined)
     */
    strategy: AbsorptionStrategySchema.optional(),

    /** Optional cap for next context facts. */
    maxFacts: z.number().int().positive().optional(),

    meta: z.record(z.string(), z.unknown()).optional(),
  })
  .strict();

export type RootAgentAbsorbRequest = z.infer<typeof RootAgentAbsorbRequestSchema>;

export const RootAgentAbsorbResultSchema = z
  .object({
    next: ContextDocumentSchema,
    absorbedCount: z.number().int().nonnegative(),
    meta: z.record(z.string(), z.unknown()).optional(),
  })
  .strict();

export type RootAgentAbsorbResult = z.infer<typeof RootAgentAbsorbResultSchema>;

export function parseRootAgentAbsorbRequest(input: unknown): RootAgentAbsorbRequest {
  return RootAgentAbsorbRequestSchema.parse(input);
}

export function parseRootAgentAbsorbResult(input: unknown): RootAgentAbsorbResult {
  return RootAgentAbsorbResultSchema.parse(input);
}
