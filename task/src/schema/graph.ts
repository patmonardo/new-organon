import { z } from 'zod';

export const KnowledgeNodeSchema = z.object({
  id: z.string(),
  type: z.enum(['claim', 'concept', 'entity', 'topic']),
  label: z.string(),
  metadata: z.record(z.string(), z.any()).optional(),
});

export const KnowledgeEdgeSchema = z.object({
  id: z.string(),
  source: z.string(),
  target: z.string(),
  relation: z.string(),
  weight: z.number().min(0).max(1).optional(),
  metadata: z.record(z.string(), z.any()).optional(),
});

export const KnowledgeGraphSchema = z.object({
  id: z.string(),
  nodes: z.array(KnowledgeNodeSchema),
  edges: z.array(KnowledgeEdgeSchema),
  createdAt: z.date().default(() => new Date()),
  metadata: z.record(z.string(), z.any()).optional(),
});

export type KnowledgeNode = z.infer<typeof KnowledgeNodeSchema>;
export type KnowledgeEdge = z.infer<typeof KnowledgeEdgeSchema>;
export type KnowledgeGraph = z.infer<typeof KnowledgeGraphSchema>;
