/**
 * RealityPipe (Model Middleware)
 *
 * Internal, in-process event pipe for the Model SDSL runtime.
 *
 * Design intent:
 * - In-memory, synchronous, typed.
 * - No external transport assumptions (no HTTP, no Nest, no queues).
 * - Minimal envelope to support correlation + provenance.
 */

import { assertKernelConclusiveAllowed } from '@organon/task';

export type RealityPipeId = string;

export type RealityPipeEnvelope<
  TKind extends string = string,
  TPayload = unknown,
  TMeta = unknown,
> = {
  id: RealityPipeId;
  ts: number;
  kind: TKind;
  payload: TPayload;
  meta?: TMeta;
  correlationId?: RealityPipeId;
  source?: string;
};

export type RealityPipePublishInput<
  TKind extends string = string,
  TPayload = unknown,
  TMeta = unknown,
> =
  | RealityPipeEnvelope<TKind, TPayload, TMeta>
  | {
      kind: TKind;
      payload: TPayload;
      meta?: TMeta;
      correlationId?: RealityPipeId;
      source?: string;
    };

export type RealityPipeHandler<
  TKind extends string = string,
  TPayload = unknown,
  TMeta = unknown,
> = (envelope: RealityPipeEnvelope<TKind, TPayload, TMeta>) => void;

export type RealityPipeSubscribeOptions<
  TKind extends string = string,
  TPayload = unknown,
  TMeta = unknown,
> = {
  kind?: TKind | readonly TKind[];
  predicate?: (envelope: RealityPipeEnvelope<TKind, TPayload, TMeta>) => boolean;
};

export interface RealityPipe<TKind extends string = string, TPayload = unknown, TMeta = unknown> {
  publish(input: RealityPipePublishInput<TKind, TPayload, TMeta>): RealityPipeEnvelope<TKind, TPayload, TMeta>;
  subscribe(
    handler: RealityPipeHandler<TKind, TPayload, TMeta>,
    opts?: RealityPipeSubscribeOptions<TKind, TPayload, TMeta>,
  ): () => void;
}

const createRealityPipeId = (): RealityPipeId => {
  const maybeUUID =
    typeof globalThis !== 'undefined' &&
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (globalThis as any).crypto?.randomUUID;
  if (typeof maybeUUID === 'function') return maybeUUID.call((globalThis as any).crypto);
  return `${Math.random().toString(36).slice(2)}-${Date.now().toString(36)}`;
};

const normalizeKinds = <TKind extends string>(
  kind: TKind | readonly TKind[] | undefined,
): ReadonlySet<TKind> | undefined => {
  if (!kind) return undefined;
  if (Array.isArray(kind)) return new Set(kind);
  return new Set([kind as TKind]);
};

export class InMemoryRealityPipe<TKind extends string = string, TPayload = unknown, TMeta = unknown>
  implements RealityPipe<TKind, TPayload, TMeta>
{
  private readonly policy: { kernelConclusiveAllowed: boolean; warnOnly: boolean };

  constructor(opts?: { kernelConclusiveAllowed?: boolean; policyWarnOnly?: boolean }) {
    this.policy = { kernelConclusiveAllowed: !!opts?.kernelConclusiveAllowed, warnOnly: opts?.policyWarnOnly !== false };
  }
  private readonly handlers = new Set<{
    handler: RealityPipeHandler<TKind, TPayload, TMeta>;
    kinds?: ReadonlySet<TKind>;
    predicate?: (envelope: RealityPipeEnvelope<TKind, TPayload, TMeta>) => boolean;
  }>();

  // Store an append-only history to enable reads and replay semantics
  private readonly history: RealityPipeEnvelope<TKind, TPayload, TMeta>[] = [];

  publish(
    input: RealityPipePublishInput<TKind, TPayload, TMeta>,
  ): RealityPipeEnvelope<TKind, TPayload, TMeta> {
    const asAny = input as any;
    const envelope: RealityPipeEnvelope<TKind, TPayload, TMeta> = (() => {
      // Accept fully-formed envelopes (id+ts) OR PrintEnvelope-like objects (id+timestamp)
      // so callers can publish schema-valid prints without losing fields.
      if (asAny && typeof asAny === 'object' && 'id' in asAny && ('ts' in asAny || 'timestamp' in asAny)) {
        const ts =
          typeof asAny.ts === 'number'
            ? asAny.ts
            : typeof asAny.timestamp === 'string'
              ? Date.parse(asAny.timestamp)
              : asAny.timestamp instanceof Date
                ? asAny.timestamp.getTime()
                : Date.now();

        return { ...asAny, ts } as RealityPipeEnvelope<TKind, TPayload, TMeta>;
      }

      return {
        id: createRealityPipeId(),
        ts: Date.now(),
        kind: (input as any).kind,
        payload: (input as any).payload,
        meta: (input as any).meta,
        correlationId: (input as any).correlationId,
        source: (input as any).source,
      };
    })();

    // Policy enforcement: only applies to kernel-sourced conclusive prints
    try {
      const role = (envelope as any).role;
      const epistemicLevel = (envelope as any).epistemicLevel;

      if (role === 'kernel' && epistemicLevel === 'conclusive') {
        assertKernelConclusiveAllowed(envelope as any, { kernelConclusiveAllowed: this.policy.kernelConclusiveAllowed });
      }
    } catch (err) {
      // Policy violation: either warn (default) or throw (strict mode)
      if (this.policy.warnOnly) {
        // Non-blocking: warn and continue
        // eslint-disable-next-line no-console
        console.warn('[reality-pipe] policy warning:', (err as Error).message);
      } else {
        // Throw early to avoid storing disallowed prints
        throw err;
      }
    }

    // append to history before publishing so subscribers can read consistent state
    this.history.push(envelope);

    for (const h of this.handlers) {
      if (h.kinds && !h.kinds.has(envelope.kind)) continue;
      if (h.predicate && !h.predicate(envelope)) continue;
      h.handler(envelope);
    }

    return envelope;
  }

  subscribe(
    handler: RealityPipeHandler<TKind, TPayload, TMeta>,
    opts?: RealityPipeSubscribeOptions<TKind, TPayload, TMeta>,
  ): () => void {
    const entry = {
      handler,
      kinds: normalizeKinds(opts?.kind),
      predicate: opts?.predicate,
    };
    this.handlers.add(entry);
    return () => this.handlers.delete(entry);
  }

  // Read API (non-mutating): returns a ReadView of prints matching filters
  read(filters?: {
    kind?: TKind | readonly TKind[];
    correlationId?: RealityPipeId;
    source?: string;
    timeRange?: { from?: number; to?: number };
    predicate?: (envelope: RealityPipeEnvelope<TKind, TPayload, TMeta>) => boolean;
    limit?: number;
    reverse?: boolean;
    // Aggregation options
    aggregate?: {
      groupBy?: string | ((e: RealityPipeEnvelope<TKind, TPayload, TMeta>) => string);
      reducer?: 'conclusive-latest' | 'latest' | 'count' | 'rollup' | 'custom';
      reducerFn?: (group: RealityPipeEnvelope<TKind, TPayload, TMeta>[]) => any;
      minConfidence?: number;
      minEvidence?: number;
      epistemicFloor?: 'tacit' | 'inferred' | 'proven' | 'conclusive';
      quorum?: number;
      timeWindowMs?: number;
    };
  }): { prints: RealityPipeEnvelope<TKind, TPayload, TMeta>[]; aggregated?: Record<string, any>; cursor?: RealityPipeId } {
    let set = this.history.slice();

    if (filters?.kind) {
      const kinds = normalizeKinds(filters.kind as any) as ReadonlySet<TKind>;
      set = set.filter((e) => kinds.has(e.kind as TKind));
    }

    if (filters?.correlationId) set = set.filter((e) => e.correlationId === filters.correlationId);
    if (filters?.source) set = set.filter((e) => e.source === filters.source);
    if (filters?.timeRange?.from) set = set.filter((e) => e.ts >= (filters.timeRange!.from as number));
    if (filters?.timeRange?.to) set = set.filter((e) => e.ts <= (filters.timeRange!.to as number));
    if (filters?.predicate) set = set.filter(filters.predicate as any);
    if (filters?.reverse) set = set.reverse();
    if (typeof filters?.limit === 'number') set = set.slice(0, filters!.limit);

    const result: { prints: typeof set; aggregated?: Record<string, any>; cursor?: RealityPipeId } = {
      prints: set,
      cursor: set.length ? set[set.length - 1].id : undefined,
    };

    if (filters?.aggregate) {
      const agg = filters.aggregate;
      // default thresholds
      const minConfidence = typeof agg.minConfidence === 'number' ? agg.minConfidence : 0.8;
      const minEvidence = typeof agg.minEvidence === 'number' ? agg.minEvidence : 1;
      const epistemicFloor = agg.epistemicFloor ?? 'inferred';
      const quorum = agg.quorum ?? 0;
      const timeWindowMs = agg.timeWindowMs;

      const groupByFn = typeof agg.groupBy === 'function'
        ? agg.groupBy
        : (e: RealityPipeEnvelope<TKind, TPayload, TMeta>) => {
            // default: meta.subject -> payload.subject -> id
            const asAny = e as any;
            return (asAny.meta?.subject as string) ?? (asAny.payload?.subject as string) ?? asAny.id;
          };

      const groups = new Map<string, RealityPipeEnvelope<TKind, TPayload, TMeta>[]>();

      const now = Date.now();

      for (const e of set) {
        // time window filter
        if (timeWindowMs && now - e.ts > timeWindowMs) continue;

        const asAny = e as any;
        // provenance required
        if (!asAny.provenance || !asAny.provenance.id) continue;
        // confidence filter
        if (typeof asAny.confidence === 'number' && asAny.confidence < minConfidence) continue;
        // evidence filter
        if (typeof asAny.payload === 'object' && Array.isArray(asAny.payload?.proof?.evidenceIds)) {
          if (asAny.payload.proof.evidenceIds.length < minEvidence) continue;
        } else if (minEvidence > 0) {
          continue;
        }
        // epistemic floor
        const levels: Record<string, number> = { tacit: 0, inferred: 1, proven: 2, conclusive: 3 };
        if (typeof asAny.epistemicLevel === 'string') {
          if (levels[asAny.epistemicLevel] < levels[epistemicFloor]) continue;
        } else {
          // if missing, treat as tacit (exclude if floor > tacit)
          if (levels['tacit'] < levels[epistemicFloor]) continue;
        }
        // quorum check requires counting distinct sources in group later; for now, include

        const key = groupByFn(e);
        const bucket = groups.get(key) ?? [];
        bucket.push(e);
        groups.set(key, bucket);
      }

      const aggregated: Record<string, any> = {};

      for (const [k, bucket] of groups) {
        // quorum check
        if (quorum > 0) {
          const sources = new Set<string>((bucket as any[]).map((b) => (b as any).provenance?.sources || []).flat());
          if (sources.size < quorum) {
            aggregated[k] = null;
            continue;
          }
        }

        if (agg.reducer === 'count') {
          aggregated[k] = { count: bucket.length };
          continue;
        }

        if (agg.reducer === 'rollup') {
          // numeric rollup on payload.value if present
          const nums = bucket.map((b) => (b as any).payload?.value).filter((v) => typeof v === 'number');
          aggregated[k] = { sum: nums.reduce((a, b) => a + b, 0), avg: nums.length ? nums.reduce((a, b) => a + b, 0) / nums.length : 0 };
          continue;
        }

        if (agg.reducer === 'custom' && typeof agg.reducerFn === 'function') {
          aggregated[k] = agg.reducerFn(bucket);
          continue;
        }

        // default reducers: conclusive-latest, latest
        const sorted = bucket.slice().sort((a, b) => b.ts - a.ts);
        if (agg.reducer === 'conclusive-latest' || !agg.reducer) {
          const conclusive = sorted.find((s) => ((s as any).epistemicLevel as string) === 'conclusive');
          if (conclusive) {
            aggregated[k] = conclusive;
            continue;
          }
        }

        // latest fallback
        aggregated[k] = sorted[0] ?? null;
      }

      result.aggregated = aggregated;
    }

    return result;
  }
}
