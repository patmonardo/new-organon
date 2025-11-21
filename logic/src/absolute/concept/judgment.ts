import { createJudgment, type Judgment, type Modality, type Polarity } from '../../schema/judgment';
import { type ActiveProperty, type ActiveRelation } from '../../schema/active';

export type QualQuant = {
  // qualitative tags and quantitative measures we already surface
  tags?: string[];
  score?: number; // e.g., strength/confidence normalized [0,1]
  weight?: number;
};

export type JudgmentInputs = {
  relations?: ActiveRelation[];
  properties?: ActiveProperty[];
  qualquant?: Record<string, QualQuant>; // by subject id or relation id
};

export class JudgmentEngineLite {
  toJudgmentFromRelation(r: ActiveRelation, qq?: QualQuant): Judgment {
    const modality: Modality = r.active === true
      ? 'actual'
      : typeof r.confidence === 'number' && r.confidence >= 0.95
      ? 'necessary'
      : 'possible';
    const polarity: Polarity = r.revoked === true ? 'deny' : 'affirm';
    const warrant = {
      relationId: r.id,
      particularityOf: r.particularityOf,
      confidence: r.confidence ?? undefined,
      weight: r.weight ?? qq?.weight,
      tags: qq?.tags ?? [],
    };
    return createJudgment({
      type: 'system.Judgment',
      subject: { id: r.source?.id ?? 'unknown', type: r.source?.type ?? 'system.Thing' },
      predicate: r.type ?? r.kind ?? 'related_to',
      object: r.target ? { id: r.target.id, type: r.target.type ?? 'system.Thing' } : undefined,
      polarity,
      modality,
      warrant,
      facets: { kind: r.kind ?? 'relation' },
    });
  }

  toJudgments({ relations = [], qualquant = {} }: JudgmentInputs): Judgment[] {
    return relations.map((r) => this.toJudgmentFromRelation(r, qualquant[r.id]));
  }
}

export const DefaultJudgmentEngineLite = new JudgmentEngineLite();
