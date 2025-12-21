import type { GdsFormProgram } from '@organon/gdsl/schema';
import type { JudgmentArtifact, SyllogismKernelInput } from '@organon/gdsl';

/**
 * Morph as Active Ground → Syllogism input seed.
 *
 * This is the minimal bridge from kernel-program Ground (morph.patterns)
 * into a discursive syllogism inference request.
 */
export function seedSyllogismInputFromProgram(input: {
	program: GdsFormProgram;
	judgment?: JudgmentArtifact;
	phenomenology?: unknown;
	proof?: unknown;
}): SyllogismKernelInput {
	const patterns = (input.program as any)?.morph?.patterns as unknown;
	const morphPatterns = Array.isArray(patterns) ? (patterns as string[]) : [];
	if (morphPatterns.length === 0) {
		throw new Error('GdsFormProgram.morph.patterns must be a non-empty string array');
	}

	return {
		morphPatterns,
		judgment: input.judgment
			? {
				thesis: input.judgment.thesis,
				grounds: input.judgment.grounds,
			}
			: undefined,
		phenomenology: input.phenomenology,
		proof: input.proof,
	};
}
