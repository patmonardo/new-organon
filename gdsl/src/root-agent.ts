import { z } from 'zod';

import { ContextDocumentSchema } from './sdk/agent-context';
import { TraceEventSchema, KernelRunResultStrictSchema } from './invariants';

/**
 * RootAgent container surface (schema-first)
 *
 * This is the minimal, validated “operating system” exchange bundle:
 * - The agent consumes a `ContextDocument`.
 * - The agent emits TAW events (`intent` → `plan` → `act` → `result`).
 * - The system produces a trace delta (kernel prints, assertions, etc.).
 * - The next loop re-absorbs trace delta back into context.
 *
 * This module stays purely structural and lives in GDSL.
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

/**
 * Minimal TAW event schemas (Zod v4)
 *
 * We intentionally mirror the shapes from @organon/task, but do not import its
 * Zod schemas here because @organon/task currently uses Zod v3.
 */

const TawGoalSchema = z.object({
  id: z.string(),
  type: z.string(),
  description: z.string(),
});

const TawEventBaseFields = {
  meta: z.record(z.string(), z.unknown()).optional(),
  correlationId: z.string().optional(),
  source: z.string().optional(),
} as const;

export const TawIntentEventSchema = z.object({
  kind: z.literal('taw.intent'),
  payload: z.object({
    goal: TawGoalSchema,
    constraints: z.array(z.string()).optional(),
  }),
  ...TawEventBaseFields,
});

export const TawPlanEventSchema = z.object({
  kind: z.literal('taw.plan'),
  payload: z.object({
    goalId: z.string(),
    steps: z.array(
      z.object({
        id: z.string(),
        description: z.string(),
      }),
    ),
  }),
  ...TawEventBaseFields,
});

export const TawActEventSchema = z.object({
  kind: z.literal('taw.act'),
  payload: z.object({
    goalId: z.string(),
    stepId: z.string().optional(),
    action: z.string(),
    input: z.unknown().optional(),
  }),
  ...TawEventBaseFields,
});

export const TawResultEventSchema = z.object({
  kind: z.literal('taw.result'),
  payload: z.object({
    goalId: z.string(),
    stepId: z.string().optional(),
    ok: z.boolean(),
    output: z.unknown().optional(),
    error: z.unknown().optional(),
  }),
  ...TawEventBaseFields,
});

export type TawIntentEvent = z.infer<typeof TawIntentEventSchema>;
export type TawPlanEvent = z.infer<typeof TawPlanEventSchema>;
export type TawActEvent = z.infer<typeof TawActEventSchema>;
export type TawResultEvent = z.infer<typeof TawResultEventSchema>;

/**
 * One closed “turn” of the RootAgent loop.
 *
 * - `intent` is required.
 * - `plan/act/result` are optional to allow partial loops.
 * - `traceDelta` is the emitted events since the prior loop.
 */
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

/**
 * A stricter view of a loop turn that includes a kernel result and enforces
 * the kernel result invariant.
 */
export const RootAgentKernelTurnSchema = RootAgentLoopTurnSchema.extend({
  kernelResult: KernelRunResultStrictSchema,
}).strict();

export type RootAgentKernelTurn = z.infer<typeof RootAgentKernelTurnSchema>;

export function parseRootAgentLoopTurn(input: unknown): RootAgentLoopTurn {
  return RootAgentLoopTurnSchema.parse(input);
}

export function parseRootAgentKernelTurn(input: unknown): RootAgentKernelTurn {
  return RootAgentKernelTurnSchema.parse(input);
}
