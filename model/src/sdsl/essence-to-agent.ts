/**
 * Essence → View → Agent (Model Runtime)
 *
 * The Model package is the middle term: it converts FactStore/Logic traces (Essence)
 * into agent-consumable views (ContextDocument) and then into multiple adapter outputs.
 *
 * This module is intentionally dependency-light:
 * - Takes structural event traces (kind/payload/meta)
 * - Produces ContextDocument + agent outputs via AgentAdapter
 */

import { AgentAdapter, type AgentOutputFormat, type FunctionCallOutput, type GraphQLOutput, type JSONLDOutput, type PromptOutput } from './agent-adapter';
import type { ContextDocument } from './agent-view';
import { contextFromFactTrace, type FactTraceEvent, type TraceGoal, type TraceSchema } from './fact-trace';

export type AgentOutputMap = {
  context?: ContextDocument;
  prompt?: PromptOutput;
  function?: FunctionCallOutput;
  graphql?: GraphQLOutput;
  jsonld?: JSONLDOutput;
  rdf?: unknown;
};

export function essenceToAgentContext(
  events: FactTraceEvent[],
  opts?: { id?: string; schema?: TraceSchema; goal?: TraceGoal; maxFacts?: number },
): ContextDocument {
  return contextFromFactTrace(events, opts);
}

export function essenceToAgentOutputs(
  events: FactTraceEvent[],
  opts?: {
    formats?: AgentOutputFormat[];
    functionName?: string;
    operationType?: 'query' | 'mutation';
    schema?: TraceSchema;
    goal?: TraceGoal;
    maxFacts?: number;
  },
): AgentOutputMap {
  const formats = opts?.formats ?? ['context'];
  const adapter = new AgentAdapter();

  const context = contextFromFactTrace(events, {
    schema: opts?.schema,
    goal: opts?.goal,
    maxFacts: opts?.maxFacts,
  });

  const out: AgentOutputMap = {};

  for (const f of formats) {
    if (f === 'context') out.context = context;
    if (f === 'prompt') out.prompt = adapter.toPrompt(context);
    if (f === 'function') out.function = adapter.toFunctionCall(context, opts?.functionName ?? 'agent.act');
    if (f === 'graphql') out.graphql = adapter.toGraphQL(context, { operationType: opts?.operationType ?? 'query' });
    if (f === 'jsonld') out.jsonld = adapter.toJSONLD(context);
    if (f === 'rdf') out.rdf = adapter.toRDF(context);
  }

  return out;
}
