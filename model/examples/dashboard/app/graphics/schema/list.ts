//@graphics/schema/list.ts
import { z } from 'zod';
import { LinkSchema } from './link';

export const ListItemSchema = z.object({
  id: z.string(),
  content: z.record(z.string(), z.any()),
  relations: z.array(LinkSchema).optional(),
});

export const ListLayoutSchema = z.object({
  type: z.enum(['linear', 'grid', 'hierarchical']),
  spacing: z.enum(['compact', 'normal', 'relaxed']).optional(),
  alignment: z.enum(['start', 'center', 'end']).optional(),
});

export const ListShapeSchema = z.object({
  items: z.array(ListItemSchema),
  layout: ListLayoutSchema,
  navigation: z.object({
    search: z.boolean().optional(),
    pagination: z.boolean().optional(),
    creation: z.boolean().optional(),
  }).optional(),
  relations: z.array(LinkSchema).optional(),
});

export type ListShape = z.infer<typeof ListShapeSchema>;
