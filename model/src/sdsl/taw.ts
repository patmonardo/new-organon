/**
 * TAW (Moment of Concept)
 *
 * Canonical internal message vocabulary for Task/Agent/Workflow binding.
 *
 * This is intentionally transport-agnostic and in-process:
 * - built on the RealityPipe envelope
 * - expresses the Concept as event kinds + payloads
 */

import type { RealityPipe, RealityPipeEnvelope, RealityPipeId } from './reality-pipe';
import type { EventMeta } from './terminology';

import { TAW_KINDS, type TawKind, type TawPayload } from '@organon/task';

export { TAW_KINDS };
export type { TawKind, TawPayload };

export type TawEnvelope = RealityPipeEnvelope<TawKind, TawPayload, EventMeta>;

export function isTawKind(kind: string): kind is TawKind {
  return (TAW_KINDS as readonly string[]).includes(kind);
}

export function publishTaw(
  bus: RealityPipe<TawKind, TawPayload, EventMeta>,
  input: {
    kind: TawKind;
    payload: TawPayload;
    meta?: EventMeta;
    correlationId?: RealityPipeId;
    source?: string;
  },
): TawEnvelope {
  return bus.publish(input);
}

export function subscribeTaw(
  bus: RealityPipe<TawKind, TawPayload, EventMeta>,
  handler: (envelope: TawEnvelope) => void,
  opts?: { kind?: TawKind | readonly TawKind[] },
): () => void {
  return bus.subscribe(handler, { kind: opts?.kind });
}
