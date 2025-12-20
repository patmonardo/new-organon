import { describe, expect, it } from 'vitest';

import {
  TruthStepSchema,
  DiscursiveProjectionSchema,
  PreScienceArtifactSchema,
  PromotionRequestSchema,
  PromotionResultSchema,
  createRelation,
  createAspect,
  createJudgment,
} from '../../src/schema/index';

describe('FormProcessor contract schemas', () => {
  it('parses a minimal kernel TruthStep (relation-centric)', () => {
    const relation = createRelation({
      id: 'rel:1',
      type: 'system.Relation',
      kind: 'friend_of',
      source: { id: 'entity:cat', type: 'system.Entity' },
      target: { id: 'entity:pat', type: 'system.Entity' },
    });

    const truthStep = TruthStepSchema.parse({
      shape: {
        core: { id: 'truthStep:1', type: 'system.TruthStep', name: 'Cat friendliness' },
        state: {},
        relation,
        evalForm: { kind: 'ProjectionFactory:EvalForm' },
        certificate: { kind: 'opaque' },
        formShape: { kind: 'graph-delta', triples: 1 },
        meta: { mode: 'kernel' },
      },
    });

    expect(truthStep.shape.relation.shape.core.kind).toBe('friend_of');
  });

  it('parses a minimal discursive projection (aspect/judgment-centric)', () => {
    const aspect = createAspect({
      id: 'aspect:1',
      type: 'system.Aspect',
      name: 'Friendly cat (discursive)',
      facets: {
        note: 'discursive narrative only touches aspects',
      },
    });

    const judgment = createJudgment({
      id: 'judgment:1',
      type: 'system.Judgment',
      subject: { id: 'entity:cat', type: 'system.Entity' },
      predicate: 'isFriendly',
      object: { id: 'entity:pat', type: 'system.Entity' },
      modality: 'actual',
    });

    const projection = DiscursiveProjectionSchema.parse({
      shape: {
        core: { id: 'projection:1', type: 'system.DiscursiveProjection' },
        state: {},
        truthStepId: 'truthStep:1',
        aspects: [aspect],
        judgments: [judgment],
        narrative: 'That is a friendly cat. I can pet it.',
        trace: { source: 'perception' },
      },
    });

    expect(projection.shape.truthStepId).toBe('truthStep:1');
    expect(projection.shape.aspects[0].shape.core.id).toBe('aspect:1');
    expect(projection.shape.judgments[0].shape.core.id).toBe('judgment:1');
  });

  it('parses a pre-science artifact and promotion request/result', () => {
    const artifact = PreScienceArtifactSchema.parse({
      shape: {
        core: { id: 'artifact:1', type: 'system.PreScienceArtifact', name: 'HITS scores' },
        state: {},
        kind: 'graph-algo',
        procedureId: 'proc:hits',
        pipelineId: 'pipe:centrality',
        payload: { node: 'entity:X', score: 0.62 },
      },
    });

    const candidate = createRelation({
      id: 'rel:candidate:1',
      type: 'system.Relation',
      kind: 'central_to',
      source: { id: 'entity:X', type: 'system.Entity' },
      target: { id: 'entity:graph', type: 'system.Entity' },
    });

    const request = PromotionRequestSchema.parse({
      shape: {
        core: { id: 'promote:1', type: 'system.PromotionRequest' },
        state: {},
        evalForm: { kind: 'ProjectionFactory:EvalForm', note: 'opaque' },
        artifacts: [artifact],
        candidateRelations: [candidate],
      },
    });

    const resultOk = PromotionResultSchema.parse({
      ok: true,
      truthStep: {
        shape: {
          core: { id: 'truthStep:2', type: 'system.TruthStep' },
          state: {},
          relation: candidate,
        },
      },
    });

    expect(request.shape.artifacts[0].shape.core.id).toBe('artifact:1');
    expect(resultOk.ok).toBe(true);
    expect(resultOk.truthStep?.shape.relation.shape.core.kind).toBe('central_to');
  });
});
