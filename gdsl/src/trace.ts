/**
 * Trace vocabulary (structural)
 *
 * Canonical meta types live in sdk/terminology.
 */

import type { EventMeta } from './sdk/terminology';

export type { FactStoreInfo, FactStoreOp, EventMeta } from './sdk/terminology';

export type TraceEvent<Payload = unknown> = {
  kind: string;
  payload?: Payload;
  meta?: EventMeta;
};
