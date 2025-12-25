import type { ConceptArtifact, JudgmentArtifact, ObjectivityArtifact, SyllogismArtifact } from './artifacts';

import { deriveObjectivityFromSyllogism } from './objectivity';

export type ConceptDerivationInput = {
	morphPatterns: string[];
	judgment?: JudgmentArtifact;
	syllogism?: SyllogismArtifact;
	objectivity?: ObjectivityArtifact;
	meta?: Record<string, unknown>;
};

/**
 * Compose a Concept artifact as the Agent-stabilized unity of Morph + Judgment + Syllogism (+ Objectivity).
 */
export function composeConceptArtifact(input: ConceptDerivationInput): ConceptArtifact {
	const objectivity =
		input.objectivity ??
		(input.syllogism ? deriveObjectivityFromSyllogism(input.syllogism) : undefined);

	return {
		kind: 'concept',
		morphPatterns: input.morphPatterns,
		judgment: input.judgment,
		syllogism: input.syllogism,
		objectivity,
		meta: input.meta,
	};
}
