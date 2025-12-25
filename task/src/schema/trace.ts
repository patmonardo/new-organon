import { z } from 'zod';

/**
 * Trace vocabulary (structural)
 *
 * NOTE: This previously lived in @organon/gdsl. It now lives in @organon/task
 * so agent runtimes can operate without importing agent-oriented schemas from GDSL.
 */

const FactStoreMetaSchema = z
  .object({
    op: z.enum(['assert', 'retract', 'revise', 'index', 'project']).optional(),
    kind: z.string().optional(),
    ids: z.array(z.string()).optional(),
    note: z.string().optional(),
    mode: z.string().optional(),
    store: z.string().optional(),
  })
  .passthrough();

export const TraceEventMetaSchema = z
  .object({
    factStore: FactStoreMetaSchema.optional(),
  })
  .passthrough();

export const TraceEventSchema = z.object({
  kind: z.string(),
  payload: z.unknown().optional(),
  meta: TraceEventMetaSchema.optional(),
});
export type TraceEvent = z.infer<typeof TraceEventSchema>;


