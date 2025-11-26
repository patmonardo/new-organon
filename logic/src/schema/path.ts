import { z } from "zod";

export const PathCriteriaSchema = z.object({
  sourceId: z.string(),
  targetId: z.string(),
  type: z.string().optional(),
  subtype: z.string().optional(),
  properties: z.record(z.string(), z.any()).optional(),
  metadata: z.record(z.string(), z.any()).optional(),
  maxDepth: z.number().optional(),
  minDepth: z.number().optional(),
  maxWidth: z.number().optional(),
  minWidth: z.number().optional(),
});

export type FormRelationPathCriteria = z.infer<typeof PathCriteriaSchema>;

/**
 * PathStep - A single step in a knowledge path
 */
export const PathStepSchema = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string().optional(),
  targetId: z.string(),
  targetType: z.string(),
  action: z.string().optional(),
  conditions: z.record(z.string(), z.any()).optional(),
  metadata: z.record(z.string(), z.any()).optional(),
});

export type PathStep = z.infer<typeof PathStepSchema>;

/**
 * Path - A directed sequence through forms
 */
export const PathSchema = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string().optional(),
  steps: z.array(PathStepSchema),
  circular: z.boolean().optional().default(false),
  metadata: z.record(z.string(), z.any()).optional(),
  created: z.date().default(() => new Date()),
  updated: z.date().default(() => new Date()),
});

export type Path = z.infer<typeof PathSchema>;
