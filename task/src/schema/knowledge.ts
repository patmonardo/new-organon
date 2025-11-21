import { z } from 'zod';
import { ProvenanceSchema } from './provenance';

export const EpistemicStatusSchema = z.enum([
  'hypothesis', // proposed, minimally justified
  'supported', // has evidence and coherent fit
  'contested', // conflicting evidence/arguments
  'established', // robust, widely coherent
  'axiomatic', // taken as foundational within a frame
]);

export const ClaimSchema = z.object({
  id: z.string(),
  statement: z.string(),
  language: z.string().default('en'),
  context: z.record(z.string(), z.any()).optional(),
});

export const JustificationSchema = z.object({
  id: z.string(),
  rationale: z.string().optional(),
  arguments: z.array(z.string()).optional(),
  modelRefs: z.array(z.string()).optional(),
});

export const KnowledgeUnitSchema = z.object({
  id: z.string(),
  claim: ClaimSchema,
  justification: JustificationSchema.optional(),
  provenance: ProvenanceSchema,
  status: EpistemicStatusSchema.default('hypothesis'),
  confidence: z.number().min(0).max(1).default(0.5),
  tags: z.array(z.string()).optional(),
  createdAt: z.date().default(() => new Date()),
  updatedAt: z.date().default(() => new Date()),
  metadata: z.record(z.string(), z.any()).optional(),
});

export type EpistemicStatus = z.infer<typeof EpistemicStatusSchema>;
export type Claim = z.infer<typeof ClaimSchema>;
export type Justification = z.infer<typeof JustificationSchema>;
export type KnowledgeUnit = z.infer<typeof KnowledgeUnitSchema>;
