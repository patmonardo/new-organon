import { z } from 'zod';

import { GdsFormEvalCallSchema } from './gds.form-eval';
import { GdsGraphStoreCallSchema } from './gds.graph-store';
import {
	GdsDatabaseIdSchema,
	GdsApplicationFormKindSchema,
	GdsGraphNameSchema,
	GdsUserSchema,
} from './gds.common';

/**
 * GDSL IR: GDS application protocol shapes
 *
 * This is the TypeScript-first "IR protocol language" used to communicate with
 * the Rust GDS boundary (JSON-in/JSON-out via NAPI).
 *
 * Notes:
 * - TB-safe: heavy data is never returned directly; prefer handles/jobs/exports.
 * - Schema-first: operation IDs are stable string literals.
 */

export const GdsGraphStoreCatalogFacadeSchema = z.literal('graph_store_catalog');
export type GdsGraphStoreCatalogFacade = z.infer<
	typeof GdsGraphStoreCatalogFacadeSchema
>;

const GraphStoreCatalogBase = z.object({
	kind: GdsApplicationFormKindSchema.optional(),
	facade: GdsGraphStoreCatalogFacadeSchema,
	user: GdsUserSchema,
	databaseId: GdsDatabaseIdSchema,
});

/**
 * Mirrors the Rust trait `GraphCatalogApplications` operations.
 *
 * See: gds/src/applications/graph_store_catalog/facade/graph_catalog_applications.rs
 */
export const GdsGraphStoreCatalogCallSchema = z.discriminatedUnion('op', [
	GraphStoreCatalogBase.extend({
		op: z.literal('list_graphs'),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('graph_memory_usage'),
		graphName: GdsGraphNameSchema,
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('drop_graph'),
		graphName: GdsGraphNameSchema,
		failIfMissing: z.boolean().default(false),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('drop_graphs'),
		graphNames: z.array(GdsGraphNameSchema).default([]),
		failIfMissing: z.boolean().default(false),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('drop_node_properties'),
		graphName: GdsGraphNameSchema,
		nodeProperties: z.array(z.string().min(1)).default([]),
		failIfMissing: z.boolean().default(false),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('drop_relationships'),
		graphName: GdsGraphNameSchema,
		relationshipType: z.string().min(1),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('stream_node_properties'),
		graphName: GdsGraphNameSchema,
		nodeProperties: z.array(z.string().min(1)).default([]),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('stream_relationship_properties'),
		graphName: GdsGraphNameSchema,
		relationshipProperties: z.array(z.string().min(1)).default([]),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('stream_relationships'),
		graphName: GdsGraphNameSchema,
		relationshipTypes: z.array(z.string().min(1)).default([]),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('write_node_properties'),
		graphName: GdsGraphNameSchema,
		nodeProperties: z.array(z.string().min(1)).default([]),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('write_node_labels'),
		graphName: GdsGraphNameSchema,
		nodeLabels: z.array(z.string().min(1)).default([]),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('write_relationship_properties'),
		graphName: GdsGraphNameSchema,
		relationshipProperties: z.array(z.string().min(1)).default([]),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('write_relationships'),
		graphName: GdsGraphNameSchema,
		relationshipType: z.string().min(1),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('export_to_csv'),
		graphName: GdsGraphNameSchema,
		exportPath: z.string().min(1),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('export_to_database'),
		graphName: GdsGraphNameSchema,
		targetDatabase: z.string().min(1),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('project_native'),
		projectionConfig: z.unknown(),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('project_generic'),
		projectionConfig: z.unknown(),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('generate_graph'),
		generationConfig: z.unknown(),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('sample_graph'),
		graphName: GdsGraphNameSchema,
		samplingConfig: z.unknown(),
	}),
]);

export type GdsGraphStoreCatalogCall = z.infer<
	typeof GdsGraphStoreCatalogCallSchema
>;

export const GdsApplicationCallSchema = z.union([
	GdsGraphStoreCatalogCallSchema,
	GdsGraphStoreCallSchema,
	GdsFormEvalCallSchema,
]);
export type GdsApplicationCall = z.infer<typeof GdsApplicationCallSchema>;

export function gdsApplicationOperationId(call: GdsApplicationCall): string {
	return `gds.${call.facade}.${call.op}`;
}
