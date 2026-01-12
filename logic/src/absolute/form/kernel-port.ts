import { z } from 'zod';

// Kernel execution boundary (JSON-first). TS authors requests; adapters run them.

export const KernelModelRefSchema = z.object({
  id: z.string(),
  kind: z.string().optional(),
  version: z.string().optional(),
});
export type KernelModelRef = z.infer<typeof KernelModelRefSchema>;

export const KernelRunRequestSchema = z.object({
  model: KernelModelRefSchema,
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

export const KERNEL_ACTIONS = {
  run: 'kernel.run',
} as const;

export interface KernelPort {
  readonly name: string;
  run(request: KernelRunRequest): Promise<KernelRunResult>;
}

export function invokeKernelModel(
  port: KernelPort,
  call: { facade: string; op: string } & Record<string, unknown>,
) {
  const modelId = `gds.${call.facade}.${call.op}`;
  return port.run({ model: { id: modelId }, input: call });
}
