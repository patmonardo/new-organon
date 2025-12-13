import { z } from 'zod';

/**
 * Kernel models (NOT LLMs)
 *
 * These are distributed/portable inference operators: GDS procedures, GNNs,
 * classical ML kernels, scoring functions, etc.
 *
 * The Model package owns their representational interface (request/result);
 * TAW owns execution.
 */

export const KernelModelRefSchema = z.object({
  id: z.string(),
  kind: z.string().optional(),
  version: z.string().optional(),
});
export type KernelModelRef = z.infer<typeof KernelModelRefSchema>;

export const KernelRunRequestSchema = z.object({
  model: KernelModelRefSchema,
  /**
   * Input is intentionally flexible: could be a graph reference, feature matrix,
   * query spec, or any transportable payload.
   */
  input: z.unknown(),
  params: z.record(z.string(), z.unknown()).optional(),
});
export type KernelRunRequest = z.infer<typeof KernelRunRequestSchema>;

export const KernelRunResultSchema = z.object({
  ok: z.boolean(),
  output: z.unknown().optional(),
  error: z.unknown().optional(),
});
export type KernelRunResult = z.infer<typeof KernelRunResultSchema>;

export const KERNEL_TAW_ACTIONS = {
  run: 'kernel.run',
} as const;
