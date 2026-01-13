import { z } from 'zod';

import type { KernelPort } from './kernel-port';
import {
  GdsFormEvalEvaluateDataSchema,
  GdsFormEvalEvaluateData,
} from '../../schema/program';

/**
 * Absolute / Form (Logic)
 *
 * This is the **pure TS** “rich API” over the GDS Link (GDS-L) application facades.
 *
 * - The **transport** is provided (e.g. `KernelPort` bound to TSJSON/NAPI).
 * - The **Form** is constructed here as a typed Application Form.
 * - The **Given** returns as handle-first data (GraphStore/Intuition refs + proof/meta).
 */

export type AbsoluteFormSession = {
  user: { username: string; isAdmin?: boolean };
  databaseId: string;
};

type ApplicationCall = { facade: string; op: string } & Record<string, unknown>;

async function invokeApplicationCall(
  port: KernelPort,
  call: ApplicationCall,
): Promise<unknown> {
  const modelId = `gds.${call.facade}.${call.op}`;
  const result = await port.run({ model: { id: modelId }, input: call });
  if (result.ok) return result.output;
  throw new Error(
    typeof (result as any)?.error?.message === 'string'
      ? (result as any).error.message
      : 'GDS call failed',
  );
}

// --- Response schemas (typed “Given” / Intuition payloads) ---

export const GraphStoreCatalogEntrySchema = z.object({
  name: z.string().min(1),
  nodeCount: z.number().int().nonnegative(),
  relationshipCount: z.number().int().nonnegative(),
  // JSON object where keys are degrees (stringified ints) and values are counts.
  degreeDistribution: z
    .record(z.string(), z.number().int().nonnegative())
    .optional(),
});
export type GraphStoreCatalogEntry = z.infer<
  typeof GraphStoreCatalogEntrySchema
>;

export const ListGraphsDataSchema = z.object({
  entries: z.array(GraphStoreCatalogEntrySchema),
});
export type ListGraphsData = z.infer<typeof ListGraphsDataSchema>;

export const GraphExistsDataSchema = z.object({
  graphName: z.string().min(1),
  exists: z.boolean(),
});
export type GraphExistsData = z.infer<typeof GraphExistsDataSchema>;

export const GraphMemoryUsageDataSchema = z.object({
  graphName: z.string().min(1),
  bytes: z.number().int().nonnegative(),
  nodes: z.number().int().nonnegative(),
  relationships: z.number().int().nonnegative(),
});
export type GraphMemoryUsageData = z.infer<typeof GraphMemoryUsageDataSchema>;

export const DropGraphsDataSchema = z.object({
  dropped: z.array(GraphStoreCatalogEntrySchema),
});
export type DropGraphsData = z.infer<typeof DropGraphsDataSchema>;

export const StreamRelationshipsDataSchema = z.object({
  graphName: z.string().min(1),
  relationships: z.array(
    z.object({
      sourceNodeId: z.number().int(),
      targetNodeId: z.number().int(),
      relationshipType: z.string().min(1),
    }),
  ),
});
export type StreamRelationshipsData = z.infer<
  typeof StreamRelationshipsDataSchema
>;

export const StreamNodePropertiesDataSchema = z.object({
  graphName: z.string().min(1),
  rows: z.array(
    z.object({
      nodeId: z.number().int(),
      nodeProperty: z.string().min(1),
      propertyValue: z.unknown(),
      nodeLabels: z.array(z.string()).default([]),
    }),
  ),
});
export type StreamNodePropertiesData = z.infer<
  typeof StreamNodePropertiesDataSchema
>;

export const StreamRelationshipPropertiesDataSchema = z.object({
  graphName: z.string().min(1),
  rows: z.array(
    z.object({
      sourceNodeId: z.number().int(),
      targetNodeId: z.number().int(),
      relationshipType: z.string().min(1),
      relationshipProperty: z.string().min(1),
      propertyValue: z.unknown(),
    }),
  ),
});
export type StreamRelationshipPropertiesData = z.infer<
  typeof StreamRelationshipPropertiesDataSchema
>;

export const GraphStorePutDataSchema = z.object({
  graphName: z.string().min(1),
  nodeCount: z.number().int().nonnegative(),
  relationshipCount: z.number().int().nonnegative(),
});
export type GraphStorePutData = z.infer<typeof GraphStorePutDataSchema>;

// Reuse the canonical schema defined in `schema/program.ts`.

function baseForm(session: AbsoluteFormSession) {
  return {
    kind: 'ApplicationForm' as const,
    user: session.user,
    databaseId: session.databaseId,
  };
}

export function createAbsoluteFormClient(
  port: KernelPort,
  session: AbsoluteFormSession,
) {
  const base = baseForm(session);

  async function call(call: ApplicationCall): Promise<unknown> {
    return await invokeApplicationCall(port, call);
  }

  return {
    /** Raw escape hatch (sometimes useful while the protocol is evolving). */
    call,

    graphStoreCatalog: {
      graphExists: async (graphName: string): Promise<GraphExistsData> => {
        const data = await call({
          ...base,
          facade: 'graph_store_catalog',
          op: 'graph_exists',
          graphName,
        } as any);
        return GraphExistsDataSchema.parse(data);
      },

      listGraphs: async (opts?: {
        graphName?: string;
        includeDegreeDistribution?: boolean;
      }): Promise<ListGraphsData> => {
        const data = await call({
          ...base,
          facade: 'graph_store_catalog',
          op: 'list_graphs',
          graphName: opts?.graphName,
          includeDegreeDistribution: opts?.includeDegreeDistribution,
        } as any);
        return ListGraphsDataSchema.parse(data);
      },

      graphMemoryUsage: async (
        graphName: string,
      ): Promise<GraphMemoryUsageData> => {
        const data = await call({
          ...base,
          facade: 'graph_store_catalog',
          op: 'graph_memory_usage',
          graphName,
        } as any);
        return GraphMemoryUsageDataSchema.parse(data);
      },

      dropGraph: async (
        graphName: string,
        opts?: { failIfMissing?: boolean },
      ): Promise<DropGraphsData> => {
        const data = await call({
          ...base,
          facade: 'graph_store_catalog',
          op: 'drop_graph',
          graphName,
          failIfMissing: opts?.failIfMissing ?? false,
        } as any);
        return DropGraphsDataSchema.parse(data);
      },

      dropGraphs: async (
        graphNames: string[],
        opts?: { failIfMissing?: boolean },
      ): Promise<DropGraphsData> => {
        const data = await call({
          ...base,
          facade: 'graph_store_catalog',
          op: 'drop_graphs',
          graphNames,
          failIfMissing: opts?.failIfMissing ?? false,
        } as any);
        return DropGraphsDataSchema.parse(data);
      },

      streamRelationships: async (
        graphName: string,
        opts?: { relationshipTypes?: string[] },
      ): Promise<StreamRelationshipsData> => {
        const data = await call({
          ...base,
          facade: 'graph_store_catalog',
          op: 'stream_relationships',
          graphName,
          relationshipTypes: opts?.relationshipTypes ?? [],
        } as any);
        return StreamRelationshipsDataSchema.parse(data);
      },

      streamNodeProperties: async (
        graphName: string,
        opts?: {
          nodeProperties?: string[];
          nodeLabels?: string[];
          listNodeLabels?: boolean;
        },
      ): Promise<StreamNodePropertiesData> => {
        const data = await call({
          ...base,
          facade: 'graph_store_catalog',
          op: 'stream_node_properties',
          graphName,
          nodeProperties: opts?.nodeProperties ?? [],
          nodeLabels: opts?.nodeLabels ?? [],
          listNodeLabels: opts?.listNodeLabels ?? false,
        } as any);
        return StreamNodePropertiesDataSchema.parse(data);
      },

      streamRelationshipProperties: async (
        graphName: string,
        opts?: {
          relationshipProperties?: string[];
          relationshipTypes?: string[];
        },
      ): Promise<StreamRelationshipPropertiesData> => {
        const data = await call({
          ...base,
          facade: 'graph_store_catalog',
          op: 'stream_relationship_properties',
          graphName,
          relationshipProperties: opts?.relationshipProperties ?? [],
          relationshipTypes: opts?.relationshipTypes ?? [],
        } as any);
        return StreamRelationshipPropertiesDataSchema.parse(data);
      },
    },

    graphStore: {
      put: async (
        graphName: string,
        snapshot: unknown,
      ): Promise<GraphStorePutData> => {
        const data = await call({
          ...base,
          facade: 'graph_store',
          op: 'put',
          graphName,
          snapshot,
        } as any);
        return GraphStorePutDataSchema.parse(data);
      },
    },

    formEval: {
      evaluate: async (args: {
        graphName: string;
        outputGraphName?: string;
        program: unknown;
        artifacts?: Record<string, unknown>;
      }): Promise<GdsFormEvalEvaluateData> => {
        const data = await call({
          ...base,
          facade: 'form_eval',
          op: 'evaluate',
          graphName: args.graphName,
          outputGraphName: args.outputGraphName,
          program: args.program,
          artifacts: args.artifacts ?? {},
        } as any);
        return GdsFormEvalEvaluateDataSchema.parse(data);
      },
    },
  };
}
