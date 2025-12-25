import type {
	EssentialRelation,
	ObjectivityArtifact,
	ObjectivityAspect,
	ObjectivityEntity,
	ObjectivityProperty,
	SyllogismArtifact,
} from './artifacts';

import { withAppearanceMeta } from './appearance-meta';

function stableId(prefix: string, index: number): string {
	return `${prefix}${index}`;
}

function truncateLabel(text: string, maxLen = 72): string {
	const trimmed = text.trim();
	if (trimmed.length <= maxLen) return trimmed;
	return `${trimmed.slice(0, Math.max(1, maxLen - 1))}…`;
}

/**
 * Derive Objectivity (Entity/Property/Aspect + EssentialRelations) from a Syllogism.
 *
 * Agent-layer stub: deterministic and intentionally shallow.
 *
 * Core principle: Objectivity emerges from the syllogistic substrate,
 * whose ground is Morph (morphPatterns).
 */
export function deriveObjectivityFromSyllogism(
	syllogism: SyllogismArtifact,
): ObjectivityArtifact {
	const entityId = syllogism.id ?? stableId('e', 1);

	const entity: ObjectivityEntity = {
		kind: 'entity',
		id: entityId,
		label: truncateLabel(syllogism.conclusion || 'entity'),
		meta: withAppearanceMeta(
			{ source: 'deriveObjectivityFromSyllogism' },
			{ moment: 'entity', pass: 'thing' },
		),
	};

	const properties: ObjectivityProperty[] = syllogism.morphPatterns.map(
		(pattern, idx) => ({
			kind: 'property',
			id: stableId('p', idx + 1),
			entityId,
			name: `morph.${pattern}`,
			value: true,
			grounds: [`pattern:${pattern}`],
			meta: withAppearanceMeta(undefined, { moment: 'property', pass: 'property' }),
		}),
	);

	const aspect: ObjectivityAspect = {
		kind: 'aspect',
		id: stableId('a', 1),
		entityId,
		name: 'ground',
		description: syllogism.morphPatterns.join(' → '),
		grounds: [`morphPatterns:${syllogism.morphPatterns.join(',')}`],
		meta: withAppearanceMeta(undefined, { moment: 'aspect', pass: 'relation' }),
	};

	const essentialRelations: EssentialRelation[] = [
		{
			kind: 'essentialRelation',
			id: stableId('r', 1),
			from: { kind: 'entity', id: entityId },
			to: { kind: 'aspect', id: aspect.id ?? stableId('a', 1) },
			relation: 'hasAspect',
		},
		...properties.map((prop, idx): EssentialRelation => ({
			kind: 'essentialRelation',
			id: stableId('r', idx + 2),
			from: { kind: 'entity', id: entityId },
			to: { kind: 'property', id: prop.id ?? stableId('p', idx + 1) },
			relation: 'hasProperty',
		})),
	];

	return {
		kind: 'objectivity',
		entities: [entity],
		properties,
		aspects: [aspect],
		essentialRelations,
		meta: {
			morphPatterns: syllogism.morphPatterns,
			conclusion: syllogism.conclusion,
		},
	};
}
