import type {
  KernelPort,
  KernelRunRequest,
  KernelRunResult,
} from './kernel-port';

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
