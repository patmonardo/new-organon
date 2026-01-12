import { z } from 'zod';

import { GdsGraphNameSchema } from './common';
import { gdsApplicationBase } from './application';

/**
 * Graph Store Catalog facade (mirrors Java: `GraphCatalogApplications`)
 *
 * See:
 * - gds/src/applications/graph_store_catalog/java/GraphCatalogApplications.java
 * - gds/src/applications/services/graph_store_catalog_dispatch.rs
 */
export const GdsGraphStoreCatalogFacadeSchema = z.literal(
  'graph_store_catalog',
);
export type GdsGraphStoreCatalogFacade = z.infer<
  typeof GdsGraphStoreCatalogFacadeSchema
>;

const GraphStoreCatalogBase = gdsApplicationBase(
  GdsGraphStoreCatalogFacadeSchema,
);

// ---- Individual “Application” calls (named after Java domain apps) ----

export const GraphExistsCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('graphExists'),
  graphName: GdsGraphNameSchema,
});

export const ListGraphsCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('listGraphs'),
  /** Optional filter: when present, list only this graph name. */
  graphName: GdsGraphNameSchema.optional(),
  /** If true, include a degree distribution histogram per entry (may be expensive). */
  includeDegreeDistribution: z.boolean().optional(),
});

export const GraphMemoryUsageCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('graphMemoryUsage'),
  graphName: GdsGraphNameSchema,
});

export const DropGraphCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('dropGraph'),
  graphName: GdsGraphNameSchema,
  failIfMissing: z.boolean().optional(),
});

export const DropGraphsCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('dropGraphs'),
  graphNames: z.array(GdsGraphNameSchema).min(1),
  failIfMissing: z.boolean().optional(),
});

export const DropNodePropertiesCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('dropNodeProperties'),
  graphName: GdsGraphNameSchema,
  nodeProperties: z.array(z.string().min(1)).min(1),
  failIfMissing: z.boolean().optional(),
});

export const DropRelationshipsCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('dropRelationships'),
  graphName: GdsGraphNameSchema,
  relationshipType: z.string().min(1),
});

export const DropGraphPropertyCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('dropGraphProperty'),
  graphName: GdsGraphNameSchema,
  graphProperty: z.string().min(1),
  failIfMissing: z.boolean().optional(),
});

export const StreamGraphPropertyCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('streamGraphProperty'),
  graphName: GdsGraphNameSchema,
  graphProperty: z.string().min(1),
});

export const StreamNodePropertiesCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('streamNodeProperties'),
  graphName: GdsGraphNameSchema,
  /** Optional node label filter. When empty, streams across all nodes. */
  nodeLabels: z.array(z.string().min(1)).optional(),
  /** If true, include node label names in each row. */
  listNodeLabels: z.boolean().optional(),
  nodeProperties: z.array(z.string().min(1)).optional(),
});

export const StreamRelationshipPropertiesCallSchema =
  GraphStoreCatalogBase.extend({
    op: z.literal('streamRelationshipProperties'),
    graphName: GdsGraphNameSchema,
    /** Optional relationship type filter. When empty, streams across all relationship types. */
    relationshipTypes: z.array(z.string().min(1)).optional(),
    relationshipProperties: z.array(z.string().min(1)).optional(),
  });

export const StreamRelationshipsCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('streamRelationships'),
  graphName: GdsGraphNameSchema,
  relationshipTypes: z.array(z.string().min(1)).optional(),
});

export const GenerateGraphCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('generateGraph'),
  generationConfig: z.unknown(),
});

export const GenerateGraphStatsCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('generateGraphStats'),
  graphName: GdsGraphNameSchema,
  nodeCount: z.number().int().nonnegative(),
  averageDegree: z.number().int().nonnegative(),
  configuration: z.record(z.string(), z.unknown()).optional(),
});

export const ExportToCsvEstimateCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('exportToCsvEstimate'),
  graphName: GdsGraphNameSchema,
});

export const SubGraphProjectCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('subGraphProject'),
  graphName: GdsGraphNameSchema,
  originGraphName: GdsGraphNameSchema,
  nodeFilter: z.string().min(1),
  relationshipFilter: z.string().min(1),
  configuration: z.record(z.string(), z.unknown()).optional(),
});

export const SampleRandomWalkWithRestartsCallSchema =
  GraphStoreCatalogBase.extend({
    op: z.literal('sampleRandomWalkWithRestarts'),
    graphName: GdsGraphNameSchema,
    originGraphName: GdsGraphNameSchema,
    configuration: z.record(z.string(), z.unknown()).optional(),
  });

export const SampleCommonNeighbourAwareRandomWalkCallSchema =
  GraphStoreCatalogBase.extend({
    op: z.literal('sampleCommonNeighbourAwareRandomWalk'),
    graphName: GdsGraphNameSchema,
    originGraphName: GdsGraphNameSchema,
    configuration: z.record(z.string(), z.unknown()).optional(),
  });

export const EstimateCommonNeighbourAwareRandomWalkCallSchema =
  GraphStoreCatalogBase.extend({
    op: z.literal('estimateCommonNeighbourAwareRandomWalk'),
    graphName: GdsGraphNameSchema,
    configuration: z.record(z.string(), z.unknown()).optional(),
  });

export const EstimateNativeProjectCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('estimateNativeProject'),
  projectionConfig: z.unknown(),
});

export const CypherProjectCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('cypherProject'),
  graphName: GdsGraphNameSchema,
  nodeQuery: z.string().min(1),
  relationshipQuery: z.string().min(1),
  configuration: z.record(z.string(), z.unknown()).optional(),
});

export const EstimateCypherProjectCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('estimateCypherProject'),
  nodeQuery: z.string().min(1),
  relationshipQuery: z.string().min(1),
  configuration: z.record(z.string(), z.unknown()).optional(),
});

export const MutateLabelCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('mutateLabel'),
  graphName: GdsGraphNameSchema,
  nodeLabel: z.string().min(1),
  mutateLabelConfig: z.record(z.string(), z.unknown()),
});

export const SampleGraphCallSchema = GraphStoreCatalogBase.extend({
  op: z.literal('sampleGraph'),
  graphName: GdsGraphNameSchema,
  samplingConfig: z.unknown(),
});

/**
 * Canonical union for graph_store_catalog calls.
 */
export const GdsGraphStoreCatalogCallSchema = z.discriminatedUnion('op', [
  GraphExistsCallSchema,
  ListGraphsCallSchema,
  GraphMemoryUsageCallSchema,
  DropGraphCallSchema,
  DropGraphsCallSchema,
  DropNodePropertiesCallSchema,
  DropRelationshipsCallSchema,
  DropGraphPropertyCallSchema,
  StreamGraphPropertyCallSchema,
  StreamNodePropertiesCallSchema,
  StreamRelationshipPropertiesCallSchema,
  StreamRelationshipsCallSchema,
  GenerateGraphCallSchema,
  GenerateGraphStatsCallSchema,
  ExportToCsvEstimateCallSchema,
  SubGraphProjectCallSchema,
  SampleRandomWalkWithRestartsCallSchema,
  SampleCommonNeighbourAwareRandomWalkCallSchema,
  EstimateCommonNeighbourAwareRandomWalkCallSchema,
  EstimateNativeProjectCallSchema,
  CypherProjectCallSchema,
  EstimateCypherProjectCallSchema,
  MutateLabelCallSchema,
  SampleGraphCallSchema,
]);
export type GdsGraphStoreCatalogCall = z.infer<
  typeof GdsGraphStoreCatalogCallSchema
>;
