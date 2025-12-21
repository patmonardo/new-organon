import { z } from 'zod';

import {
	GdsDatabaseIdSchema,
	GdsGraphNameSchema,
	GdsUserSchema,
} from './gds.common';

import { GdslMorphSchema } from './morph';

/**
 * GDS Form Eval protocol (Absolute Form / kernel surface)
 *
 * This is the TS-first contract for the Rust kernel FormProcessor.
 *
 * NOTE: This package defines the *shape* of the boundary only.
 * Execution is via a KernelPort adapter (e.g. TS-JSON/NAPI) in higher layers.
 */

export const GdsFormEvalFacadeSchema = z.literal('form_eval');
export type GdsFormEvalFacade = z.infer<typeof GdsFormEvalFacadeSchema>;

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
			 *
			 * Working convention for early moments:
			 * - `morph.patterns[0] = "essence"` (first moment; essentiality / presupposed)
			 * - `morph.patterns[1] = "shine"` (second moment; positedness)
			 * - `morph.patterns[2] = "reflection"` (third moment; reflective consciousness / citta)
			 *
			 * Back-compat:
			 * - `"cit"` is accepted as an alias for `"essence"` by the kernel.
		 *
		 * Convention for the second moment (Shine):
		 * - `shape.validation_rules.moment = "shine"`
		 * - `shape.validation_rules.hegel = "Essence→Shine"`
		 * - `shape.validation_rules.yoga = "YS IV.3 nirmāṇa-cittāni asmitā-mātra"`
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
