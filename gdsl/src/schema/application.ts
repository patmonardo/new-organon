import { z } from 'zod';

import { GdsFormEvalCallSchema } from './program';
import {
	GdsDatabaseIdSchema,
	GdsApplicationFormKindSchema,
	GdsGraphNameSchema,
	GdsUserSchema,
} from './common';

/**
 * GDSL IR: GDS application protocol shapes (GDS-L / "GDS Link")
 *
 * This is the TypeScript-first **protocol layer** used to communicate with the
 * Rust GDS boundary (JSON-in/JSON-out via TS-JSON / NAPI).
 *
 * Terminology:
 * - **GDS-L (GDS Link)**: the client→server protocol itself (these payload shapes).
 * - **KernelPort**: the transport/adapter that actually carries these payloads across
 *   a boundary (in-process NAPI, remote HTTP, tests/mocks, etc.).
 * - **G-DSL**: a broader "generic DSL" space that can *emit* GDS-L payloads; specific
 *   S-DSLs can be viewed as projections/clients over this G-DSL space.
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
	GdsFormEvalCallSchema,
]);
export type GdsApplicationCall = z.infer<typeof GdsApplicationCallSchema>;

export function gdsApplicationOperationId(call: GdsApplicationCall): string {
	return `gds.${call.facade}.${call.op}`;
}


