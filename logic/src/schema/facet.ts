import { z } from 'zod';

export const FacetSchema = z.object({
  id: z.string().min(1),
  name: z.string().min(1),
  description: z.string().optional(),
  scope: z.array(z.string()).optional(),
  attributes: z.record(z.string(), z.any()).optional(),
  constraints: z.record(z.string(), z.any()).optional(),
  tags: z.array(z.string()).optional(),
  provenance: z.any().optional(),
  meta: z.record(z.string(), z.any()).optional(),
});

export type Facet = z.infer<typeof FacetSchema>;
