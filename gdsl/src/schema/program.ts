import { z } from 'zod';

import {
	GdsApplicationFormKindSchema,
	GdsDatabaseIdSchema,
	GdsGraphNameSchema,
	GdsUserSchema,
} from './common';

/**
 * Program (Absolute Form / kernel surface)
 *
 * This is the TS-first contract for the Rust kernel FormProcessor.
 *
 * NOTE: This package defines the *shape* of the boundary only.
 * Execution is via a KernelPort adapter (e.g. TS-JSON/NAPI) in higher layers.
 */

export const GdsFormEvalFacadeSchema = z.literal('form_eval');
export type GdsFormEvalFacade = z.infer<typeof GdsFormEvalFacadeSchema>;

/**
 * Morph (Active Ground) schema
 *
 * The kernel executes `morph.patterns` (string operator chain).
 *
 * `morph.steps` is optional planning metadata that higher layers can use.
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

	// Discursive placeholders (kept structural; higher layers may interpret).
	z
		.object({
			kind: z.literal('judge'),
			moment: z.enum(['existence', 'reflection', 'necessity', 'concept']).optional(),
		})
		.passthrough(),

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
		/** Optional structured plan. */
		steps: z.array(GdslMorphStepSchema).optional(),
	})
	.passthrough();
export type GdslMorph = z.infer<typeof GdslMorphSchema>;

export const GdsFormProgramSchema = z
	.object({
		/**
		 * Optional structural metadata about the program.
		 *
		 * This mirrors the kernel's `Shape` envelope. The kernel may ignore it today,
		 * but it is part of the contract for schema-first expansion.
		 *
		 * Dialectical convention (semantic mapping; not enforced by the boundary):
		 * - `shape`   → Essence
		 * - `context` → Determination of Essence / Reflection
		 * - `morph`   → Ground as **Active Ground** (operator chain)
		 */
		shape: z
			.object({
				required_fields: z.array(z.string()).optional(),
				optional_fields: z.array(z.string()).optional(),
				type_constraints: z.record(z.string(), z.string()).optional(),
				validation_rules: z.record(z.string(), z.string()).optional(),
			})
			.passthrough()
			.optional(),

		/**
		 * Optional execution-context metadata.
		 * Mirrors the kernel's `Context` envelope.
		 */
		context: z
			.object({
				dependencies: z.array(z.string()).optional(),
				execution_order: z.array(z.string()).optional(),
				runtime_strategy: z.string().optional(),
				conditions: z.array(z.string()).optional(),
			})
			.passthrough()
			.optional(),

		// NOTE: kernel executes `morph.patterns`; `morph.steps` is optional planning metadata.
		morph: GdslMorphSchema,
	})
	.passthrough();
export type GdsFormProgram = z.infer<typeof GdsFormProgramSchema>;

const FormEvalBase = z.object({
	kind: GdsApplicationFormKindSchema.optional(),
	facade: GdsFormEvalFacadeSchema,
	user: GdsUserSchema,
	databaseId: GdsDatabaseIdSchema,
});

/**
 * Mirrors the kernel's `FormRequest` at the JSON boundary.
 *
 * - `graphName` corresponds to the base graph loaded from ExecutionContext
 * - `outputGraphName` optionally persists a new graph in the context
 * - `program.morph.patterns` is the operator pipeline
 */
export const GdsFormEvalCallSchema = z.discriminatedUnion('op', [
	FormEvalBase.extend({
		op: z.literal('evaluate'),
		graphName: GdsGraphNameSchema,
		outputGraphName: GdsGraphNameSchema.optional(),
		program: GdsFormProgramSchema,
		artifacts: z.record(z.string(), z.unknown()).optional(),
	}),
]);
export type GdsFormEvalCall = z.infer<typeof GdsFormEvalCallSchema>;

export function isGdsFormEvalCall(input: unknown): input is GdsFormEvalCall {
	return GdsFormEvalCallSchema.safeParse(input).success;
}

/**
 * TS-JSON response `data` for `form_eval.evaluate`.
 *
 * Mirrors: `gds/src/applications/services/tsjson_napi.rs` (handle_form_eval → "evaluate").
 */
export const GdsFormEvalEvaluateDataSchema = z
	.object({
		graphName: GdsGraphNameSchema,
		outputGraphName: GdsGraphNameSchema.nullable().optional(),
		persistedOutputGraph: z.boolean().optional(),
		operator: z.string().min(1),
		execution_time_ms: z.number().nonnegative().optional(),
		nodeCount: z.number().int().nonnegative().optional(),
		relationshipCount: z.number().int().nonnegative().optional(),
		proof: z.unknown(),
	})
	.passthrough();
export type GdsFormEvalEvaluateData = z.infer<typeof GdsFormEvalEvaluateDataSchema>;


