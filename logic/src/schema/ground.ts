import { z } from "zod";
import { BaseSchema } from "./base";

// Ground represents an abstract, explanatory container — a Nishkriya.
export const GroundSchema = BaseSchema.extend({
  type: z.literal("logic.Ground"),
  // list of particular condition ids that this ground explains
  contributingConditionIds: z.array(z.string()).optional(),
  // metaphysical annotations
  metaphysics: z.object({
    role: z.literal("ground").optional(),
    universality: z.union([z.literal("universal"), z.literal("particular")]).optional(),
    mode: z.union([z.literal("absolute"), z.literal("relative")]).optional(),
  }).optional(),
});

export type Ground = z.infer<typeof GroundSchema>;
