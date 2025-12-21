import { describe, it, expect } from 'vitest';

import {
	ConceptArtifactSchema,
	ObjectivityArtifactSchema,
	SyllogismArtifactSchema,
} from '../src/schema/index';

describe('concept/objectivity schemas', () => {
	it('parses objectivity with entity/property/aspect and essential relations', () => {
		const obj = ObjectivityArtifactSchema.parse({
			kind: 'objectivity',
			entities: [{ kind: 'entity', id: 'e1', label: 'E' }],
			properties: [
				{ kind: 'property', id: 'p1', entityId: 'e1', name: 'color', value: 'red' },
			],
			aspects: [{ kind: 'aspect', id: 'a1', entityId: 'e1', name: 'appearance' }],
			essentialRelations: [
				{
					kind: 'essentialRelation',
					from: { kind: 'entity', id: 'e1' },
					to: { kind: 'property', id: 'p1' },
					relation: 'hasProperty',
				},
			],
		});

		expect(obj.entities[0]?.label).toBe('E');
		expect(obj.essentialRelations[0]?.relation).toBe('hasProperty');
	});

	it('parses a concept wrapping syllogism and optional objectivity', () => {
		const syllogism = SyllogismArtifactSchema.parse({
			kind: 'syllogism',
			morphPatterns: ['essence', 'shine', 'reflection'],
			premises: [{ thesis: 'ground is active' }],
			conclusion: 'objectivity emerges',
		});

		const concept = ConceptArtifactSchema.parse({
			kind: 'concept',
			morphPatterns: syllogism.morphPatterns,
			syllogism,
			objectivity: { kind: 'objectivity', entities: [], properties: [], aspects: [], essentialRelations: [] },
		});

		expect(concept.morphPatterns).toEqual(['essence', 'shine', 'reflection']);
		expect(concept.syllogism?.conclusion).toBe('objectivity emerges');
	});
});
