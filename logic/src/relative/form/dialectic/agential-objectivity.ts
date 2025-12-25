import type { DialecticIR } from '@schema/dialectic';
import type { JudgmentArtifact } from './artifacts';

import type { ThingInWorld } from './appearance-contained';
import { loadDialecticIR } from './registry';

export type AgentialObjectivityRegime = 'mechanism' | 'chemism' | 'teleology';

export type AgentialObjectivity = {
	kind: 'agentialObjectivity';
	regime: AgentialObjectivityRegime;
	/** Dialectic IR id (stable). */
	irId: string;
	/** Registry key to load IR (stable). */
	irKey: string;
	meta?: Record<string, unknown>;
};

const mechanismIRKey = '@relative/concept/object/mechanism/mechanism-ir#mechanismIR';
const chemismIRKey = '@relative/concept/object/chemism/chemism-ir#chemismIR';
const teleologyIRKey = '@relative/concept/object/teleology/teleology-ir#teleologyIR';

export async function loadAgentialObjectivityIR(
	agential: Pick<AgentialObjectivity, 'irKey'>,
): Promise<DialecticIR> {
	return loadDialecticIR(agential.irKey);
}

/**
 * Agent-level selection of Objectivity regime (Mechanism → Chemism → Teleology).
 *
 * Deterministic stub:
 * - Mechanism is treated as the conceptual default when we only have Ground patterns.
 * - Chemism is treated as a "law of appearance" regime: if a Judgment is present or
 *   contradiction/negativity cues are present, select Chemism.
 * - Teleology is selected when purposive cues appear.
 */
export function deriveAgentialObjectivity(
	thingWorld: ThingInWorld,
	input?: { judgment?: JudgmentArtifact },
): AgentialObjectivity {
	const aspectNames = (thingWorld.relationsAsAspects ?? []).map((a) => a.name);
	const aspectText = aspectNames.join(' ').toLowerCase();
	const judgmentText = input?.judgment
		? `${input.judgment.thesis} ${(input.judgment.grounds ?? []).join(' ')} ${(input.judgment.contradictions ?? []).join(' ')}`
		: '';
	const chemismCue =
		Boolean(input?.judgment) ||
		(input?.judgment?.contradictions?.length ?? 0) > 0 ||
		judgmentText.toLowerCase().includes('contradict') ||
		aspectText.includes('affinity') ||
		aspectText.includes('chem') ||
		aspectText.includes('recipro') ||
		aspectText.includes('negative');
	const teleologyCue =
		aspectText.includes('purpose') ||
		aspectText.includes('means') ||
		aspectText.includes('end') ||
		aspectText.includes('goal');

	let regime: AgentialObjectivityRegime = 'mechanism';
	let irId = 'mechanism-ir';
	let irKey = mechanismIRKey;

	if (teleologyCue) {
		regime = 'teleology';
		irId = 'teleology-ir';
		irKey = teleologyIRKey;
	} else if (chemismCue) {
		regime = 'chemism';
		irId = 'chemism-ir';
		irKey = chemismIRKey;
	}

	return {
		kind: 'agentialObjectivity',
		regime,
		irId,
		irKey,
		meta: {
			thingId: thingWorld.thing.id,
			worldId: thingWorld.world.id,
			aspectNames,
			judgment: input?.judgment
				? {
					moment: input.judgment.moment,
					foundationMoment: input.judgment.foundationMoment,
					contradictions: input.judgment.contradictions,
				}
				: undefined,
		},
	};
}
