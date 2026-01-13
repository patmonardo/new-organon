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

/**
 * Logic API (Singular surface for Agents)
 *
 * Intent: expose one stable TypeScript entrypoint that an Agent can call.
 *
 * - Knowing (Absolute Form / Science) lives in the Rust `gds` kernel.
 * - Transport is via KernelPort adapters (TSJSON/NAPI/etc.).
 * - Logic adds conceiving/projection helpers that construct calls and
 *   interpret results, without executing kernel code directly.
 */

export type LogicApi = {
  readonly kernel: KernelPort;
  /** Kernel Absolute Form (FormProcessor) calls. */
  form: {
    evaluate(call: FormEvalCall): Promise<KernelRunResult>;
  };
};

/**
 * Minimal call shape for the kernel FormProcessor.
 *
 * Structural on purpose so agents or helpers can build it without transport coupling.
 */
export type FormEvalCall = {
  facade: 'form_eval';
  op: 'evaluate';
  user: { username: string; isAdmin?: boolean };
  databaseId: string;
  graphName: string;
  outputGraphName?: string;
  program: { morph: { patterns: string[] } } & Record<string, unknown>;
  artifacts?: Record<string, unknown>;
} & Record<string, unknown>;

function toKernelRunRequest(
  call: { facade: string; op: string } & Record<string, unknown>,
): KernelRunRequest {
  // Canonical kernel model id convention used by KernelPort adapters.
  const modelId = `gds.${call.facade}.${call.op}`;
  const req: KernelRunRequest = {
    model: { id: modelId },
    input: call,
  };
  return req;
}

export function createLogicApi(kernel: KernelPort): LogicApi {
  return {
    kernel,
    form: {
      async evaluate(call: FormEvalCall): Promise<KernelRunResult> {
        const request: KernelRunRequest = toKernelRunRequest(call as any);
        return kernel.run(request);
      },
    },
  };
}
