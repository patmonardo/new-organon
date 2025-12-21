import { z } from 'zod';

import {
	SyllogismArtifactSchema,
	type SyllogismArtifact,
} from './schema/syllogism';

/**
 * SyllogismKernelPort
 *
 * Discursive inference boundary for producing a SyllogismArtifact.
 *
 * This is intentionally separate from the non-discursive compute kernel.
 */

export const SyllogismKernelInputSchema = z
	.object({
		/** Active Ground operator chain. */
		morphPatterns: z.array(z.string().min(1)).min(1),
		/** Optional judgment artifact (if already derived). */
		judgment: z
			.object({
				thesis: z.string().min(1),
				grounds: z.array(z.string().min(1)).optional(),
			})
			.passthrough()
			.optional(),
		/** Optional phenomenology payload (structural, permissive). */
		phenomenology: z.unknown().optional(),
		/** Optional kernel proof/witness payload. */
		proof: z.unknown().optional(),
	})
	.passthrough();
export type SyllogismKernelInput = z.infer<typeof SyllogismKernelInputSchema>;

export const SyllogismKernelResultSchema = z.object({
	ok: z.boolean(),
	artifact: SyllogismArtifactSchema.optional(),
	error: z.object({ message: z.string().min(1) }).optional(),
});
export type SyllogismKernelResult = z.infer<typeof SyllogismKernelResultSchema>;

export interface SyllogismKernelPort {
	readonly name: string;
	sylogize(input: SyllogismKernelInput): Promise<SyllogismKernelResult>;
}

function tokenize(text: string): string[] {
	return text
		.split(/\s+/g)
		.map((t) => t.trim())
		.filter(Boolean);
}

/**
 * StubSyllogismKernelPort
 *
 * Deterministic implementation for tests/demos.
 *
 * Produces a syllogism that:
 * - takes morphPatterns as Active Ground
 * - uses `judgment.thesis` when present, else synthesizes a minimal conclusion
 */
export class StubSyllogismKernelPort implements SyllogismKernelPort {
	readonly name: string;

	constructor(name: string = 'stub-syllogism-kernel') {
		this.name = name;
	}

	async sylogize(input: SyllogismKernelInput): Promise<SyllogismKernelResult> {
		try {
			const parsed = SyllogismKernelInputSchema.parse(input);
			const chain = parsed.morphPatterns.join(' -> ');

			const conclusion =
				parsed.judgment?.thesis ??
				`Syllogism (stub): truth of ground for morph=${chain}`;

			const premises = [
				{
					id: 'p1',
					thesis: `Ground (Active): ${chain}`,
					grounds: parsed.judgment?.grounds ?? [],
				},
				...(parsed.proof ? [{ id: 'p2', thesis: 'Kernel proof is present' }] : []),
			];

			const tokens = tokenize(conclusion).map((text) => ({ text }));

			const artifact: SyllogismArtifact = SyllogismArtifactSchema.parse({
				kind: 'syllogism',
				morphPatterns: parsed.morphPatterns,
				premises,
				conclusion,
				tokens,
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
