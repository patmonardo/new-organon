import { z } from 'zod';

import {
	JudgmentArtifactSchema,
	type JudgmentArtifact,
	FoundationMomentSchema,
	JudgmentMomentSchema,
} from './schema/judgment';

/**
 * JudgmentKernelPort
 *
 * Discursive inference boundary for producing a JudgmentArtifact.
 *
 * This is deliberately separate from `KernelPort` (compute substrate) to keep
 * the non-discursive Rust kernel boundary clean.
 */

export const JudgmentKernelInputSchema = z
	.object({
		/** Optional declared judgment moment. */
		moment: JudgmentMomentSchema.optional(),
		/**
		 * Phenomenology payload (IDC + contradictions + foundation/judgment seeds).
		 *
		 * Kept structural and permissive so `@organon/gdsl` does not depend on `@organon/logic`.
		 */
		phenomenology: z
			.object({
				contradictions: z
					.array(
						z
							.object({
								id: z.string().min(1).optional(),
								claim: z.string().min(1),
							})
							.passthrough(),
					)
					.optional(),
				foundation: z
					.object({
						moment: FoundationMomentSchema.optional(),
						thesis: z.string().min(1).optional(),
						resolves: z.array(z.string().min(1)).optional(),
					})
					.passthrough()
					.optional(),
			})
			.passthrough()
			.optional(),
		/** Optional kernel proof/witness payload. */
		proof: z.unknown().optional(),
	})
	.passthrough();
export type JudgmentKernelInput = z.infer<typeof JudgmentKernelInputSchema>;

export const JudgmentKernelResultSchema = z.object({
	ok: z.boolean(),
	artifact: JudgmentArtifactSchema.optional(),
	error: z.object({ message: z.string().min(1) }).optional(),
});
export type JudgmentKernelResult = z.infer<typeof JudgmentKernelResultSchema>;

export interface JudgmentKernelPort {
	readonly name: string;
	judge(input: JudgmentKernelInput): Promise<JudgmentKernelResult>;
}

function tokenizeNominalSequence(text: string): string[] {
	return text
		.split(/\s+/g)
		.map((t) => t.trim())
		.filter(Boolean);
}

/**
 * StubJudgmentKernelPort
 *
 * Deterministic, non-LLM implementation for tests/demos.
 * Produces a JudgmentArtifact primarily from `phenomenology.foundation.thesis`.
 */
export class StubJudgmentKernelPort implements JudgmentKernelPort {
	readonly name: string;

	constructor(name: string = 'stub-judgment-kernel') {
		this.name = name;
	}

	async judge(input: JudgmentKernelInput): Promise<JudgmentKernelResult> {
		try {
			const parsed = JudgmentKernelInputSchema.parse(input);

			const thesis =
				parsed.phenomenology?.foundation?.thesis ??
				'Judgment (stub): no foundation thesis provided';

			const tokens = tokenizeNominalSequence(thesis).map((text) => ({ text }));

			const contradictions = (parsed.phenomenology?.contradictions ?? [])
				.map((c) => c.id ?? c.claim)
				.filter(Boolean);

			const resolves = parsed.phenomenology?.foundation?.resolves ?? [];
			const grounds = [
				...(parsed.moment ? [`moment=${parsed.moment}`] : []),
				...(parsed.phenomenology?.foundation?.moment
					? [`foundation.moment=${parsed.phenomenology.foundation.moment}`]
					: []),
				...(resolves.length ? [`foundation.resolves=${resolves.join(',')}`] : []),
				...(parsed.proof ? ['proof=present'] : []),
			];

			const artifact: JudgmentArtifact = JudgmentArtifactSchema.parse({
				kind: 'judgment',
				moment: parsed.moment,
				foundationMoment: parsed.phenomenology?.foundation?.moment,
				thesis,
				tokens,
				grounds,
				contradictions: contradictions.length ? contradictions : undefined,
				meta: {
					kernel: this.name,
					stub: true,
				},
			});

			return { ok: true, artifact };
		} catch (error) {
			return {
				ok: false,
				error: {
					message: error instanceof Error ? error.message : String(error),
				},
			};
		}
	}
}
