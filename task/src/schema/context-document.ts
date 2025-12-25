import { z } from 'zod';

/**
 * Agent Context Display Language (structural)
 *
 * This is the portable schema for what an Agent consumes:
 * facts + schema + optional constraints/goals/dependencies.
 *
 * NOTE: This previously lived in @organon/gdsl. It now lives in @organon/task
 * so agent runtimes do not reach into GDSL.
 */

export const StructuredFactSchema = z.object({
  id: z.string(),
  label: z.string().optional(),
  type: z.string(),
  value: z.unknown(),
  relevance: z.enum(['critical', 'important', 'relevant', 'peripheral', 'irrelevant']).optional(),
  confidence: z.enum(['certain', 'high', 'medium', 'low', 'speculative']).optional(),
  provenance: z.enum(['asserted', 'inferred', 'observed', 'hypothesized', 'inherited']).optional(),
});
export type StructuredFact = z.infer<typeof StructuredFactSchema>;

export const SchemaDescriptionSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  description: z.string().optional(),
  fieldCount: z.number(),
  requiredFields: z.array(z.string()),
  optionalFields: z.array(z.string()),
});
export type SchemaDescription = z.infer<typeof SchemaDescriptionSchema>;

export const ContextConstraintSchema = z.object({
  type: z.enum(['must', 'should', 'should_not', 'must_not']),
  description: z.string(),
  fieldId: z.string().optional(),
});
export type ContextConstraint = z.infer<typeof ContextConstraintSchema>;

export const ContextDocumentSchema = z.object({
  id: z.string(),
  timestamp: z.string(),
  facts: z.array(StructuredFactSchema),
  schema: SchemaDescriptionSchema,
  constraints: z.array(ContextConstraintSchema).optional(),
  goal:
    z
      .object({
        id: z.string(),
        type: z.string(),
        description: z.string(),
      })
      .optional(),
  dependencies:
    z
      .array(
        z.object({
          from: z.string(),
          to: z.string(),
          type: z.string(),
        }),
      )
      .optional(),
});
export type ContextDocument = z.infer<typeof ContextDocumentSchema>;


