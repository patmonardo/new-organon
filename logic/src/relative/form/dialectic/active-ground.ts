import type { KernelFormProgram } from '@schema/kernel';
import type { JudgmentArtifact } from './artifacts';

/**
 * Discursive syllogism request input (logic-layer, not kernel-layer).
 *
 * NOTE: This type used to be imported from `@organon/gdsl` (syllogism-kernel),
 * but GDSL no longer contains agent/discursive kernel surfaces.
 */
export type SyllogismKernelInput = {
  morphPatterns: string[];
  judgment?: { thesis: string; grounds?: string[] };
  phenomenology?: unknown;
  proof?: unknown;
} & Record<string, unknown>;

/**
 * Morph as Active Ground → Syllogism input seed.
 *
 * This is the minimal bridge from kernel-program Ground (morph.patterns)
 * into a discursive syllogism inference request.
 */
export function seedSyllogismInputFromProgram(input: {
  program: KernelFormProgram;
  judgment?: JudgmentArtifact;
  phenomenology?: unknown;
  proof?: unknown;
}): SyllogismKernelInput {
  const patterns = (input.program as any)?.morph?.patterns as unknown;
  const morphPatterns = Array.isArray(patterns) ? (patterns as string[]) : [];
  if (morphPatterns.length === 0) {
    throw new Error(
      'GdsFormProgram.morph.patterns must be a non-empty string array',
    );
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
