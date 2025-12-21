import { z } from 'zod';

/**
 * Morph (Active Ground) schema
 *
 * The kernel executes `morph.patterns` (string operator chain).
 *
 * `morph.steps` is an optional, *structured* plan that higher layers (Logic/Task)
 * can use to derive phenomenal artifacts (Judgment/Syllogism) while keeping the
 * kernel non-discursive.
 */

export const GdslFormOpSchema = z.string().min(1);
export type GdslFormOp = z.infer<typeof GdslFormOpSchema>;

export const GdslMorphStepSchema = z.discriminatedUnion('kind', [
	// Non-discursive kernel Form ISA operator (must correspond to a kernel operator name).
	z
		.object({
			kind: z.literal('form'),
			op: GdslFormOpSchema,
			params: z.record(z.string(), z.unknown()).optional(),
		})
		.passthrough(),

	// Discursive kernel inference: Judgment (ResultStore artifact).
	z
		.object({
			kind: z.literal('judge'),
			moment: z.enum(['existence', 'reflection', 'necessity', 'concept']).optional(),
		})
		.passthrough(),

	// Discursive kernel inference: Syllogism (Truth of Ground).
	z
		.object({
			kind: z.literal('syllogize'),
		})
		.passthrough(),
]);
export type GdslMorphStep = z.infer<typeof GdslMorphStepSchema>;

export const GdslMorphSchema = z
	.object({
		/** Executable kernel operator chain. */
		patterns: z.array(GdslFormOpSchema).min(1),
		/**
		 * Optional structured plan.
		 *
		 * Convention (default plan for Reflection launch):
		 * - form: essence → shine → reflection
		 * - judge: (moment inferred or provided)
		 * - syllogize
		 */
		steps: z.array(GdslMorphStepSchema).optional(),
	})
	.passthrough();
export type GdslMorph = z.infer<typeof GdslMorphSchema>;
