import { z } from 'zod';

export const GdslDatabaseIdSchema = z.string().min(1);
export type GdslDatabaseId = z.infer<typeof GdslDatabaseIdSchema>;

export const GdslGraphNameSchema = z.string().min(1);
export type GdslGraphName = z.infer<typeof GdslGraphNameSchema>;

export const GdslGraphRefSchema = z.discriminatedUnion('kind', [
	z.object({
		kind: z.literal('named_graph'),
		graphName: GdslGraphNameSchema,
	}),
	z.object({
		kind: z.literal('catalog_graph'),
		databaseId: GdslDatabaseIdSchema,
		graphName: GdslGraphNameSchema,
	}),
]);
export type GdslGraphRef = z.infer<typeof GdslGraphRefSchema>;
