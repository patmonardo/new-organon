import type {
	EssentialRelation,
	ObjectivityArtifact,
	ObjectivityAspect,
} from './artifacts';

import { withAppearanceMeta } from './appearance-meta';

export type TeleologyRegulation = {
	kind: 'teleologyRegulation';
	teleologyAspect: ObjectivityAspect;
	reifiedRelations: ObjectivityAspect[];
	truthOfRelations: EssentialRelation[];
	meta?: Record<string, unknown>;
};

function stableId(prefix: string, index: number): string {
	return `${prefix}${index}`;
}

function reifyEssentialRelationAsAspect(
	rel: EssentialRelation,
	index: number,
): ObjectivityAspect {
	return {
		kind: 'aspect',
		id: rel.id ? `aspect:${rel.id}` : stableId('rel-aspect-', index + 1),
		name: 'essentialRelation',
		description: `${rel.from.kind}:${rel.from.id} ${rel.relation} ${rel.to.kind}:${rel.to.id}`,
		grounds: rel.grounds,
		meta: withAppearanceMeta(rel.meta as any, { moment: 'aspect', pass: 'relation' }),
	};
}

/**
 * Teleology as TruthOf(EssentialRelation).
 *
 * Agent-layer rule:
 * - EssentialRelations are treated as Aspects (relations-as-aspects) in Appearance.
 * - Teleology "regulates" these relations: it is expressed as a TruthOf relation
 *   whose regulators are Mechanism/Chemism (encoded in meta).
 */
export function deriveTeleologyRegulation(
	obj: ObjectivityArtifact,
	input?: {
		regulators?: { mechanismIrId?: string; chemismIrId?: string };
		targetRelationKinds?: string[];
	},
): { objectivity: ObjectivityArtifact; regulation?: TeleologyRegulation } {
	const targetKinds = new Set(input?.targetRelationKinds ?? ['hasProperty', 'hasAspect']);
	const targets = (obj.essentialRelations ?? []).filter((r) => targetKinds.has(r.relation));
	if (targets.length === 0) return { objectivity: obj };

	const teleologyAspect: ObjectivityAspect = {
		kind: 'aspect',
		id: stableId('teleology-', 1),
		name: 'teleology',
		description: 'TruthOf(EssentialRelation) — regulative unity of mechanism/chemism',
		meta: withAppearanceMeta(
			{ source: 'deriveTeleologyRegulation' },
			{ moment: 'aspect', pass: 'relation' },
		),
	};

	const reified = targets.map((r, i) => reifyEssentialRelationAsAspect(r, i));

	const truthOfRelations: EssentialRelation[] = reified.map((aspect, i) => ({
		kind: 'essentialRelation',
		id: stableId('truthOf-', i + 1),
		from: { kind: 'aspect', id: teleologyAspect.id ?? stableId('teleology-', 1) },
		to: { kind: 'aspect', id: aspect.id ?? stableId('rel-aspect-', i + 1) },
		relation: 'truthOf',
		meta: {
			regulators: {
				mechanism: input?.regulators?.mechanismIrId ?? 'mechanism-ir',
				chemism: input?.regulators?.chemismIrId ?? 'chemism-ir',
			},
		},
	}));

	const next: ObjectivityArtifact = {
		...obj,
		aspects: [...(obj.aspects ?? []), teleologyAspect, ...reified],
		essentialRelations: [...(obj.essentialRelations ?? []), ...truthOfRelations],
		meta: {
			...(obj.meta ?? {}),
			teleology: {
				enabled: true,
				regulators: truthOfRelations[0]?.meta,
			},
		},
	};

	return {
		objectivity: next,
		regulation: {
			kind: 'teleologyRegulation',
			teleologyAspect,
			reifiedRelations: reified,
			truthOfRelations,
			meta: { targets: targets.map((t) => t.id ?? t.relation) },
		},
	};
}
