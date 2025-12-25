import type {
	ObjectivityArtifact,
	ObjectivityAspect,
	ObjectivityEntity,
	ObjectivityProperty,
} from './artifacts';

import { withAppearanceMeta } from './appearance-meta';

export type ThingInWorld = {
	thing: ObjectivityEntity;
	world: {
		kind: 'world';
		id: string;
		label: string;
		meta?: Record<string, unknown>;
	};
	properties: ObjectivityProperty[];
	relationsAsAspects: ObjectivityAspect[];
};

/**
 * Appearance law adapter: treat Entity/Property/Aspect as Thing–Property–Relation-as-Aspect.
 *
 * This is the “contained logic” side: it does not change the kernel program;
 * it annotates/organizes what emerges from the Agent’s objectivity derivation.
 */
export function deriveThingWorldRelation(
	obj: ObjectivityArtifact,
	input?: { worldId?: string; worldLabel?: string },
): ThingInWorld {
	const worldId = input?.worldId ?? 'world-1';
	const worldLabel = input?.worldLabel ?? 'World';

	const thing0 = obj.entities[0];
	const thing: ObjectivityEntity = thing0
		? {
			...thing0,
			meta: withAppearanceMeta(thing0.meta as any, {
				moment: 'entity',
				pass: 'thing',
			}),
		}
		: {
			kind: 'entity',
			id: 'thing-1',
			label: 'Thing',
			meta: withAppearanceMeta(undefined, { moment: 'entity', pass: 'thing' }),
		};

	const properties = (obj.properties ?? []).map((p) => ({
		...p,
		meta: withAppearanceMeta(p.meta as any, { moment: 'property', pass: 'property' }),
	}));

	const relationsAsAspects = (obj.aspects ?? []).map((a) => ({
		...a,
		meta: withAppearanceMeta(a.meta as any, { moment: 'aspect', pass: 'relation' }),
	}));

	return {
		thing,
		world: {
			kind: 'world',
			id: worldId,
			label: worldLabel,
			meta: { source: 'deriveThingWorldRelation' },
		},
		properties,
		relationsAsAspects,
	};
}
