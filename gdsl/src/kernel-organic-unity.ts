import type { TawActEvent, TawResultEvent } from '@organon/task';

import type { KernelPort, KernelRunRequest, KernelRunResult } from './kernel-api';
import type { EventMeta, TraceEvent } from './trace';
import { kernelRunRequestToTawActEvent, kernelRunResultToTawResultEvent } from './taw-kernel';
import { kernelRunToTraceEvents } from './kernel-trace';

export type KernelOrganicUnityOptions = {
  goalId: string;
  stepId?: string;
  correlationId?: string;
  source?: string;

  /** Shared meta for both TAW and trace prints. */
  meta?: EventMeta;

  /** Optional stable identifier for this run in trace space. */
  runId?: string;
};

export type KernelOrganicUnity = {
  request: KernelRunRequest;
  result: KernelRunResult;

  taw: {
    act: TawActEvent;
    result: TawResultEvent;
  };

  trace: TraceEvent[];
  runId?: string;
};

function deriveRunId(opts: KernelOrganicUnityOptions): string | undefined {
  if (opts.runId) return opts.runId;
  if (opts.correlationId && opts.stepId) return `${opts.correlationId}:${opts.stepId}:kernel.run`;
  if (opts.correlationId) return `${opts.correlationId}:kernel.run`;
  return undefined;
}

export async function runKernelOrganicUnity(
  port: KernelPort,
  request: KernelRunRequest,
  opts: KernelOrganicUnityOptions,
): Promise<KernelOrganicUnity> {
  const act = kernelRunRequestToTawActEvent(request, {
    goalId: opts.goalId,
    stepId: opts.stepId,
    correlationId: opts.correlationId,
    source: opts.source,
    meta: opts.meta,
  });

  const result = await port.run(request);

  const resultEvent = kernelRunResultToTawResultEvent(result, {
    goalId: opts.goalId,
    stepId: opts.stepId,
    correlationId: opts.correlationId,
    source: opts.source,
    meta: opts.meta,
  });

  const runId = deriveRunId(opts);

  const trace = kernelRunToTraceEvents(request, result, {
    runId,
    meta: opts.meta,
  });

  return {
    request,
    result,
    taw: { act, result: resultEvent },
    trace,
    runId,
  };
}
