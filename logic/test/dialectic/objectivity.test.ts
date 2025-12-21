import { describe, it, expect } from 'vitest';

import {
	deriveObjectivityFromSyllogism,
	composeConceptArtifact,
	deriveThingWorldRelation,
	deriveAgentialObjectivity,
	deriveTeleologyRegulation,
} from '../../src/relative/form/dialectic/index.js';

describe('syllogism → objectivity → concept (agent layer)', () => {
	it('derives entity/property/aspect and essential relations from morph patterns', () => {
		const syllogism = {
			kind: 'syllogism',
			morphPatterns: ['essence', 'shine', 'reflection'],
			premises: [{ thesis: 'ground is active' }],
			conclusion: 'objectivity emerges',
		} as const;

		const obj = deriveObjectivityFromSyllogism(syllogism);
		const thingWorld = deriveThingWorldRelation(obj);
		const agential = deriveAgentialObjectivity(thingWorld);

		expect(obj.entities.length).toBe(1);
		expect(obj.properties.map((p) => p.name)).toEqual([
			'morph.essence',
			'morph.shine',
			'morph.reflection',
		]);
		expect(obj.aspects[0]?.name).toBe('ground');
		expect(obj.essentialRelations.length).toBeGreaterThanOrEqual(4);

		// Appearance law: Entity=Thing, Aspect=Relation-as-Aspect
		expect((obj.entities[0]?.meta as any)?.container).toBe('appearance');
		expect((obj.entities[0]?.meta as any)?.semanticRole).toBe('thing');
		expect((obj.aspects[0]?.meta as any)?.semanticRole).toBe('relation-as-aspect');

		expect(thingWorld.world.kind).toBe('world');
		expect(thingWorld.thing.kind).toBe('entity');
		expect(thingWorld.relationsAsAspects[0]?.name).toBe('ground');

		// Ground-as-relation selects Mechanism in this stub
		expect(agential.regime).toBe('mechanism');
		expect(agential.irId).toBe('mechanism-ir');
	});

	it('composes a concept artifact and auto-derives objectivity when missing', () => {
		const concept = composeConceptArtifact({
			morphPatterns: ['essence', 'shine', 'reflection'],
			syllogism: {
				kind: 'syllogism',
				morphPatterns: ['essence', 'shine', 'reflection'],
				premises: [],
				conclusion: 'objectivity emerges',
			},
		});

		expect(concept.kind).toBe('concept');
		expect(concept.objectivity?.kind).toBe('objectivity');
		expect(concept.objectivity?.properties.length).toBe(3);
	});

	it('selects Chemism when judgment/contradiction cues are present', () => {
		const syllogism = {
			kind: 'syllogism',
			morphPatterns: ['essence', 'shine', 'reflection'],
			premises: [],
			conclusion: 'objectivity emerges',
		} as const;

		const obj = deriveObjectivityFromSyllogism(syllogism);
		const thingWorld = deriveThingWorldRelation(obj);
		const agential = deriveAgentialObjectivity(thingWorld, {
			judgment: {
				kind: 'judgment',
				moment: 'reflection',
				foundationMoment: 'negative',
				thesis: 'appearance contradicts itself',
				contradictions: ['c1'],
			},
		});

		expect(agential.regime).toBe('chemism');
		expect(agential.irId).toBe('chemism-ir');
	});

	it('derives telos as TruthOf(essential relation) regulated by mechanism/chemism', () => {
		const syllogism = {
			kind: 'syllogism',
			morphPatterns: ['essence', 'shine', 'reflection'],
			premises: [],
			conclusion: 'objectivity emerges',
		} as const;

		const obj0 = deriveObjectivityFromSyllogism(syllogism);
		const { objectivity: obj1, regulation } = deriveTeleologyRegulation(obj0, {
			regulators: { mechanismIrId: 'mechanism-ir', chemismIrId: 'chemism-ir' },
		});

		expect(regulation?.teleologyAspect.name).toBe('teleology');
		const truth = regulation?.truthOfRelations[0];
		expect(truth?.relation).toBe('truthOf');
		expect(truth?.from.kind).toBe('aspect');
		expect(truth?.to.kind).toBe('aspect');
		expect((truth?.meta as any)?.regulators?.mechanism).toBe('mechanism-ir');
		expect((truth?.meta as any)?.regulators?.chemism).toBe('chemism-ir');

		// Objectivity is enriched with teleology + reified relation-as-aspect
		expect(obj1.aspects.some((a) => a.name === 'teleology')).toBe(true);
		expect(obj1.essentialRelations.some((r) => r.relation === 'truthOf')).toBe(true);
	});
});
