/**
 * FCI Bus (Model Middleware)
 *
 * Internal, in-process bus for the Model SDSL runtime.
 *
 * Design intent:
 * - CPU-internal “bus” semantics (FCI): in-memory, synchronous, typed.
 * - No external transport assumptions (no HTTP, no Nest, no queues).
 * - Minimal envelope to support correlation + provenance.
 */

export type FciId = string;

export type FciEnvelope<TKind extends string = string, TPayload = unknown, TMeta = unknown> = {
  id: FciId;
  ts: number;
  kind: TKind;
  payload: TPayload;
  meta?: TMeta;
  correlationId?: FciId;
  source?: string;
};

export type FciPublishInput<TKind extends string = string, TPayload = unknown, TMeta = unknown> =
  | FciEnvelope<TKind, TPayload, TMeta>
  | {
      kind: TKind;
      payload: TPayload;
      meta?: TMeta;
      correlationId?: FciId;
      source?: string;
    };

export type FciHandler<TKind extends string = string, TPayload = unknown, TMeta = unknown> = (
  envelope: FciEnvelope<TKind, TPayload, TMeta>,
) => void;

export type FciSubscribeOptions<TKind extends string = string, TPayload = unknown, TMeta = unknown> = {
  kind?: TKind | readonly TKind[];
  predicate?: (envelope: FciEnvelope<TKind, TPayload, TMeta>) => boolean;
};

export interface FciBus<TKind extends string = string, TPayload = unknown, TMeta = unknown> {
  publish(input: FciPublishInput<TKind, TPayload, TMeta>): FciEnvelope<TKind, TPayload, TMeta>;
  subscribe(
    handler: FciHandler<TKind, TPayload, TMeta>,
    opts?: FciSubscribeOptions<TKind, TPayload, TMeta>,
  ): () => void;
}

const createFciId = (): FciId => {
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

export class InMemoryFciBus<TKind extends string = string, TPayload = unknown, TMeta = unknown>
  implements FciBus<TKind, TPayload, TMeta>
{
  private readonly handlers = new Set<{
    handler: FciHandler<TKind, TPayload, TMeta>;
    kinds?: ReadonlySet<TKind>;
    predicate?: (envelope: FciEnvelope<TKind, TPayload, TMeta>) => boolean;
  }>();

  publish(input: FciPublishInput<TKind, TPayload, TMeta>): FciEnvelope<TKind, TPayload, TMeta> {
    const envelope: FciEnvelope<TKind, TPayload, TMeta> =
      'id' in input && 'ts' in input
        ? input
        : {
            id: createFciId(),
            ts: Date.now(),
            kind: input.kind,
            payload: input.payload,
            meta: input.meta,
            correlationId: input.correlationId,
            source: input.source,
          };

    for (const h of this.handlers) {
      if (h.kinds && !h.kinds.has(envelope.kind)) continue;
      if (h.predicate && !h.predicate(envelope)) continue;
      h.handler(envelope);
    }

    return envelope;
  }

  subscribe(
    handler: FciHandler<TKind, TPayload, TMeta>,
    opts?: FciSubscribeOptions<TKind, TPayload, TMeta>,
  ): () => void {
    const entry = {
      handler,
      kinds: normalizeKinds(opts?.kind),
      predicate: opts?.predicate,
    };
    this.handlers.add(entry);
    return () => this.handlers.delete(entry);
  }
}
