import { z } from 'zod';

import { GdsGraphNameSchema } from './common';
import { gdsApplicationBase } from './application';

/**
 * Graph Store facade.
 *
 * See: gds/src/applications/services/graph_store_dispatch.rs
 */
export const GdsGraphStoreFacadeSchema = z.literal('graph_store');
export type GdsGraphStoreFacade = z.infer<typeof GdsGraphStoreFacadeSchema>;

const GraphStoreBase = gdsApplicationBase(GdsGraphStoreFacadeSchema);

export const PutGraphStoreCallSchema = GraphStoreBase.extend({
  op: z.literal('put'),
  graphName: GdsGraphNameSchema,
  snapshot: z
    .object({
      nodes: z.array(z.number().int()).min(1),
      relationships: z
        .array(
          z
            .object({
              type: z.string().min(1),
              source: z.number().int(),
              target: z.number().int(),
              properties: z.record(z.string(), z.unknown()).optional(),
            })
            .passthrough(),
        )
        .optional(),
      nodeProperties: z.record(z.string(), z.array(z.unknown())).optional(),
    })
    .passthrough(),
});

export const GdsGraphStoreCallSchema = z.discriminatedUnion('op', [
  PutGraphStoreCallSchema,
]);
export type GdsGraphStoreCall = z.infer<typeof GdsGraphStoreCallSchema>;
