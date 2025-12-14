import type { FunctionCallOutput } from './agent-adapter';
import type { PromptOutput } from './agent-adapter';
import type { ContextDocument } from './agent-view';
import {
  type TawActEvent,
  type TawEvent,
  type TawIntentEvent,
  type TawPlanEvent,
  type TawResultEvent,
  TawActEventSchema,
  TawIntentEventSchema,
  TawPlanEventSchema,
  TawResultEventSchema,
} from './taw-schema';
import {
  KERNEL_TAW_ACTIONS,
  KernelRunRequestSchema,
  type KernelRunRequest,
  type KernelRunResult,
} from './kernel';

export type FunctionCallToTawActOptions = {
  goalId: string;
  stepId?: string;
  meta?: Record<string, unknown>;
  correlationId?: string;
  source?: string;
};

export type StepsToTawPlanOptions = {
  goalId: string;
  meta?: Record<string, unknown>;
  correlationId?: string;
  source?: string;
  /** Prefix for generated step ids (defaults to "s"). */
  stepIdPrefix?: string;
};

export type ContextToTawIntentOptions = {
  /** If omitted, uses `context.goal`. */
  goal?: { id: string; type: string; description: string };
  constraints?: string[];
  meta?: Record<string, unknown>;
  correlationId?: string;
  source?: string;
};

export type PlanPromptOptions = {
  /** If set, asks for at most this many steps. */
  maxSteps?: number;
  /** Defaults to "numbered" for easier parsing. */
  style?: 'numbered' | 'bulleted';
};

export type PlanStepInput =
  | string
  | {
      id?: string;
      description: string;
    };

/**
 * Bridge: Model AgentAdapter output → TAW concept surface.
 *
 * The Model package produces agent-consumable views (e.g. function calls).
 * TAW consumes these as actions (`taw.act`) in an evented execution stream.
 */
export function functionCallOutputToTawActEvent(
  call: FunctionCallOutput,
  opts: FunctionCallToTawActOptions,
): TawActEvent {
  return TawActEventSchema.parse({
    kind: 'taw.act',
    payload: {
      goalId: opts.goalId,
      stepId: opts.stepId,
      action: call.name,
      input: call.arguments,
    },
    meta: opts.meta,
    correlationId: opts.correlationId,
    source: opts.source,
  });
}

export function contextDocumentToTawIntentEvent(
  context: ContextDocument,
  opts: ContextToTawIntentOptions = {},
): TawIntentEvent {
  const goal = opts.goal ?? context.goal;
  if (!goal) {
    throw new Error('Cannot build taw.intent: no goal provided and context.goal is missing');
  }

  return TawIntentEventSchema.parse({
    kind: 'taw.intent',
    payload: {
      goal,
      constraints: opts.constraints,
    },
    meta: opts.meta,
    correlationId: opts.correlationId,
    source: opts.source,
  });
}

export function stepsToTawPlanEvent(
  steps: readonly PlanStepInput[],
  opts: StepsToTawPlanOptions,
): TawPlanEvent {
  const stepIdPrefix = opts.stepIdPrefix ?? 's';

  const normalizedSteps = steps
    .map((s, i) => {
      if (typeof s === 'string') {
        const description = s.trim();
        return description
          ? { id: `${stepIdPrefix}${i + 1}`, description }
          : undefined;
      }

      const description = s.description.trim();
      if (!description) return undefined;

      return {
        id: (s.id?.trim() ? s.id.trim() : `${stepIdPrefix}${i + 1}`),
        description,
      };
    })
    .filter((s): s is { id: string; description: string } => Boolean(s));

  if (normalizedSteps.length === 0) {
    throw new Error('Cannot build taw.plan: no steps provided');
  }

  return TawPlanEventSchema.parse({
    kind: 'taw.plan',
    payload: {
      goalId: opts.goalId,
      steps: normalizedSteps,
    },
    meta: opts.meta,
    correlationId: opts.correlationId,
    source: opts.source,
  });
}

/**
 * Parses a plain-text plan (bulleted or numbered lines) into a `taw.plan` event.
 *
 * Accepted line shapes:
 * - "- Do the thing"
 * - "* Do the thing"
 * - "1. Do the thing" / "1) Do the thing"
 */
export function planTextToTawPlanEvent(
  planText: string,
  opts: StepsToTawPlanOptions,
): TawPlanEvent {
  const lines = planText.split(/\r?\n/);
  const steps: string[] = [];

  for (const line of lines) {
    const bullet = /^\s*[-*]\s+(.+)\s*$/.exec(line);
    if (bullet?.[1]) {
      steps.push(bullet[1]);
      continue;
    }

    const numbered = /^\s*\d+[.)]\s+(.+)\s*$/.exec(line);
    if (numbered?.[1]) {
      steps.push(numbered[1]);
      continue;
    }
  }

  return stepsToTawPlanEvent(steps, opts);
}

/**
 * Convenience: use a PromptOutput's `content` as plan text.
 *
 * This intentionally does not assume a specific prompt template.
 */
export function promptOutputToTawPlanEvent(
  prompt: PromptOutput,
  opts: StepsToTawPlanOptions,
): TawPlanEvent {
  return planTextToTawPlanEvent(prompt.content, opts);
}

/**
 * Deterministic prompt wrapper that asks an external planner (LLM or otherwise)
 * to emit an easily-parsed list of steps.
 */
export function promptOutputToPlanPromptText(
  prompt: PromptOutput,
  opts: PlanPromptOptions = {},
): string {
  const style = opts.style ?? 'numbered';
  const maxSteps = opts.maxSteps;

  const lines: string[] = [prompt.content.trimEnd(), '', '## Response Format'];

  if (style === 'numbered') {
    lines.push('Return ONLY a numbered list of steps (no prose).');
    lines.push('Example:');
    lines.push('1. First step');
    lines.push('2. Second step');
  } else {
    lines.push('Return ONLY a bulleted list of steps (no prose).');
    lines.push('Example:');
    lines.push('- First step');
    lines.push('- Second step');
  }

  if (typeof maxSteps === 'number') {
    lines.push('');
    lines.push(`Limit to at most ${maxSteps} steps.`);
  }

  return lines.join('\n');
}

export type KernelRunToTawActOptions = {
  goalId: string;
  stepId?: string;
  meta?: Record<string, unknown>;
  correlationId?: string;
  source?: string;
};

export function kernelRunRequestToTawActEvent(
  request: KernelRunRequest,
  opts: KernelRunToTawActOptions,
): TawActEvent {
  const validated = KernelRunRequestSchema.parse(request);

  return TawActEventSchema.parse({
    kind: 'taw.act',
    payload: {
      goalId: opts.goalId,
      stepId: opts.stepId,
      action: KERNEL_TAW_ACTIONS.run,
      input: validated,
    },
    meta: opts.meta,
    correlationId: opts.correlationId,
    source: opts.source,
  });
}

export type KernelResultToTawResultOptions = {
  goalId: string;
  stepId?: string;
  meta?: Record<string, unknown>;
  correlationId?: string;
  source?: string;
};

export function kernelRunResultToTawResultEvent(
  result: KernelRunResult,
  opts: KernelResultToTawResultOptions,
): TawResultEvent {
  return TawResultEventSchema.parse({
    kind: 'taw.result',
    payload: {
      goalId: opts.goalId,
      stepId: opts.stepId,
      ok: result.ok,
      output: result.output,
      error: result.error,
    },
    meta: opts.meta,
    correlationId: opts.correlationId,
    source: opts.source,
  });
}
