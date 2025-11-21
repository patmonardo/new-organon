// Knowledge delta: quantify what new knowledge a Kriya pass produced.

type LR = any;

export type KnowledgeDelta = {
  newRelations: number;
  newProperties: number;
  modalityUpgrades: number;
  score: number;
  details?: {
    newRelationIds: string[];
    newPropertyIds: string[];
    upgradedRelationIds: string[];
  };
};

export function computeKnowledgeDelta(
  before: { relations: LR[]; properties: LR[] },
  after: { relations: LR[]; properties: LR[] },
): KnowledgeDelta {
  const beforeRel = new Map<string, LR>();
  const beforeProp = new Map<string, LR>();
  for (const r of before.relations ?? []) beforeRel.set((r as any).id, r);
  for (const p of before.properties ?? []) beforeProp.set((p as any).id, p);

  const newRelationIds: string[] = [];
  const upgradedRelationIds: string[] = [];
  const newPropertyIds: string[] = [];

  for (const r of after.relations ?? []) {
    const id = (r as any).id;
    const prev = beforeRel.get(id);
    if (!prev) {
      newRelationIds.push(id);
      continue;
    }
    const prevMod = (prev as any)?.provenance?.modality?.kind as
      | string
      | undefined;
    const currMod = (r as any)?.provenance?.modality?.kind as
      | string
      | undefined;
    if ((prevMod ?? 'possible') !== 'actual' && currMod === 'actual') {
      upgradedRelationIds.push(id);
    }
  }

  for (const p of after.properties ?? []) {
    const id = (p as any).id;
    if (!beforeProp.has(id)) newPropertyIds.push(id);
  }

  const modalityUpgrades = upgradedRelationIds.length;
  const newRelations = newRelationIds.length;
  const newProperties = newPropertyIds.length;
  const score = newRelations + 0.5 * newProperties + 2 * modalityUpgrades;

  return {
    newRelations,
    newProperties,
    modalityUpgrades,
    score,
    details: { newRelationIds, newPropertyIds, upgradedRelationIds },
  };
}
