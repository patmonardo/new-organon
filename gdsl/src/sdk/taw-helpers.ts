import {
  TawActEventSchema,
  TawIntentEventSchema,
  TawPlanEventSchema,
  type TawActEvent,
  type TawIntentEvent,
  type TawPlanEvent,
} from '../root-agent';

import type { ContextDocument } from './agent-context';

export type ContextToTawIntentOptions = {
  /** If omitted, uses `context.goal`. */
  goal?: { id: string; type: string; description: string };
  constraints?: string[];
  meta?: Record<string, unknown>;
  correlationId?: string;
  source?: string;
};

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

export type StepsToTawPlanOptions = {
  goalId: string;
  meta?: Record<string, unknown>;
  correlationId?: string;
  source?: string;
  /** Prefix for generated step ids (defaults to "s"). */
  stepIdPrefix?: string;
};

export type PlanStepInput =
  | string
  | {
      id?: string;
      description: string;
    };

export function stepsToTawPlanEvent(steps: readonly PlanStepInput[], opts: StepsToTawPlanOptions): TawPlanEvent {
  const stepIdPrefix = opts.stepIdPrefix ?? 's';

  const normalizedSteps = steps
    .map((s, i) => {
      if (typeof s === 'string') {
        const description = s.trim();
        return description ? { id: `${stepIdPrefix}${i + 1}`, description } : undefined;
      }

      const description = s.description.trim();
      if (!description) return undefined;

      return {
        id: s.id?.trim() ? s.id.trim() : `${stepIdPrefix}${i + 1}`,
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
export function planTextToTawPlanEvent(planText: string, opts: StepsToTawPlanOptions): TawPlanEvent {
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

export type PlanPromptOptions = {
  /** If set, asks for at most this many steps. */
  maxSteps?: number;
  /** Defaults to "numbered" for easier parsing. */
  style?: 'numbered' | 'bulleted';
};

/**
 * Deterministic prompt wrapper that asks an external planner to emit an easily-parsed list of steps.
 *
 * This intentionally does not assume a specific prompt template.
 */
export function promptTextToPlanPromptText(promptText: string, opts: PlanPromptOptions = {}): string {
  const style = opts.style ?? 'numbered';
  const maxSteps = opts.maxSteps;

  const lines: string[] = [promptText.trimEnd(), '', '## Response Format'];

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

export type ActionToTawActOptions = {
  goalId: string;
  stepId?: string;
  meta?: Record<string, unknown>;
  correlationId?: string;
  source?: string;
};

/**
 * Minimal helper for non-kernel actions.
 * External wrappers (MCP/Genkit/etc.) can use this for tool calls.
 */
export function actionToTawActEvent(action: string, input: unknown, opts: ActionToTawActOptions): TawActEvent {
  return TawActEventSchema.parse({
    kind: 'taw.act',
    payload: {
      goalId: opts.goalId,
      stepId: opts.stepId,
      action,
      input,
    },
    meta: opts.meta,
    correlationId: opts.correlationId,
    source: opts.source,
  });
}
