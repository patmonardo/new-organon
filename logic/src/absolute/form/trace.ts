/**
 * Trace vocabulary (structural)
 *
 * Canonical meta types live in sdk/terminology.
 */

import type { EventMeta } from './invariants';
export type { FactStoreInfo, FactStoreOp, EventMeta } from './invariants';

export type TraceEvent<Payload = unknown> = {
  kind: string;
  payload?: Payload;
  meta?: EventMeta;
};
