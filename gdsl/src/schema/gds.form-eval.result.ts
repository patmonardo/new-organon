import { z } from 'zod';

import { ConceptArtifactSchema } from './concept';
import { SyllogismArtifactSchema } from './syllogism';

/** Result payload for Form evaluation. */
export const GdsFormEvalResultSchema = z
	.object({
		/** Concepts produced by evaluation (optional, may be empty). */
		concepts: z.array(ConceptArtifactSchema).default([]),
		/** Syllogisms produced during evaluation (optional). */
		syllogisms: z.array(SyllogismArtifactSchema).default([]),
		/** Arbitrary artifacts keyed by name for extensibility. */
		artifacts: z.record(z.string(), z.unknown()).default({}),
		/** Execution metadata (timing, traces, provenance). */
		meta: z.record(z.string(), z.unknown()).optional(),
	})
	.passthrough();
export type GdsFormEvalResult = z.infer<typeof GdsFormEvalResultSchema>;
