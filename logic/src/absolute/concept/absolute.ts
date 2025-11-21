/**
 * AbsoluteConcept — a light, noumenal integrator for Qual/Quant logic.
 *
 * Pure functions only. No bus, no persistence. Intended for tests/demos and
 * higher-order control flows that sit above the Form (phenomenal) engines.
 */

import { DefaultJudgmentEngineLite } from './judgment';
import type { Judgment } from '../../schema/judgment';
import type { ActiveProperty, ActiveAspect } from '../../schema/active';
import { intentsForGround, type IntentRegistry } from './concept';
import { evaluateAspectTruth, type AspectTruth } from './truth';

// Optional: future derivations (placeholders for now)
import { deriveMechanisticEdges, type MechanismEdge } from './mechanism';
import { deriveChemisticEdges, type ChemismEdge } from './chemism';
import { planTeleology, type TeleologyGoal } from './teleology';
import { computeAspectConflicts } from '../form/projection.conflict';
import { assessStageForAspects, type Stage } from './stage';
import { computeAttentionWeights, type AttentionEntry } from './attention';
import synthesizeIdeasFromJudgments, { type IdeaConfig, type Idea } from './idea';

export type QualQuantInput = {
  // per relation id qualitative/quantitative hints
  [relationId: string]: { tags?: string[]; score?: number; weight?: number };
};

export type AbsoluteConceptInputs = {
  relations?: ActiveAspect[];
  properties?: ActiveProperty[];
  qualquant?: QualQuantInput;
  intentRegistry?: IntentRegistry;
  // opt-in: emit Idea-stage aggregates derived from Judgments
  emitIdeaStage?: boolean;
  ideaConfig?: IdeaConfig;
  // future: syllogisms, mechanisms, chemisms inputs
};

export type AbsoluteConceptOutputs = {
  judgments: Judgment[];
  // derived edges (optional, empty until implemented)
  mechanism: MechanismEdge[];
  chemism: ChemismEdge[];
  teleology: TeleologyGoal[];
  // samyama bhumi assessments for provided aspects
  bhumis?: Stage[];
  // per-aspect realm classification: 'nature' | 'spirit' | 'logic' | 'unknown'
  // Note: 'logic' is intentionally considered last — Nature & Spirit are
  // determined first, then remaining aspects may be classified as Logic
  // (Transcendental/Discriminative) when heuristics suggest so.
  aspectRealms?: Array<{
    aspectId: string;
    realm: 'nature' | 'spirit' | 'logic' | 'unknown';
  }>;
  // controller/planning helpers from intents
  intentCandidates: Array<{
    intentId: string;
    score: number;
    payload: unknown;
  }>;
  // optional truth evaluations for aspects provided
  aspectTruth?: AspectTruth[];
  // attention weights per aspect (normalized 0..1)
  attention?: AttentionEntry[];
  // generator metadata for audits (algorithm id/version, role, params)
  generatorMetadata?: { id: string; version?: string; role?: string; params?: unknown };
  ideas?: Idea[];
};

export function processAbsoluteConcept(
  input: AbsoluteConceptInputs,
): AbsoluteConceptOutputs {
  const relations = input.relations ?? [];
  const properties = input.properties ?? [];
  const qualquant = { ...(input.qualquant ?? {}) } as QualQuantInput;

  // Pre-pass: derive conflicts and annotate qualquant with negative tags
  const conflicts = computeAspectConflicts({ relations });
  for (const id of conflicts.conflictedIds) {
    const entry = (qualquant[id] = qualquant[id] ?? {});
    entry.tags = Array.from(new Set([...(entry.tags ?? []), 'contradict']));
    entry.weight = Math.min(entry.weight ?? 0, -1);
    entry.score = Math.min(entry.score ?? 0, 0);
  }

  // 1) First-order: Aspects → Judgments (S–P–O with polarity/modality)
  const judgments = DefaultJudgmentEngineLite.toJudgments({
    relations,
    properties,
    qualquant,
  });

  // 2) Optional derivations (mechanism/chemism/teleology) — placeholders
  const mechanism = deriveMechanisticEdges({
    relations,
    properties,
    judgments,
  } as any);
  const chemism = deriveChemisticEdges({
    relations,
    properties,
    judgments,
  } as any);

  // 3) Intent candidates for planning/control
  const intentCandidates = intentsForGround(
    { relations: relations as unknown[], properties: properties as unknown[] },
    input.intentRegistry ?? { intents: [] },
  );

  // 4) Truth evaluation (pure): evaluate truth for provided aspects from qualquant
  const aspectTruth: AspectTruth[] = relations.map((r) =>
    evaluateAspectTruth(r as any, [
      {
        score: input.qualquant?.[r.id]?.score,
        weight: input.qualquant?.[r.id]?.weight,
        tags: input.qualquant?.[r.id]?.tags,
        source: r.particularityOf,
      },
    ]),
  );

  // 4b) Realm classification: seed a small classifier so AbsoluteConcept "knows"
  // which aspects belong to Nature, Spirit, or (as a last resort) Logic.
  // Heuristics (checked in order):
  // 1) Spirit: qualquant tag 'spirit', meta.dialectic === 'absolute', or type contains ':absolute'
  // 2) Nature: qualquant tag 'nature', type contains 'linked' or 'particular'
  // 3) Logic: qualquant tag 'logic' or type mentions 'judg'|'syllog'|'teleolog'|'truth' —
  //    assigned last because Logic (Transcendental Logic) discriminates what the
  //    Aspects of Nature and Spirit are not.
  // 4) Unknown: fallback
  const aspectRealms = relations.map((r) => {
    const q = input.qualquant?.[r.id];
    const tags = q?.tags ?? [];
    const metaDialectic = (r as any)?.meta?.dialectic;
    const t = String((r as any).type ?? '').toLowerCase();
    let realm: 'nature' | 'spirit' | 'logic' | 'unknown' = 'unknown';

    if (
      tags.includes('spirit') ||
      metaDialectic === 'absolute' ||
      t.includes(':absolute')
    ) {
      realm = 'spirit';
    } else if (
      tags.includes('nature') ||
      t.includes('linked') ||
      t.includes('particular')
    ) {
      realm = 'nature';
    } else if (
      tags.includes('logic') ||
      t.includes('judg') ||
      t.includes('syllog') ||
      t.includes('teleolog') ||
      t.includes('truth')
    ) {
      realm = 'logic';
    }

    return { aspectId: (r as any).id as string, realm };
  });

  // 5) Teleology planning (pure): infer goals from confident or hinted aspects
  const teleology = planTeleology({
    relations: relations as any,
    judgments,
    qualquant: input.qualquant,
    aspectTruth,
    threshold: 0.99,
  });

  // 6) Samyama bhumi mapping (pure): seed 11-stage bhumi assessments from truth
  // (left intentionally generic — bhumi is more about Jnana/Path Knowledge; keep
  // aspect-level mapping as a conservative seed)
  const bhumis = assessStageForAspects(aspectTruth, input.qualquant, undefined).map(
    (b) => ({
      ...b,
      provenance: {
        algorithm: 'samyama-v1',
        version: '0.1',
        inputs: { truth: aspectTruth?.find((t) => t.aspectId === b.aspectId) ?? null },
        timestamp: new Date().toISOString(),
      },
    }),
  );

  // 7) Attention: compute normalized attention weights from truth + qualquant
  const attention = computeAttentionWeights(aspectTruth, input.qualquant, {
    qualquantBias: 1.0,
    contradictionPenalty: 0.5,
  });

  const generatorMetadata = { id: 'absolute-concept-generator', version: '0.1', role: 'concept-generator', params: { attentionConfig: { qualquantBias: 1.0 } } };
  const ideas = input.emitIdeaStage ? synthesizeIdeasFromJudgments(judgments, input.ideaConfig) : undefined;

  return {
    judgments,
    mechanism,
    chemism,
    teleology,
    intentCandidates,
    bhumis,
  aspectRealms,
  aspectTruth,
  attention,
  ideas,
  generatorMetadata,
  };
}

export default processAbsoluteConcept;
