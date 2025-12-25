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

export type FactTraceEvent = {
  kind: string;
  payload?: unknown;
  meta?: EventMeta;
};

export type TraceGoal = {
  id: string;
  type: string;
  description: string;
};

export type TraceSchema = {
  id: string;
  name?: string;
  description?: string;
};

function nowIso() {
  return new Date().toISOString();
}

function inferProvenance(op?: any): StructuredFact['provenance'] {
  if (op === 'assert') return 'asserted';
  if (op === 'revise') return 'inferred';
  if (op === 'retract') return 'asserted';
  if (op === 'project') return 'inferred';
  if (op === 'index') return 'observed';
  return undefined;
}

function factId(evt: FactTraceEvent, idx: number) {
  const ids = (evt.meta as any)?.factStore?.ids;
  const idPart = Array.isArray(ids) && ids.length ? ids[0] : String(idx);
  return `${evt.kind}:${idPart}`;
}

export function contextFromFactTrace(
  events: FactTraceEvent[],
  opts?: {
    id?: string;
    schema?: TraceSchema;
    goal?: TraceGoal;
    maxFacts?: number;
  },
): ContextDocument {
  const maxFacts = opts?.maxFacts ?? 200;
  const slice = events.slice(0, maxFacts);

  const schemaId = opts?.schema?.id ?? inferSchemaId(slice);

  const facts: StructuredFact[] = slice.map((evt, i) => {
    const fs = (evt.meta as any)?.factStore;
    return {
      id: factId(evt, i),
      label: fs?.kind ?? evt.kind,
      type: evt.kind,
      value: evt.payload ?? null,
      provenance: inferProvenance(fs?.op),
      confidence: undefined,
      relevance: undefined,
    };
  });

  return {
    id: opts?.id ?? `ctx-${schemaId}-${Date.now()}`,
    timestamp: nowIso(),
    facts,
    schema: {
      id: schemaId,
      name: opts?.schema?.name,
      description: opts?.schema?.description,
      fieldCount: facts.length,
      requiredFields: [],
      optionalFields: facts.map((f) => f.id),
    },
    constraints: [],
    goal: opts?.goal,
    dependencies: [],
  };
}

function inferSchemaId(events: FactTraceEvent[]) {
  for (const e of events) {
    const kind = (e.meta as any)?.factStore?.kind;
    if (typeof kind === 'string' && kind.length) return `trace:${kind}`;
  }
  return 'trace';
}
