import type { FactTraceEvent, EventMeta } from './fact-trace';
import type { KernelPort } from './kernel-port';
import type { KernelRunRequest, KernelRunResult } from './kernel';
import type { TawActEvent, TawResultEvent } from './taw-schema';
import {
  kernelRunRequestToTawActEvent,
  kernelRunResultToTawResultEvent,
} from './agent-to-taw';
import { kernelRunToFactTraceEvents } from './kernel-trace';

export type KernelOrganicUnityOptions = {
  goalId: string;
  stepId?: string;
  correlationId?: string;
  source?: string;

  /**
   * Shared meta for both TAW events and FactTrace events.
   * Keep this structural: narration belongs in TS space, not in the kernel.
   */
  meta?: EventMeta;

  /** Optional stable identifier for this run in trace space. */
  runId?: string;
};

export type KernelOrganicUnity = {
  request: KernelRunRequest;
  result: KernelRunResult;

  /** Discursive print (TAW). */
  taw: {
    act: TawActEvent;
    result: TawResultEvent;
  };

  /** Discursive print (FactTrace). */
  trace: FactTraceEvent[];

  /** The runId actually used (if any). */
  runId?: string;
};

function deriveRunId(opts: KernelOrganicUnityOptions): string | undefined {
  if (opts.runId) return opts.runId;
  if (opts.correlationId && opts.stepId) return `${opts.correlationId}:${opts.stepId}:kernel.run`;
  if (opts.correlationId) return `${opts.correlationId}:kernel.run`;
  return undefined;
}

/**
 * Organic unity helper: runs a kernel (Knowing) and returns its prints in TS space (Conceiving/Thinking).
 *
 * - Kernel execution is sublingual lawful activity.
 * - TS artifacts are the Eval → Print moment.
 */
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

  const trace = kernelRunToFactTraceEvents(request, result, {
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
