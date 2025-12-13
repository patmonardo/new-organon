import { AgentAdapter, type FunctionCallOutput, type PromptOutput } from './agent-adapter';
import { contextFromFactTrace, type FactTraceEvent, type TraceGoal, type TraceSchema } from './fact-trace';
import type { ContextDocument } from './agent-view';
import {
  contextDocumentToTawIntentEvent,
  functionCallOutputToTawActEvent,
  kernelRunRequestToTawActEvent,
  planTextToTawPlanEvent,
  promptOutputToPlanPromptText,
  type ContextToTawIntentOptions,
  type FunctionCallToTawActOptions,
  type KernelRunToTawActOptions,
  type PlanPromptOptions,
  type StepsToTawPlanOptions,
} from './agent-to-taw';
import type { TawEvent } from './taw-schema';
import type { KernelRunRequest } from './kernel';

export type DemoLoopSeed = {
  context: ContextDocument;
  prompt: PromptOutput;
  intentEvent: TawEvent;
  planPromptText: string;
};

/**
 * Demo loop (Model → TAW) seed:
 * 1) FactTrace → ContextDocument
 * 2) ContextDocument → `taw.intent`
 * 3) ContextDocument → PromptOutput → deterministic planning prompt text
 */
export function seedDemoLoopFromTrace(
  trace: FactTraceEvent[],
  opts: {
    schema?: TraceSchema;
    goal: TraceGoal;
    maxFacts?: number;
    intent?: Omit<ContextToTawIntentOptions, 'goal'>;
    planPrompt?: PlanPromptOptions;
    adapter?: AgentAdapter;
  },
): DemoLoopSeed {
  const context = contextFromFactTrace(trace, {
    schema: opts.schema,
    goal: opts.goal,
    maxFacts: opts.maxFacts,
  });

  const adapter = opts.adapter ?? new AgentAdapter();
  const prompt = adapter.toPrompt(context);

  const intentEvent = contextDocumentToTawIntentEvent(context, {
    goal: opts.goal,
    ...opts.intent,
  });

  const planPromptText = promptOutputToPlanPromptText(prompt, opts.planPrompt);

  return { context, prompt, intentEvent, planPromptText };
}

/**
 * Demo loop step: planner text → `taw.plan`.
 */
export function plannerTextToTawPlan(
  planText: string,
  opts: StepsToTawPlanOptions,
): TawEvent {
  return planTextToTawPlanEvent(planText, opts);
}

export type DemoActChoice =
  | { type: 'function'; call: FunctionCallOutput }
  | { type: 'kernel.run'; request: KernelRunRequest };

/**
 * Demo loop step: chosen action (function call / kernel request) → `taw.act`.
 */
export function choiceToTawAct(
  choice: DemoActChoice,
  opts:
    | (Omit<FunctionCallToTawActOptions, 'goalId'> & { goalId: string })
    | (Omit<KernelRunToTawActOptions, 'goalId'> & { goalId: string }),
): TawEvent {
  if (choice.type === 'function') {
    return functionCallOutputToTawActEvent(choice.call, opts as FunctionCallToTawActOptions);
  }

  return kernelRunRequestToTawActEvent(choice.request, opts as KernelRunToTawActOptions);
}
