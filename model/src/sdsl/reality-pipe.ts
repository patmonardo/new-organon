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
  private readonly handlers = new Set<{
    handler: RealityPipeHandler<TKind, TPayload, TMeta>;
    kinds?: ReadonlySet<TKind>;
    predicate?: (envelope: RealityPipeEnvelope<TKind, TPayload, TMeta>) => boolean;
  }>();

  publish(
    input: RealityPipePublishInput<TKind, TPayload, TMeta>,
  ): RealityPipeEnvelope<TKind, TPayload, TMeta> {
    const envelope: RealityPipeEnvelope<TKind, TPayload, TMeta> =
      'id' in input && 'ts' in input
        ? input
        : {
            id: createRealityPipeId(),
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
}
