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

export const GdsAlgorithmsFacadeSchema = z.literal('algorithms');
export type GdsAlgorithmsFacade = z.infer<typeof GdsAlgorithmsFacadeSchema>;

const GraphStoreCatalogBase = z.object({
	kind: GdsApplicationFormKindSchema.optional(),
	facade: GdsGraphStoreCatalogFacadeSchema,
	user: GdsUserSchema,
	databaseId: GdsDatabaseIdSchema,
});

const AlgorithmsBase = z.object({
	kind: GdsApplicationFormKindSchema.optional(),
	facade: GdsAlgorithmsFacadeSchema,
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
		op: z.literal('graph_exists'),
		graphName: GdsGraphNameSchema,
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('list_graphs'),
		/** Optional filter: when present, list only this graph name. */
		graphName: GdsGraphNameSchema.optional(),
		/** If true, include a degree distribution histogram per entry (may be expensive). */
		includeDegreeDistribution: z.boolean().optional(),
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
		/** Optional node label filter. When empty, streams across all nodes. */
		nodeLabels: z.array(z.string().min(1)).default([]),
		/** If true, include node label names in each row. */
		listNodeLabels: z.boolean().default(false),
		nodeProperties: z.array(z.string().min(1)).default([]),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('stream_relationship_properties'),
		graphName: GdsGraphNameSchema,
		/** Optional relationship type filter. When empty, streams across all relationship types. */
		relationshipTypes: z.array(z.string().min(1)).default([]),
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
		projectionConfig: z
			.object({
				graphName: GdsGraphNameSchema,
				sourceGraphName: GdsGraphNameSchema.optional(),
				/** Java parity: "*" means PROJECT_ALL. Empty/omitted means "all". */
				nodeLabels: z.array(z.string().min(1)).optional(),
				/** Java parity: "*" means PROJECT_ALL. Empty/omitted means "all". */
				nodeProperties: z.array(z.string().min(1)).optional(),
				/** Java parity: "*" means PROJECT_ALL. Empty/omitted means "all". */
				relationshipTypes: z.array(z.string().min(1)).optional(),
				/** Java parity: "*" means PROJECT_ALL. Empty/omitted means "all". */
				relationshipProperties: z.array(z.string().min(1)).optional(),
				/** Per-type property selector map: { [relationshipType]: propertyKey } */
				relationshipPropertySelectors: z.record(z.string().min(1), z.string().min(1)).optional(),
				/** Default weight property for algorithms (may be overridden per relationship type). */
				weightProperty: z.string().min(1).optional(),
				fictitiousLoading: z.boolean().optional(),
			})
			.passthrough(),
	}),

	GraphStoreCatalogBase.extend({
		op: z.literal('project_generic'),
		projectionConfig: z
			.object({
				graphName: GdsGraphNameSchema,
				sourceGraphName: GdsGraphNameSchema.optional(),
				nodeLabels: z.array(z.string().min(1)).optional(),
				nodeProperties: z.array(z.string().min(1)).optional(),
				relationshipTypes: z.array(z.string().min(1)).optional(),
				relationshipProperties: z.array(z.string().min(1)).optional(),
				relationshipPropertySelectors: z.record(z.string().min(1), z.string().min(1)).optional(),
				weightProperty: z.string().min(1).optional(),
				fictitiousLoading: z.boolean().optional(),
			})
			.passthrough(),
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

/**
 * Algorithm execution mode (Java GDS parity).
 * - stream: Returns individual result rows
 * - stats: Returns aggregated statistics only
 * - mutate: Writes results to in-memory graph projection
 * - write: Writes results back to database
 */
export const AlgorithmModeSchema = z
	.enum(['stream', 'stats', 'mutate', 'write'])
	.default('stream');
export type AlgorithmMode = z.infer<typeof AlgorithmModeSchema>;

export const GdsAlgorithmsCallSchema = z.discriminatedUnion('op', [
	// ============================================================================
	// Pathfinding Algorithms (unified with mode parameter)
	// ============================================================================
	AlgorithmsBase.extend({
		op: z.literal('bfs'),
		mode: AlgorithmModeSchema,
		graphName: GdsGraphNameSchema,
		sourceNode: z.number().int().nonnegative(),
		targetNodes: z.array(z.number().int().nonnegative()).optional(),
		maxDepth: z.number().int().positive().optional(),
		trackPaths: z.boolean().optional(),
		concurrency: z.number().int().positive().optional(),
		delta: z.number().int().positive().optional(),
	}),
	AlgorithmsBase.extend({
		op: z.literal('dfs'),
		mode: AlgorithmModeSchema,
		graphName: GdsGraphNameSchema,
		sourceNode: z.number().int().nonnegative(),
		targetNodes: z.array(z.number().int().nonnegative()).optional(),
		maxDepth: z.number().int().positive().optional(),
		trackPaths: z.boolean().optional(),
		concurrency: z.number().int().positive().optional(),
	}),
	AlgorithmsBase.extend({
		op: z.literal('dijkstra'),
		mode: AlgorithmModeSchema,
		graphName: GdsGraphNameSchema,
		sourceNode: z.number().int().nonnegative(),
		targetNode: z.number().int().nonnegative().optional(),
		targetNodes: z.array(z.number().int().nonnegative()).optional(),
		weightProperty: z.string().min(1).optional(),
		direction: z.enum(['incoming', 'outgoing', 'both']).optional(),
		trackRelationships: z.boolean().optional(),
		concurrency: z.number().int().positive().optional(),
	}),
	AlgorithmsBase.extend({
		op: z.literal('bellman_ford'),
		mode: AlgorithmModeSchema,
		graphName: GdsGraphNameSchema,
		sourceNode: z.number().int().nonnegative(),
		weightProperty: z.string().min(1).optional(),
		direction: z.enum(['incoming', 'outgoing']).optional(),
		relationshipTypes: z.array(z.string().min(1)).optional(),
		trackNegativeCycles: z.boolean().optional(),
		trackPaths: z.boolean().optional(),
		concurrency: z.number().int().positive().optional(),
	}),
	AlgorithmsBase.extend({
		op: z.literal('astar'),
		mode: AlgorithmModeSchema,
		graphName: GdsGraphNameSchema,
		sourceNode: z.number().int().nonnegative(),
		targetNode: z.number().int().nonnegative().optional(),
		targetNodes: z.array(z.number().int().nonnegative()).optional(),
		weightProperty: z.string().min(1).optional(),
		direction: z.enum(['incoming', 'outgoing']).optional(),
		relationshipTypes: z.array(z.string().min(1)).optional(),
		heuristic: z.enum(['manhattan', 'euclidean', 'haversine']).optional(),
		concurrency: z.number().int().positive().optional(),
	}),
	AlgorithmsBase.extend({
		op: z.literal('delta_stepping'),
		mode: AlgorithmModeSchema,
		graphName: GdsGraphNameSchema,
		sourceNode: z.number().int().nonnegative(),
		delta: z.number().positive().optional(),
		weightProperty: z.string().min(1).optional(),
		direction: z.enum(['incoming', 'outgoing']).optional(),
		relationshipTypes: z.array(z.string().min(1)).optional(),
		concurrency: z.number().int().positive().optional(),
	}),
	AlgorithmsBase.extend({
		op: z.literal('yens'),
		mode: AlgorithmModeSchema,
		graphName: GdsGraphNameSchema,
		sourceNode: z.number().int().nonnegative(),
		targetNode: z.number().int().nonnegative(),
		k: z.number().int().positive().optional(),
		weightProperty: z.string().min(1).optional(),
		direction: z.enum(['incoming', 'outgoing']).optional(),
		relationshipTypes: z.array(z.string().min(1)).optional(),
		concurrency: z.number().int().positive().optional(),
	}),
	AlgorithmsBase.extend({
		op: z.literal('all_shortest_paths'),
		mode: AlgorithmModeSchema,
		graphName: GdsGraphNameSchema,
		weighted: z.boolean().optional(),
		weightProperty: z.string().min(1).optional(),
		direction: z.enum(['incoming', 'outgoing', 'undirected']).optional(),
		relationshipTypes: z.array(z.string().min(1)).optional(),
		maxResults: z.number().int().positive().optional(),
		concurrency: z.number().int().positive().optional(),
	}),
	AlgorithmsBase.extend({
		op: z.literal('spanning_tree'),
		mode: AlgorithmModeSchema,
		graphName: GdsGraphNameSchema,
		startNode: z.number().int().nonnegative().optional(),
		computeMinimum: z.boolean().optional(),
		weightProperty: z.string().min(1).optional(),
		direction: z.enum(['incoming', 'outgoing', 'undirected']).optional(),
		relationshipTypes: z.array(z.string().min(1)).optional(),
		concurrency: z.number().int().positive().optional(),
	}),
	AlgorithmsBase.extend({
		op: z.literal('topological_sort'),
		mode: AlgorithmModeSchema,
		graphName: GdsGraphNameSchema,
		computeMaxDistance: z.boolean().optional(),
		concurrency: z.number().int().positive().optional(),
	}),
	AlgorithmsBase.extend({
		op: z.literal('random_walk'),
		mode: AlgorithmModeSchema,
		graphName: GdsGraphNameSchema,
		walksPerNode: z.number().int().positive().optional(),
		walkLength: z.number().int().positive().optional(),
		returnFactor: z.number().positive().optional(),
		inOutFactor: z.number().positive().optional(),
		sourceNodes: z.array(z.number().int().nonnegative()).optional(),
		randomSeed: z.number().int().nonnegative().optional(),
		concurrency: z.number().int().positive().optional(),
	}),
	// ============================================================================
	// Centrality Algorithms (future)
	// ============================================================================
	AlgorithmsBase.extend({
		op: z.literal('pagerank'),
		mode: AlgorithmModeSchema,
		graphName: GdsGraphNameSchema,
		relationshipTypes: z.array(z.string().min(1)).default([]),
		weightProperty: z.string().min(1).optional(),
		relationshipPropertySelectors: z.record(z.string().min(1), z.string().min(1)).optional(),
		config: z
			.object({
				maxIterations: z.number().int().positive().optional(),
				tolerance: z.number().positive().optional(),
				dampingFactor: z.number().min(0).max(1).optional(),
			})
			.optional(),
	}),
]);
export type GdsAlgorithmsCall = z.infer<typeof GdsAlgorithmsCallSchema>;

export const GdsApplicationCallSchema = z.union([
	GdsGraphStoreCatalogCallSchema,
	GdsAlgorithmsCallSchema,
	GdsFormEvalCallSchema,
]);
export type GdsApplicationCall = z.infer<typeof GdsApplicationCallSchema>;

export function gdsApplicationOperationId(call: GdsApplicationCall): string {
	return `gds.${call.facade}.${call.op}`;
}


