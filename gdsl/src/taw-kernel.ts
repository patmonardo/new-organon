import {
  TawActEventSchema,
  TawResultEventSchema,
  type TawActEvent,
  type TawResultEvent,
} from '@organon/task';

import {
  KERNEL_TAW_ACTIONS,
  KernelRunRequestSchema,
  type KernelRunRequest,
  type KernelRunResult,
} from './kernel-api';

export type KernelRunToTawActOptions = {
  goalId: string;
  stepId?: string;
  meta?: Record<string, unknown>;
  correlationId?: string;
  source?: string;
};

export function kernelRunRequestToTawActEvent(
  request: KernelRunRequest,
  opts: KernelRunToTawActOptions,
): TawActEvent {
  const validated = KernelRunRequestSchema.parse(request);

  return TawActEventSchema.parse({
    kind: 'taw.act',
    payload: {
      goalId: opts.goalId,
      stepId: opts.stepId,
      action: KERNEL_TAW_ACTIONS.run,
      input: validated,
    },
    meta: opts.meta,
    correlationId: opts.correlationId,
    source: opts.source,
  });
}

export type KernelResultToTawResultOptions = {
  goalId: string;
  stepId?: string;
  meta?: Record<string, unknown>;
  correlationId?: string;
  source?: string;
};

export function kernelRunResultToTawResultEvent(
  result: KernelRunResult,
  opts: KernelResultToTawResultOptions,
): TawResultEvent {
  return TawResultEventSchema.parse({
    kind: 'taw.result',
    payload: {
      goalId: opts.goalId,
      stepId: opts.stepId,
      ok: result.ok,
      output: result.ok ? result.output : undefined,
      error: result.ok ? undefined : result.error,
    },
    meta: opts.meta,
    correlationId: opts.correlationId,
    source: opts.source,
  });
}
