import type { KernelRunRequest, KernelRunResult } from './kernel-port';
import type { EventMeta, FactStoreInfo, TraceEvent } from './trace';

export const KERNEL_TRACE_KINDS = {
  request: 'kernel.run.request',
  result: 'kernel.run.result',
} as const;

function mergeFactStore(
  base: FactStoreInfo | undefined,
  override: Partial<FactStoreInfo> | undefined,
): FactStoreInfo | undefined {
  if (!base && !override) return undefined;
  return {
    ...(base ?? {}),
    ...(override ?? {}),
    ids:
      override?.ids !== undefined
        ? override.ids
        : base?.ids !== undefined
        ? base.ids
        : undefined,
  };
}

export function kernelRunRequestToTraceEvent(
  request: KernelRunRequest,
  opts?: {
    runId?: string;
    meta?: EventMeta;
    factStore?: Partial<FactStoreInfo>;
  },
): TraceEvent<KernelRunRequest> {
  const meta: EventMeta | undefined =
    opts?.meta || opts?.runId || opts?.factStore
      ? {
          ...(opts?.meta ?? {}),
          factStore: mergeFactStore(
            {
              store: 'kernel',
              kind: 'kernel.run',
              op: 'index',
              ids: opts?.runId ? [opts.runId] : undefined,
              note: `Kernel run requested: ${request.model.id}`,
            },
            opts?.factStore,
          ),
        }
      : undefined;

  return {
    kind: KERNEL_TRACE_KINDS.request,
    payload: request,
    meta,
  };
}

export function kernelRunResultToTraceEvent(
  result: KernelRunResult,
  opts?: {
    runId?: string;
    request?: KernelRunRequest;
    meta?: EventMeta;
    factStore?: Partial<FactStoreInfo>;
  },
): TraceEvent<
  KernelRunResult | { request: KernelRunRequest; result: KernelRunResult }
> {
  const meta: EventMeta | undefined =
    opts?.meta || opts?.runId || opts?.factStore
      ? {
          ...(opts?.meta ?? {}),
          factStore: mergeFactStore(
            {
              store: 'kernel',
              kind: 'kernel.run',
              op: result.ok ? 'project' : 'index',
              ids: opts?.runId ? [opts.runId] : undefined,
              note: result.ok ? 'Kernel run succeeded' : 'Kernel run failed',
            },
            opts?.factStore,
          ),
        }
      : undefined;

  return {
    kind: KERNEL_TRACE_KINDS.result,
    payload: opts?.request ? { request: opts.request, result } : result,
    meta,
  };
}

export function kernelRunToTraceEvents(
  request: KernelRunRequest,
  result: KernelRunResult,
  opts?: {
    runId?: string;
    meta?: EventMeta;
    requestFactStore?: Partial<FactStoreInfo>;
    resultFactStore?: Partial<FactStoreInfo>;
  },
): TraceEvent[] {
  return [
    kernelRunRequestToTraceEvent(request, {
      runId: opts?.runId,
      meta: opts?.meta,
      factStore: opts?.requestFactStore,
    }),
    kernelRunResultToTraceEvent(result, {
      runId: opts?.runId,
      meta: opts?.meta,
      factStore: opts?.resultFactStore,
      request,
    }),
  ];
}
