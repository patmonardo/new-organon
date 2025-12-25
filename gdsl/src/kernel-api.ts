import { z } from 'zod';

/**
 * Kernel API (TS-side contract)
 *
 * This module is the canonical TypeScript surface for describing kernel-like
 * computation (GDS procedures, GNNs, ML kernels, scoring functions, etc.).
 *
 * It intentionally does NOT provide an implementation adapter.
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

/**
 * Stable action identifiers for execution/event streams.
 *
 * Note: execution happens outside this package; these constants are just a
 * shared language boundary.
 */
export const KERNEL_ACTIONS = {
  run: 'kernel.run',
} as const;

/**
 * KernelPort (execution boundary)
 *
 * Implementations live in higher layers (Task, adapters, servers).
 *
 * Terminology:
 * - When the target is the Rust GDS kernel, a `KernelPort` implementation is
 *   effectively a **GDS-L (GDS Link)** transport adapter.
 */
export interface KernelPort {
  readonly name: string;
  run(request: KernelRunRequest): Promise<KernelRunResult>;
}
