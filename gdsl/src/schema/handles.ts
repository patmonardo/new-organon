import { z } from 'zod';

import { GdsGraphNameSchema } from './common';

/**
 * Handle-ish response shapes (client-visible references)
 *
 * These are intentionally small and stable: for large “manifolds of sensation”
 * (algorithm streams, ML pipeline outputs, embeddings), the kernel persists
 * “intuition” into GraphStores and returns *references*.
 *
 * In other words: **kernel emits handles**, not terabyte payloads.
 */

/**
 * A reference to a graph stored in the kernel’s GraphCatalog/GraphStore layer.
 *
 * - Interpreted as FormDB/HyperStore addressability at the protocol level.
 * - The graph can then be queried/streamed/exported via GraphStore APIs.
 */
export const GdsGraphHandleSchema = z.object({
	graphName: GdsGraphNameSchema,
});
export type GdsGraphHandle = z.infer<typeof GdsGraphHandleSchema>;


