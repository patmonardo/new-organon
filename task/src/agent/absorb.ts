import type { StructuredFact } from '../schema/context-document';
import type { TraceEvent } from '../schema/trace';
import type { RootAgentAbsorbRequest, RootAgentAbsorbResult } from './envelope';

function nowIso() {
  return new Date().toISOString();
}

function traceFactId(evt: TraceEvent, idx: number) {
  const ids = evt.meta?.factStore?.ids;
  const idPart = Array.isArray(ids) && ids.length ? ids[0] : String(idx);
  return `${evt.kind}:${idPart}`;
}

function traceEventToFact(evt: TraceEvent, idx: number): StructuredFact {
  const fs = evt.meta?.factStore;

  return {
    id: traceFactId(evt, idx),
    label: fs?.kind ?? evt.kind,
    type: evt.kind,
    value: evt.payload ?? null,
    provenance:
      fs?.op === 'assert'
        ? 'asserted'
        : fs?.op === 'revise'
          ? 'inferred'
          : fs?.op === 'retract'
            ? 'asserted'
            : fs?.op === 'project'
              ? 'inferred'
              : fs?.op === 'index'
                ? 'observed'
                : undefined,
    confidence: undefined,
    relevance: undefined,
  };
}

export function absorbAppend(request: RootAgentAbsorbRequest): RootAgentAbsorbResult {
  const maxFacts = request.maxFacts ?? Infinity;
  const previousFacts = request.previous.facts;
  const appended = request.traceDelta.map((e, i) => traceEventToFact(e, i));

  const nextFacts = [...previousFacts, ...appended].slice(0, maxFacts);

  return {
    next: {
      ...request.previous,
      id: `${request.previous.id}:absorb:${Date.now()}`,
      timestamp: nowIso(),
      facts: nextFacts,
      schema: {
        ...request.previous.schema,
        fieldCount: nextFacts.length,
        optionalFields: nextFacts.map((f) => f.id),
      },
    },
    absorbedCount: appended.length,
  };
}

export function absorb(request: RootAgentAbsorbRequest): RootAgentAbsorbResult {
  const strategy = request.strategy ?? 'append';
  if (strategy === 'append') return absorbAppend(request);

  // "recompute" is intentionally implementation-defined; default to append
  // to keep the loop closed without assuming a full trace store exists.
  return absorbAppend(request);
}
