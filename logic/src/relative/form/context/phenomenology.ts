import { z } from 'zod';

import type { Context } from '@schema';

import type { FormContext } from './context-form';

/**
 * Phenomenology facet namespace.
 *
 * This is a discursive/relative (TS) layer construct; the kernel does not interpret it.
 */
export const CONTEXT_PHENOMENOLOGY_FACET_NS = 'phenomenology' as const;

/**
 * Foundation moment (Positive–Negative–Infinite).
 *
 * Working reading:
 * - `infinite` = power to present a contradiction-free foundation (resolution horizon).
 */
export const FoundationMomentSchema = z.enum(['positive', 'negative', 'infinite']);
export type FoundationMoment = z.infer<typeof FoundationMomentSchema>;

export const ContextContradictionSchema = z
  .object({
    /** Stable-ish local id (optional). */
    id: z.string().min(1).optional(),
    /** Minimal statement of the contradiction. */
    claim: z.string().min(1),
    /** Optional evidence/witness payload (e.g. traces, diffs, failing tests). */
    evidence: z.unknown().optional(),
    /** ISO timestamp (optional). */
    at: z.string().datetime().optional(),
  })
  .passthrough();
export type ContextContradiction = z.infer<typeof ContextContradictionSchema>;

/**
 * Foundation as Resolution.
 *
 * This is the (relative) “contradiction-free state” that can ground judgment.
 */
export const ContextFoundationSchema = z
  .object({
    moment: FoundationMomentSchema,
    /** The resolution statement that claims a contradiction-free basis. */
    thesis: z.string().min(1),
    /** Contradiction ids (or labels) resolved by this foundation. */
    resolves: z.array(z.string().min(1)).optional(),
    /** Explicit flag for consumers; defaults true when present. */
    contradictionFree: z.boolean().optional().default(true),
  })
  .passthrough();
export type ContextFoundation = z.infer<typeof ContextFoundationSchema>;

/**
 * Judgment (Truth of Determination of Reflection).
 *
 * This is intentionally minimal: consumers may treat `thesis` as the asserted judgment,
 * optionally annotated with grounds.
 */
export const ContextJudgmentSchema = z
  .object({
    thesis: z.string().min(1),
    grounds: z.array(z.string().min(1)).optional(),
    truth: z.boolean().optional(),
  })
  .passthrough();
export type ContextJudgment = z.infer<typeof ContextJudgmentSchema>;

export const ContextPhenomenologySchema = z
  .object({
    /** Identity commitments: what is held fixed in the scope. */
    identity: z.record(z.string(), z.unknown()).optional(),
    /** Difference: revisions/variations relative to prior identity. */
    difference: z.record(z.string(), z.unknown()).optional(),
    /** Contradictions surfaced in the scope. */
    contradictions: z.array(ContextContradictionSchema).optional(),
    /** Foundation = Resolution (positive/negative/infinite). */
    foundation: ContextFoundationSchema.optional(),
    /** Judgment emerging from foundation. */
    judgment: ContextJudgmentSchema.optional(),
  })
  .passthrough();
export type ContextPhenomenology = z.infer<typeof ContextPhenomenologySchema>;

function readFacets(ctx: Context): Record<string, unknown> {
  return (ctx.shape as any)?.facets && typeof (ctx.shape as any).facets === 'object'
    ? ((ctx.shape as any).facets as Record<string, unknown>)
    : {};
}

export function readContextPhenomenology(ctx: Context): ContextPhenomenology | undefined {
  const facets = readFacets(ctx);
  const raw = facets[CONTEXT_PHENOMENOLOGY_FACET_NS];
  const parsed = ContextPhenomenologySchema.safeParse(raw);
  return parsed.success ? parsed.data : undefined;
}

/** Merge a phenomenology patch into the context facet (shallow merge at the facet level). */
export function mergeContextPhenomenology(
  ctx: FormContext,
  patch: Partial<ContextPhenomenology>,
): FormContext {
  return ctx.mergeFacet(CONTEXT_PHENOMENOLOGY_FACET_NS, patch as Record<string, unknown>);
}

export function recordContextContradiction(
  ctx: FormContext,
  contradiction: ContextContradiction,
): FormContext {
  const doc = ctx.toSchema();
  const current = readContextPhenomenology(doc);
  const next = [...(current?.contradictions ?? []), ContextContradictionSchema.parse(contradiction)];
  return mergeContextPhenomenology(ctx, { contradictions: next });
}

export function setContextFoundation(ctx: FormContext, foundation: ContextFoundation): FormContext {
  return mergeContextPhenomenology(ctx, { foundation: ContextFoundationSchema.parse(foundation) });
}

/**
 * Minimal derivation: judgment emerges from foundation.
 *
 * This does not attempt to "prove" anything; it simply establishes the dependency direction
 * for consumers (e.g. LLM prompting, agent planning).
 */
export function deriveJudgmentFromFoundation(foundation: ContextFoundation): ContextJudgment {
  const f = ContextFoundationSchema.parse(foundation);
  const grounds = [
    `foundation.moment=${f.moment}`,
    ...(f.resolves?.length ? [`foundation.resolves=${f.resolves.join(',')}`] : []),
  ];
  return ContextJudgmentSchema.parse({ thesis: f.thesis, grounds });
}
