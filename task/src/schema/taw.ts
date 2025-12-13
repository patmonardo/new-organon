/**
 * TAW Concept Surface (Schema-first)
 * ================================
 *
 * Minimal event vocabulary for Task/Agent/Workflow as the “moment of Concept”.
 *
 * This is intentionally barely prescribed:
 * - `intent` posits a goal
 * - `plan` determines steps
 * - `act` performs an action
 * - `result` records outcome
 */

import { z } from 'zod';

export const TAW_KINDS = [
  'taw.intent',
  'taw.plan',
  'taw.act',
  'taw.result',
] as const;

export const TawKindSchema = z.enum(TAW_KINDS);
export type TawKind = z.infer<typeof TawKindSchema>;

export const TawGoalSchema = z.object({
  id: z.string(),
  type: z.string(),
  description: z.string(),
});
export type TawGoal = z.infer<typeof TawGoalSchema>;

export const TawIntentPayloadSchema = z.object({
  goal: TawGoalSchema,
  constraints: z.array(z.string()).optional(),
});
export type TawIntentPayload = z.infer<typeof TawIntentPayloadSchema>;

export const TawPlanPayloadSchema = z.object({
  goalId: z.string(),
  steps: z.array(
    z.object({
      id: z.string(),
      description: z.string(),
    }),
  ),
});
export type TawPlanPayload = z.infer<typeof TawPlanPayloadSchema>;

export const TawActPayloadSchema = z.object({
  goalId: z.string(),
  stepId: z.string().optional(),
  action: z.string(),
  input: z.unknown().optional(),
});
export type TawActPayload = z.infer<typeof TawActPayloadSchema>;

export const TawResultPayloadSchema = z.object({
  goalId: z.string(),
  stepId: z.string().optional(),
  ok: z.boolean(),
  output: z.unknown().optional(),
  error: z.unknown().optional(),
});
export type TawResultPayload = z.infer<typeof TawResultPayloadSchema>;

export const TawPayloadSchema = z.union([
  TawIntentPayloadSchema,
  TawPlanPayloadSchema,
  TawActPayloadSchema,
  TawResultPayloadSchema,
]);
export type TawPayload = z.infer<typeof TawPayloadSchema>;

const TawEventBaseFields = {
  meta: z.record(z.string(), z.unknown()).optional(),
  correlationId: z.string().optional(),
  source: z.string().optional(),
} as const;

export const TawIntentEventSchema = z.object({
  kind: z.literal('taw.intent'),
  payload: TawIntentPayloadSchema,
  ...TawEventBaseFields,
});
export type TawIntentEvent = z.infer<typeof TawIntentEventSchema>;

export const TawPlanEventSchema = z.object({
  kind: z.literal('taw.plan'),
  payload: TawPlanPayloadSchema,
  ...TawEventBaseFields,
});
export type TawPlanEvent = z.infer<typeof TawPlanEventSchema>;

export const TawActEventSchema = z.object({
  kind: z.literal('taw.act'),
  payload: TawActPayloadSchema,
  ...TawEventBaseFields,
});
export type TawActEvent = z.infer<typeof TawActEventSchema>;

export const TawResultEventSchema = z.object({
  kind: z.literal('taw.result'),
  payload: TawResultPayloadSchema,
  ...TawEventBaseFields,
});
export type TawResultEvent = z.infer<typeof TawResultEventSchema>;

export const TawEventSchema = z.discriminatedUnion('kind', [
  TawIntentEventSchema,
  TawPlanEventSchema,
  TawActEventSchema,
  TawResultEventSchema,
]);
export type TawEvent = z.infer<typeof TawEventSchema>;
