import { z } from 'zod';
import { KnowledgeGraphSchema } from './graph';
import { KnowledgeUnitSchema } from './knowledge';

// Topic model as reflective encyclopedia of science for an Agent/Task context
export const TopicIdentitySchema = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string().optional(),
});

export const TopicModelSchema = z.object({
  identity: TopicIdentitySchema,
  corpusRef: z.string().optional(), // pointer to dataset(s), opaque here
  vocabulary: z.array(z.string()).optional(),
  topics: z
    .array(
      z.object({
        id: z.string(),
        label: z.string(),
        terms: z
          .array(z.object({ term: z.string(), weight: z.number() }))
          .optional(),
        knowledgeUnits: z.array(KnowledgeUnitSchema).optional(),
        metadata: z.record(z.string(), z.any()).optional(),
      }),
    )
    .optional(),
  graph: KnowledgeGraphSchema.optional(),
  createdAt: z.date().default(() => new Date()),
  updatedAt: z.date().default(() => new Date()),
  metadata: z.record(z.string(), z.any()).optional(),
});

export type TopicIdentity = z.infer<typeof TopicIdentitySchema>;
export type TopicModel = z.infer<typeof TopicModelSchema>;
