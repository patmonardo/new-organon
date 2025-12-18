/**
 * Fact Trace Bridge (Model Middle Term)
 *
 * Converts a stream of Logic-like events (kind/payload/meta) into an Agent-facing
 * ContextDocument. This lives in @model/sdsl on purpose:
 * - No dependency on @organon/logic types (accepts structural event-like inputs)
 * - Produces the agent's Generic Context Display Language
 */

import type { ContextDocument, StructuredFact } from './agent-view';
import type { EventMeta } from './terminology';

export type { DialecticalInfo, EventMeta, FactStoreInfo, FactStoreMode } from './terminology';

export type {
  FactTraceEvent,
  TraceGoal,
  TraceSchema,
} from '@organon/gdsl';

import {
  contextFromFactTrace as baseContextFromFactTrace,
  type FactTraceEvent,
  type TraceGoal,
  type TraceSchema,
} from '@organon/gdsl';

export function contextFromFactTrace(
  events: FactTraceEvent[],
  opts?: {
    id?: string;
    schema?: TraceSchema;
    goal?: TraceGoal;
    maxFacts?: number;
  },
): ContextDocument {
  return baseContextFromFactTrace(events, opts) as unknown as ContextDocument;
}
