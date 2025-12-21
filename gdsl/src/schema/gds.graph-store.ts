import { z } from 'zod';

import { GdsDatabaseIdSchema, GdsGraphNameSchema, GdsUserSchema } from './gds.common';

export const GdsGraphStoreFacadeSchema = z.literal('graph_store');
export type GdsGraphStoreFacade = z.infer<typeof GdsGraphStoreFacadeSchema>;

const GraphStoreBase = z.object({
	facade: GdsGraphStoreFacadeSchema,
	user: GdsUserSchema,
	databaseId: GdsDatabaseIdSchema,
});

export const GdsGraphStoreRelationshipSchema = z.object({
	type: z.string().min(1),
	source: z.number().int(),
	target: z.number().int(),
});
export type GdsGraphStoreRelationship = z.infer<typeof GdsGraphStoreRelationshipSchema>;

export const GdsGraphStoreSnapshotSchema = z.object({
	/** Original/external node ids. */
	nodes: z.array(z.number().int()).min(1),
	relationships: z.array(GdsGraphStoreRelationshipSchema).default([]),
});
export type GdsGraphStoreSnapshot = z.infer<typeof GdsGraphStoreSnapshotSchema>;

export const GdsGraphStorePutCallSchema = GraphStoreBase.extend({
	op: z.literal('put'),
	graphName: GdsGraphNameSchema,
	snapshot: GdsGraphStoreSnapshotSchema,
});
export type GdsGraphStorePutCall = z.infer<typeof GdsGraphStorePutCallSchema>;

export const GdsGraphStoreCallSchema = z.discriminatedUnion('op', [
	GdsGraphStorePutCallSchema,
]);
export type GdsGraphStoreCall = z.infer<typeof GdsGraphStoreCallSchema>;
