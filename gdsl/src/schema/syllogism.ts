import { z } from 'zod';

/**
 * Syllogism Artifact (Truth of Ground)
 *
 * The syllogism is treated as the discursive artifact that emerges as the
 * truth of Ground (here: Morph as Active Ground).
 *
 * Protocol-only: this is a stable, transportable shape; it does not prescribe
 * the inference engine (LLM/GNN/rules).
 */

export const SyllogismTokenSchema = z
	.object({
		text: z.string().min(1),
		kind: z.string().min(1).optional(),
	})
	.passthrough();
export type SyllogismToken = z.infer<typeof SyllogismTokenSchema>;

export const SyllogismPremiseSchema = z
	.object({
		/** Optional identifier for cross-linking. */
		id: z.string().min(1).optional(),
		/** Premise statement. */
		thesis: z.string().min(1),
		/** Optional grounds for this premise. */
		grounds: z.array(z.string().min(1)).optional(),
	})
	.passthrough();
export type SyllogismPremise = z.infer<typeof SyllogismPremiseSchema>;

export const SyllogismArtifactSchema = z
	.object({
		kind: z.literal('syllogism'),
		id: z.string().min(1).optional(),
		/**
		 * Morph as Active Ground: the operator chain that is taken as the effective ground.
		 *
		 * This is the minimal bridge between kernel "Ground" and discursive syllogism.
		 */
		morphPatterns: z.array(z.string().min(1)).min(1),
		/** Premises (major/minor/etc) in free form. */
		premises: z.array(SyllogismPremiseSchema).default([]),
		/** Conclusion / derived judgment. */
		conclusion: z.string().min(1),
		/** Nominal sequence tokens (optional, loose). */
		tokens: z.array(SyllogismTokenSchema).default([]),
		/** Additional metadata (provenance, model id, confidence, traces). */
		meta: z.record(z.string(), z.unknown()).optional(),
	})
	.passthrough();
export type SyllogismArtifact = z.infer<typeof SyllogismArtifactSchema>;
