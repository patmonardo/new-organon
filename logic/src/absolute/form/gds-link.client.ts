import { z } from 'zod';

import type { KernelPort } from '@organon/gdsl';
import {
  invokeGdsApplicationCall,
  type GdsFormEvalEvaluateData,
  GdsFormEvalEvaluateDataSchema,
  type GdsApplicationCall,
  type GdsDatabaseId,
  type GdsUser,
} from '@organon/gdsl';

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
  user: GdsUser;
  databaseId: GdsDatabaseId;
};

// --- Response schemas (typed “Given” / Intuition payloads) ---

export const GraphStoreCatalogEntrySchema = z.object({
  name: z.string().min(1),
  nodeCount: z.number().int().nonnegative(),
  relationshipCount: z.number().int().nonnegative(),
});
export type GraphStoreCatalogEntry = z.infer<typeof GraphStoreCatalogEntrySchema>;

export const ListGraphsDataSchema = z.object({
  entries: z.array(GraphStoreCatalogEntrySchema),
});
export type ListGraphsData = z.infer<typeof ListGraphsDataSchema>;

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

export const GraphStorePutDataSchema = z.object({
  graphName: z.string().min(1),
  nodeCount: z.number().int().nonnegative(),
  relationshipCount: z.number().int().nonnegative(),
});
export type GraphStorePutData = z.infer<typeof GraphStorePutDataSchema>;

function baseForm(session: AbsoluteFormSession) {
  return {
    kind: 'ApplicationForm' as const,
    user: session.user,
    databaseId: session.databaseId,
  };
}

export function createAbsoluteFormClient(port: KernelPort, session: AbsoluteFormSession) {
  const base = baseForm(session);

  async function call(call: GdsApplicationCall): Promise<unknown> {
    return await invokeGdsApplicationCall(port, call);
  }

  return {
    /** Raw escape hatch (sometimes useful while the protocol is evolving). */
    call,

    graphStoreCatalog: {
      listGraphs: async (): Promise<ListGraphsData> => {
        const data = await call({ ...base, facade: 'graph_store_catalog', op: 'list_graphs' } as any);
        return ListGraphsDataSchema.parse(data);
      },

      graphMemoryUsage: async (graphName: string): Promise<GraphMemoryUsageData> => {
        const data = await call({ ...base, facade: 'graph_store_catalog', op: 'graph_memory_usage', graphName } as any);
        return GraphMemoryUsageDataSchema.parse(data);
      },

      dropGraph: async (graphName: string, opts?: { failIfMissing?: boolean }): Promise<DropGraphsData> => {
        const data = await call({
          ...base,
          facade: 'graph_store_catalog',
          op: 'drop_graph',
          graphName,
          failIfMissing: opts?.failIfMissing ?? false,
        } as any);
        return DropGraphsDataSchema.parse(data);
      },

      dropGraphs: async (graphNames: string[], opts?: { failIfMissing?: boolean }): Promise<DropGraphsData> => {
        const data = await call({
          ...base,
          facade: 'graph_store_catalog',
          op: 'drop_graphs',
          graphNames,
          failIfMissing: opts?.failIfMissing ?? false,
        } as any);
        return DropGraphsDataSchema.parse(data);
      },
    },

    graphStore: {
      put: async (graphName: string, snapshot: unknown): Promise<GraphStorePutData> => {
        const data = await call({ ...base, facade: 'graph_store', op: 'put', graphName, snapshot } as any);
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


