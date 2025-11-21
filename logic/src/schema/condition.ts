import { z } from "zod";
import { BaseSchema } from "./base";

// Condition represents a transactional, particular occurrence — a Kriya.
export const ConditionSchema = BaseSchema.extend({
  type: z.literal("logic.Condition"),
  // optional pointer to the ground (absolute) that explains this condition
  groundedBy: z.string().optional(),
  metaphysics: z.object({
    role: z.literal("condition").optional(),
    universality: z.union([z.literal("particular"), z.literal("universal")]).optional(),
    mode: z.union([z.literal("relative"), z.literal("absolute")]).optional(),
  }).optional(),
});

export type Condition = z.infer<typeof ConditionSchema>;
