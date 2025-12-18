import { contextFromFactTrace, type FactTraceEvent, type TraceGoal, type TraceSchema } from './fact-trace';
import type { ContextDocument } from './agent-context';
import {
  contextDocumentToTawIntentEvent,
  planTextToTawPlanEvent,
  promptTextToPlanPromptText,
  type ContextToTawIntentOptions,
  type PlanPromptOptions,
  type StepsToTawPlanOptions,
} from './taw-helpers';
import type { TawIntentEvent, TawPlanEvent } from '../root-agent';

export type DemoLoopSeed = {
  context: ContextDocument;
  intentEvent: TawIntentEvent;
  /** Optional: deterministic planning prompt text for an external planner. */
  planPromptText?: string;
};

/**
 * Demo loop (GDSL SDK) seed:
 * 1) FactTrace → ContextDocument
 * 2) ContextDocument → `taw.intent`
 * 3) Optional: promptText → deterministic planning prompt text
 */
export function seedDemoLoopFromTrace(
  trace: FactTraceEvent[],
  opts: {
    schema?: TraceSchema;
    goal: TraceGoal;
    maxFacts?: number;
    intent?: Omit<ContextToTawIntentOptions, 'goal'>;
    planPrompt?: PlanPromptOptions;
    /** If provided, wraps it into a deterministic planning prompt. */
    promptText?: string;
  },
): DemoLoopSeed {
  const context = contextFromFactTrace(trace, {
    schema: opts.schema,
    goal: opts.goal,
    maxFacts: opts.maxFacts,
  });

  const intentEvent = contextDocumentToTawIntentEvent(context, {
    goal: opts.goal,
    ...opts.intent,
  });

  const planPromptText =
    typeof opts.promptText === 'string'
      ? promptTextToPlanPromptText(opts.promptText, opts.planPrompt)
      : undefined;

  return { context, intentEvent, planPromptText };
}

/** Demo loop step: planner text → `taw.plan`. */
export function plannerTextToTawPlan(planText: string, opts: StepsToTawPlanOptions): TawPlanEvent {
  return planTextToTawPlanEvent(planText, opts);
}
