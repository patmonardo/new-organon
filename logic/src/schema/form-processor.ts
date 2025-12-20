import { z } from "zod";

import { BaseCore, BaseSchema, BaseState, Type, Label } from "./base";
import { RelationSchema } from "./relation";
import { AspectSchema } from "./aspect";
import { JudgmentSchema } from "./judgment";

/**
 * FormProcessor contract (boundary vocabulary)
 *
 * - Kernel-side "Form" is non-discursive: it yields TruthSteps (relational/scientific).
 * - RelativeForm can only receive/express this discursively as aspectual projections.
 */

// =====================
// Kernel return: TruthStep
// =====================

export const TruthStepCore = BaseCore.extend({
  type: Type,
  name: Label.optional(),
});
export type TruthStepCore = z.infer<typeof TruthStepCore>;

export const TruthStepDoc = z.object({
  core: TruthStepCore,
  state: BaseState.default({}),

  /**
   * The essential output form: Relation is the truth of the Reflected.
   * This is the kernel’s "scientific" return surface.
   */
  relation: RelationSchema,

  /**
   * Opaque carrier for the submitted Pure/Application Form
   * (e.g. ProjectionFactory:EvalForm), if present.
   */
  evalForm: z.unknown().optional(),

  /**
   * Opaque certificate/proof artifact (optional).
   * This is intentionally unspecified at this stage.
   */
  certificate: z.unknown().optional(),

  /**
   * Optional form-shape/graph delta or snapshot.
   * This is intentionally opaque to keep the boundary flexible.
   */
  formShape: z.unknown().optional(),

  meta: z.record(z.string(), z.unknown()).optional(),
});

export const TruthStepSchema = BaseSchema.extend({ shape: TruthStepDoc });
export type TruthStep = z.infer<typeof TruthStepSchema>;

// =====================
// Pre-Science: artifacts (graph/ML moments)
// =====================

export const PreScienceArtifactKind = z.enum([
  "graph-algo",
  "ml-pipeline",
  "measurement",
  "trace",
  "projection",
]);
export type PreScienceArtifactKind = z.infer<typeof PreScienceArtifactKind>;

export const PreScienceArtifactCore = BaseCore.extend({
  type: Type,
  name: Label.optional(),
});
export type PreScienceArtifactCore = z.infer<typeof PreScienceArtifactCore>;

export const PreScienceArtifactDoc = z.object({
  core: PreScienceArtifactCore,
  state: BaseState.default({}),
  kind: PreScienceArtifactKind,

  /**
   * Optional identifiers to keep Procedure→Pipeline provenance explicit.
   */
  procedureId: z.string().optional(),
  pipelineId: z.string().optional(),

  /**
   * Opaque payload: scores, embeddings, clusters, anomalies, traces, etc.
   */
  payload: z.unknown().optional(),

  meta: z.record(z.string(), z.unknown()).optional(),
});

export const PreScienceArtifactSchema = BaseSchema.extend({
  shape: PreScienceArtifactDoc,
});
export type PreScienceArtifact = z.infer<typeof PreScienceArtifactSchema>;

// =====================
// Emergence: promotion request/result
// =====================

export const PromotionRequestCore = BaseCore.extend({
  type: Type,
  name: Label.optional(),
});
export type PromotionRequestCore = z.infer<typeof PromotionRequestCore>;

export const PromotionRequestDoc = z.object({
  core: PromotionRequestCore,
  state: BaseState.default({}),

  /**
   * Submitted Pure/Application Form (ProjectionFactory:EvalForm), opaque by design.
   */
  evalForm: z.unknown().optional(),

  /**
   * Pre-scientific outputs (Procedure→Pipeline artifacts) that motivate the request.
   */
  artifacts: z.array(PreScienceArtifactSchema).default([]),

  /**
   * Optional candidate relations proposed by Pre-Science.
   * Kernel may accept/reject/transform into a scientific TruthStep.
   */
  candidateRelations: z.array(RelationSchema).default([]),

  meta: z.record(z.string(), z.unknown()).optional(),
});

export const PromotionRequestSchema = BaseSchema.extend({
  shape: PromotionRequestDoc,
});
export type PromotionRequest = z.infer<typeof PromotionRequestSchema>;

export const PromotionResultDoc = z.object({
  /**
   * Success returns a TruthStep; failure returns an obstruction.
   * Both forms remain intentionally lightweight.
   */
  ok: z.boolean(),
  truthStep: TruthStepSchema.optional(),
  obstruction: z.unknown().optional(),
  demandedMediation: z.unknown().optional(),
  meta: z.record(z.string(), z.unknown()).optional(),
});
export const PromotionResultSchema = PromotionResultDoc;
export type PromotionResult = z.infer<typeof PromotionResultSchema>;

// ==========================================
// Relative receipt: DiscursiveProjection (Aspect)
// ==========================================

export const DiscursiveProjectionCore = BaseCore.extend({
  type: Type,
  name: Label.optional(),
});
export type DiscursiveProjectionCore = z.infer<typeof DiscursiveProjectionCore>;

export const DiscursiveProjectionDoc = z.object({
  core: DiscursiveProjectionCore,
  state: BaseState.default({}),

  /**
   * Link to a kernel TruthStep (if the projection came from kernel return).
   */
  truthStepId: z.string().optional(),

  /**
   * Discursive narrative can only touch Aspects.
   */
  aspects: z.array(AspectSchema).default([]),

  /**
   * Discursive articulation as judgments (optional).
   */
  judgments: z.array(JudgmentSchema).default([]),

  /**
   * Human-facing rendering (optional).
   */
  narrative: z.string().optional(),

  /**
   * Trace bundle / provenance (opaque).
   */
  trace: z.unknown().optional(),

  meta: z.record(z.string(), z.unknown()).optional(),
});

export const DiscursiveProjectionSchema = BaseSchema.extend({
  shape: DiscursiveProjectionDoc,
});
export type DiscursiveProjection = z.infer<typeof DiscursiveProjectionSchema>;
