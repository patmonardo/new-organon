/**
 * Runtime Algorithms (Algorithm-first core)
 *
 * Pure functions that implement the runtime semantics.
 *
 * The OOP runtime class should be a thin control shell around these.
 */

import type { ContextDocument } from './agent-view';
import type { FactTraceEvent, TraceGoal, TraceSchema } from './fact-trace';
import { contextFromFactTrace } from './fact-trace';
import { essenceToAgentOutputs, type AgentOutputMap } from './essence-to-agent';
import type { AgentOutputFormat } from './agent-adapter';

export type RuntimeState = {
  trace: FactTraceEvent[];
};

export function createRuntimeState(): RuntimeState {
  return { trace: [] };
}

export function recordEvent(
  state: RuntimeState,
  event: FactTraceEvent,
  opts?: { maxEvents?: number },
): RuntimeState {
  const maxEvents = opts?.maxEvents ?? 500;
  const next = [...state.trace, event];
  const trimmed = next.length > maxEvents ? next.slice(next.length - maxEvents) : next;
  return { trace: trimmed };
}

export function snapshotTrace(state: RuntimeState): FactTraceEvent[] {
  return [...state.trace];
}

export function toContextFromState(
  state: RuntimeState,
  opts?: { schema?: TraceSchema; goal?: TraceGoal; maxFacts?: number },
): ContextDocument {
  return contextFromFactTrace(state.trace, {
    schema: opts?.schema,
    goal: opts?.goal,
    maxFacts: opts?.maxFacts,
  });
}

export function toAgentOutputsFromState(
  state: RuntimeState,
  opts?: {
    formats?: AgentOutputFormat[];
    functionName?: string;
    operationType?: 'query' | 'mutation';
    schema?: TraceSchema;
    goal?: TraceGoal;
    maxFacts?: number;
  },
): AgentOutputMap {
  return essenceToAgentOutputs(state.trace, {
    formats: opts?.formats,
    functionName: opts?.functionName,
    operationType: opts?.operationType,
    schema: opts?.schema,
    goal: opts?.goal,
    maxFacts: opts?.maxFacts,
  });
}
