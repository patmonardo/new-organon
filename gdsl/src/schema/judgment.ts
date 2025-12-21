import { z } from 'zod';

/**
 * Judgment Artifact (ResultStore)
 *
 * A Judgment is treated as a kernel-side artifact at the boundary:
 * it is a transportable, schema-first representation of a discursive judgment
 * (a nominal sequence / Nāma) over a Context.
 *
 * NOTE:
 * - This is a protocol layer type (GDSL).
 * - It does not prescribe *how* the judgment is produced (LLM, GNN, rules, etc.).
 */

export const JudgmentMomentSchema = z.enum([
	'existence',
	'reflection',
	'necessity',
	'concept',
]);
export type JudgmentMoment = z.infer<typeof JudgmentMomentSchema>;

export const FoundationMomentSchema = z.enum(['positive', 'negative', 'infinite']);
export type FoundationMoment = z.infer<typeof FoundationMomentSchema>;

export const JudgmentTokenSchema = z
	.object({
		text: z.string().min(1),
		kind: z.string().min(1).optional(),
	})
	.passthrough();
export type JudgmentToken = z.infer<typeof JudgmentTokenSchema>;

export const JudgmentArtifactSchema = z
	.object({
		kind: z.literal('judgment'),
		/** Optional identifier for cross-linking in traces/graphs. */
		id: z.string().min(1).optional(),
		/** Which Judgment family this belongs to (existence/reflection/necessity/concept). */
		moment: JudgmentMomentSchema.optional(),
		/** Optional foundation moment (positive/negative/infinite). */
		foundationMoment: FoundationMomentSchema.optional(),
		/** The asserted judgment (discursive thesis). */
		thesis: z.string().min(1),
		/** Nominal sequence (tokenization is intentionally loose). */
		tokens: z.array(JudgmentTokenSchema).default([]),
		/** Grounds/reasons (strings, links, ids). */
		grounds: z.array(z.string().min(1)).optional(),
		/** Contradiction ids (or labels) addressed by this judgment. */
		contradictions: z.array(z.string().min(1)).optional(),
		/** Additional payload (e.g. provenance, model id, confidence, traces). */
		meta: z.record(z.string(), z.unknown()).optional(),
	})
	.passthrough();
export type JudgmentArtifact = z.infer<typeof JudgmentArtifactSchema>;
