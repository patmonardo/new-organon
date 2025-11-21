// Qualitative & Quantitative utilities for Essence
// - Qualitative: Whole/Parts truth for Grounds, subjectivity marker suggestion
// - Quantitative: counts/degree metrics over Relations/Conditions

export type QualQuantPolicy = {
  wholeThreshold?: number; // min parts to consider a ground a Whole
  particularityThreshold?: number; // min particulars under an Absolute to count as Whole→Mechanism
};

type LR = any; // LooseRelation


export function isWholeGround(ground: LR, policy: QualQuantPolicy = {}): boolean {
  const threshold = policy.wholeThreshold ?? 3;
  const parts: string[] = (ground?.contributingConditionIds ?? []) as string[];
  return Array.isArray(parts) && parts.length >= threshold;
}

export function computeGroundCardinality(
  ground: LR,
  allRelations: LR[],
) {
  const absId = ground?.id;
  const parts = allRelations.filter((r) => (r as any).particularityOf === absId);
  const contributing = (ground?.contributingConditionIds ?? []) as string[];
  return {
    parts: parts.length,
    conditions: contributing.length,
  };
}

export function computeRelationDegrees(relations: LR[]) {
  const out: Record<string, number> = {};
  const inn: Record<string, number> = {};
  for (const r of relations) {
    const s = (r as any).sourceId;
    const t = (r as any).targetId;
    if (s) out[s] = (out[s] ?? 0) + 1;
    if (t) inn[t] = (inn[t] ?? 0) + 1;
  }
  return { out, in: inn };
}

export function suggestModalityForGround(
  ground: LR,
  policy: QualQuantPolicy = {},
) {
  const whole = isWholeGround(ground, policy);
  const contributing = (ground?.contributingConditionIds ?? []) as string[];
  const confidence = Math.min(1, 0.5 + contributing.length / 10);
  return whole
    ? { kind: 'actual', confidence }
    : { kind: 'possible', confidence: Math.min(confidence, 0.6) };
}

// --- Truth of Relation (Hegelian mapping) ----------------------------------

export type TruthOfRelation = 'Mechanism' | 'Chemism' | 'Teleology';

// Classify the truth of a Ground (Absolute container) by Whole↔Parts
export function classifyTruthOfGround(ground: LR, allRelations: LR[], policy: QualQuantPolicy = {}): TruthOfRelation {
  // If the ground aggregates enough parts/conditions, we treat its truth as Mechanism
  if (isWholeGround(ground, policy)) return 'Mechanism';

  // If it binds expressive properties via its particulars, lean Chemism
  const absId = ground?.id;
  const parts = allRelations.filter((r) => (r as any).particularityOf === absId);
  const hasExpressive = parts.some((p) => !!(p as any)?.provenance?.viaTriggerPropertyId || !!(p as any)?.provenance?.viaRelation);
  if (hasExpressive) return 'Chemism';

  // Otherwise interpret as Teleology: an inner directive seeking its outer
  return 'Teleology';
}

// Classify the truth of an Essential relation by inspecting its Absolute and provenance
export function classifyTruthOfRelation(
  relation: LR,
  allRelations: LR[],
  policy: QualQuantPolicy = {},
  opts?: { properties?: Array<any> },
): TruthOfRelation {
  // Locate its Absolute container and sibling particulars
  const absId = (relation as any)?.particularityOf as string | undefined;
  const absolute = allRelations.find((r) => (r as any).id === absId);
  const particulars = allRelations.filter((r) => (r as any).particularityOf === absId);

  // Chemism has priority when there is clear Expression caused by this relation
  const props = opts?.properties ?? [];
  const hasExpression = props.some((p) => (p as any)?.provenance?.viaRelation === (relation as any).id);
  if (hasExpression) return 'Chemism';

  // Mechanism: many particulars under one Absolute (Whole→Parts)
  const pThresh = policy.particularityThreshold ?? 2;
  if (absolute && particulars.length >= pThresh) return 'Mechanism';

  // Teleology: inward intuition striving to outer — inner/outer markers in provenance
  const inner = (relation as any)?.provenance?.metaphysics?.intuition === 'inner';
  const outer = (absolute as any)?.provenance?.metaphysics?.intuition === 'intellectual' || (relation as any)?.directed === true;
  if (inner && outer) return 'Teleology';

  // Fallback: if no strong signal, prefer Mechanism when absolute exists, else Chemism
  if (absolute) return 'Mechanism';
  return 'Chemism';
}

// Compute the "transcendental location" of a transaction (relation):
// returns its immediate locus (transaction) and its truth-location (Absolute + classification)
export function locateTranscendental(
  relation: LR,
  allRelations: LR[],
  policy: QualQuantPolicy = {},
  opts?: { properties?: Array<any> },
) {
  const absId = (relation as any)?.particularityOf as string | undefined;
  const absolute = allRelations.find((r) => (r as any).id === absId);
  const truth = classifyTruthOfRelation(relation, allRelations, policy, opts);
  return {
    locus: {
      id: (relation as any).id,
      type: (relation as any).type,
      sourceId: (relation as any).sourceId,
      targetId: (relation as any).targetId,
    },
    transcendental: absolute
      ? { id: (absolute as any).id, kind: (absolute as any).kind, type: (absolute as any).type }
      : undefined,
    truth,
  };
}
