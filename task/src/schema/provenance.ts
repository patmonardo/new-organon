import { z } from 'zod';

// Source and provenance of knowledge claims
export const KnowledgeOriginSchema = z.enum([
  'empirical', // sensible cognition / data-derived
  'transcendental', // higher cognition / a priori structure
  'synthetic', // result of synthesis across sources
  'reflective', // agent judgment / meta-cognition
]);

export const EvidenceSchema = z.object({
  id: z.string(),
  kind: z
    .enum([
      'observation',
      'experiment',
      'derivation',
      'citation',
      'model',
      'testimonial',
      'other',
    ])
    .default('other'),
  description: z.string().optional(),
  dataRef: z.string().optional(), // pointer into @organon/model, left opaque here
  strength: z.number().min(0).max(1).optional(),
  timestamp: z.date().optional(),
  metadata: z.record(z.string(), z.any()).optional(),
});

export const ProvenanceSchema = z.object({
  id: z.string(),
  origin: KnowledgeOriginSchema,
  sources: z.array(z.string()).optional(),
  evidence: z.array(EvidenceSchema).optional(),
  agentId: z.string().optional(),
  createdAt: z.preprocess((val) => (typeof val === 'string' ? new Date(val) : val), z.date()).default(() => new Date()),
  metadata: z.record(z.string(), z.any()).optional(),
});

export type KnowledgeOrigin = z.infer<typeof KnowledgeOriginSchema>;
export type Evidence = z.infer<typeof EvidenceSchema>;
export type Provenance = z.infer<typeof ProvenanceSchema>;
