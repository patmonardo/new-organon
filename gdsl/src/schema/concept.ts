import { z } from 'zod';

import { JudgmentArtifactSchema } from './judgment';
import { SyllogismArtifactSchema } from './syllogism';
import { ObjectivityArtifactSchema } from './objectivity';

/**
 * Concept Artifact
 *
 * Concept is treated here as an Agent-stabilized unity of:
 * - Morph as Active Ground (minimal kernel bridge)
 * - Judgment (ResultStore)
 * - Syllogism (Truth of Ground)
 * - Objectivity (Entity/Property/Aspect + Essential Relations)
 *
 * Protocol-only: schema-first transport shape.
 */

export const ConceptArtifactSchema = z
	.object({
		kind: z.literal('concept'),
		id: z.string().min(1).optional(),
		morphPatterns: z.array(z.string().min(1)).min(1),
		judgment: JudgmentArtifactSchema.optional(),
		syllogism: SyllogismArtifactSchema.optional(),
		objectivity: ObjectivityArtifactSchema.optional(),
		meta: z.record(z.string(), z.unknown()).optional(),
	})
	.passthrough();
export type ConceptArtifact = z.infer<typeof ConceptArtifactSchema>;
