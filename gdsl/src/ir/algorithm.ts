import { z } from 'zod';

export const GdslNodeIdSchema = z.number().int().min(0).max(0xffff_ffff);
export type GdslNodeId = z.infer<typeof GdslNodeIdSchema>;

export const GdslNodeRefSchema = z.object({
	kind: z.literal('node_id'),
	nodeId: GdslNodeIdSchema,
});
export type GdslNodeRef = z.infer<typeof GdslNodeRefSchema>;

export const GdslDirectionSchema = z.enum(['outgoing', 'incoming']);
export type GdslDirection = z.infer<typeof GdslDirectionSchema>;

export const GdslPageRankDegreeFunctionSchema = z.enum(['out', 'in', 'avg']);
export type GdslPageRankDegreeFunction = z.infer<
	typeof GdslPageRankDegreeFunctionSchema
>;

export const GdslPageRankSpecSchema = z.object({
	kind: z.literal('pagerank'),
	damping: z.number().min(0).max(1).default(0.85),
	maxIterations: z.number().int().positive().default(20),
	tolerance: z.number().positive().optional(),
	degreeFunction: GdslPageRankDegreeFunctionSchema.default('out'),
});
export type GdslPageRankSpec = z.infer<typeof GdslPageRankSpecSchema>;

export const GdslDijkstraSpecSchema = z.object({
	kind: z.literal('dijkstra'),
	source: GdslNodeRefSchema,
	targets: z.array(GdslNodeRefSchema).default([]),
	weightProperty: z.string().min(1).optional(),
	trackRelationships: z.boolean().default(false),
	relationshipTypes: z.array(z.string().min(1)).default([]),
	direction: GdslDirectionSchema.default('outgoing'),
});
export type GdslDijkstraSpec = z.infer<typeof GdslDijkstraSpecSchema>;

export const GdslAlgorithmSpecSchema = z.discriminatedUnion('kind', [
	GdslPageRankSpecSchema,
	GdslDijkstraSpecSchema,
]);
export type GdslAlgorithmSpec = z.infer<typeof GdslAlgorithmSpecSchema>;

export function gdslAlgorithmOperationId(input: {
	kind: GdslAlgorithmSpec['kind'];
}): string {
	return `gds.algorithm.${input.kind}`;
}
