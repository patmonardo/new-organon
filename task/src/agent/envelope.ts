import { z } from 'zod';

import { ContextDocumentSchema } from '../schema/context-document';
import { TraceEventSchema } from '../schema/trace';
import { TawActEventSchema, TawIntentEventSchema, TawPlanEventSchema, TawResultEventSchema } from '../schema/taw';

/**
 * RootAgent container surface (schema-first)
 *
 * This is the minimal validated exchange bundle for an agent loop:
 * - consumes a ContextDocument
 * - emits TAW events (intent → plan → act → result)
 * - carries a trace delta
 *
 * NOTE: This previously lived in @organon/gdsl. It now lives in @organon/task.
 */

export const RootAgentStepIdSchema = z.string().min(1);
export type RootAgentStepId = z.infer<typeof RootAgentStepIdSchema>;

export const RootAgentLoopIdSchema = z.string().min(1);
export type RootAgentLoopId = z.infer<typeof RootAgentLoopIdSchema>;

export const RootAgentEnvelopeMetaSchema = z
  .object({
    loopId: RootAgentLoopIdSchema.optional(),
    stepId: RootAgentStepIdSchema.optional(),
    note: z.string().optional(),
  })
  .passthrough();
export type RootAgentEnvelopeMeta = z.infer<typeof RootAgentEnvelopeMetaSchema>;

export const RootAgentLoopTurnSchema = z
  .object({
    meta: RootAgentEnvelopeMetaSchema.optional(),

    context: ContextDocumentSchema,

    intent: TawIntentEventSchema,
    plan: TawPlanEventSchema.optional(),
    act: TawActEventSchema.optional(),
    result: TawResultEventSchema.optional(),

    traceDelta: z.array(TraceEventSchema).default([]),
  })
  .strict();
export type RootAgentLoopTurn = z.infer<typeof RootAgentLoopTurnSchema>;

export const RootAgentKernelTurnSchema = RootAgentLoopTurnSchema.extend({
  // Intentionally structural; kernel result validation is transport/runtime-specific.
  kernelResult: z.unknown(),
}).strict();
export type RootAgentKernelTurn = z.infer<typeof RootAgentKernelTurnSchema>;

export const RootAgentBootEnvelopeSchema = z
  .object({
    context: ContextDocumentSchema,
    intent: TawIntentEventSchema,
    planPromptText: z.string().optional(),
    // Declared action surface; kept opaque at schema layer.
    syscalls: z.record(z.string(), z.unknown()).optional(),
    meta: z.record(z.string(), z.unknown()).optional(),
  })
  .strict();
export type RootAgentBootEnvelope = z.infer<typeof RootAgentBootEnvelopeSchema>;

export const AbsorptionStrategySchema = z.enum(['append', 'recompute']);
export type AbsorptionStrategy = z.infer<typeof AbsorptionStrategySchema>;

export const RootAgentAbsorbRequestSchema = z
  .object({
    previous: ContextDocumentSchema,
    traceDelta: z.array(TraceEventSchema),
    strategy: AbsorptionStrategySchema.optional(),
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

export function parseRootAgentBootEnvelope(input: unknown): RootAgentBootEnvelope {
  return RootAgentBootEnvelopeSchema.parse(input);
}
export function parseRootAgentLoopTurn(input: unknown): RootAgentLoopTurn {
  return RootAgentLoopTurnSchema.parse(input);
}
export function parseRootAgentKernelTurn(input: unknown): RootAgentKernelTurn {
  return RootAgentKernelTurnSchema.parse(input);
}
export function parseRootAgentAbsorbRequest(input: unknown): RootAgentAbsorbRequest {
  return RootAgentAbsorbRequestSchema.parse(input);
}
export function parseRootAgentAbsorbResult(input: unknown): RootAgentAbsorbResult {
  return RootAgentAbsorbResultSchema.parse(input);
}


