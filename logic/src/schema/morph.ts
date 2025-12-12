/**
 * Morph Schema - "Ground = Shape + Context"
 *
 * Morph represents the Ground (Grund) - the unity of Shape and Context.
 * It is the active container that holds moments and enables transformation.
 *
 * Dialectical Role:
 * - Unifies Shape (Logic) and Context (Scope) into a concrete Ground
 * - Acts as the "active unity" that holds moments together
 * - Defines the transformation principle (Reason/Ratio)
 * - Mediates the transition from Essence to Concept
 *
 * Relationship to Shape Engines:
 * - Used by MorphEngine to manage grounds and transformations
 * - Morph.facets.container shows what it holds (active unity)
 * - Morph.facets.transformation describes the mechanism of change
 * - Morph.composition defines pipeline/composite structures
 *
 * Shape Engine Pattern:
 * - Facets: Ground structure (container, transformation)
 * - Signature: Operational interface (transform/pipeline ops)
 * - State: Runtime status and composition state
 */

import { z } from "zod";
import { BaseSchema, BaseState, BaseCore, Type, Id, Label } from "./base";

// Core/state (keep morph-specific core, but align with uniform base)
export const MorphCore = BaseCore.extend({
  type: Type, // e.g., "system.Morph"
  inputType: Label.default("FormShape"),
  outputType: Label.default("FormShape"),
  transformFn: z.string().optional(),
});
export type MorphCore = z.infer<typeof MorphCore>;

export const MorphState = BaseState;
export type MorphState = z.infer<typeof MorphState>;

// Optional composition (kept as-is)
export const MorphComposition = z
  .object({
    kind: z.enum(["single", "pipeline", "composite"]).default("single"),
    mode: z.enum(["sequential", "parallel"]).optional(),
    steps: z.array(Id).default([]),
  })
  .default({ kind: "single", steps: [] });
export type MorphComposition = z.infer<typeof MorphComposition>;

// Facets structure for ground/transformation data
export const MorphFacets = z.object({
  // Core dialectical state
  dialecticState: z.any().optional(),

  // Current phase
  phase: z.string().optional(),

  // Container: the active unity that holds moments
  container: z.object({
    holds: z.array(z.string()), // IDs of held forms
    activeUnity: z.array(z.object({
      name: z.string(),
      definition: z.string(),
      contains: z.string().optional(),
    })),
  }).optional(),

  // Transformation: the principle of change
  transformation: z.object({
    principle: z.string().optional(),
    mechanism: z.string().optional(),
    transitions: z.array(z.object({
      from: z.string(),
      to: z.string(),
      mechanism: z.string().optional(),
      reason: z.string().optional(),
    })),
  }).optional(),

  // Evaluation context
  context: z.any().optional(),

  // Additional morph-specific facets via catchall
}).catchall(z.any());
export type MorphFacets = z.infer<typeof MorphFacets>;

// Shape: uniform core/state/signature/facets + morph extras
export const MorphShape = z.object({
  core: MorphCore,
  state: MorphState.default({}),
  signature: z.object({}).catchall(z.any()).optional(),
  facets: z.record(z.string(), z.any()).default({}),
  composition: MorphComposition,
  config: z.record(z.string(), z.unknown()).default({}),
  meta: z.record(z.string(), z.unknown()).default({}),
});
export type MorphShape = z.infer<typeof MorphShape>;

// Schema
export const MorphSchema = BaseSchema.extend({
  shape: MorphShape,
});
export type Morph = z.infer<typeof MorphSchema>;

// Create/update (uniform signature handling)
export function createMorph(input: {
  id?: string;
  type: z.input<typeof Type>;
  name?: string;
  description?: string;

  inputType?: z.input<typeof Label>;
  outputType?: z.input<typeof Label>;
  transformFn?: string;

  composition?: z.input<typeof MorphComposition>;
  config?: Record<string, unknown>;
  meta?: Record<string, unknown>;

  signature?: Record<string, unknown>;
  facets?: Record<string, unknown>;

  state?: z.input<typeof MorphState>;
  version?: string;
  ext?: Record<string, unknown>;
}): Morph {
  const id =
    input.id ??
    `morph:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
      .toString(36)
      .padStart(4, "0")}`;

  return MorphSchema.parse({
    shape: {
      core: MorphCore.parse({
        id,
        type: input.type,
        name: input.name,
        description: input.description,
        inputType: input.inputType ?? "FormShape",
        outputType: input.outputType ?? "FormShape",
        transformFn: input.transformFn,
      }),
      state: MorphState.parse(input.state ?? {}),
      signature: input.signature,
      facets: input.facets ?? {},
      composition: MorphComposition.parse(input.composition ?? {}),
      config: input.config ?? {},
      meta: input.meta ?? {},
    },
    revision: 0,
    version: input.version,
    ext: input.ext ?? {},
  });
}

type UpdateMorphPatch = Partial<{
  core: Partial<z.input<typeof MorphCore>>;
  state: Partial<z.input<typeof MorphState>>;
  composition: z.input<typeof MorphComposition>;
  config: Record<string, unknown>;
  meta: Record<string, unknown>;
  signature: Record<string, unknown> | null | undefined; // null => clear, undefined => preserve
  facets: Record<string, unknown>;
  version: string;
  ext: Record<string, unknown>;
}>;

export function updateMorph(current: Morph, patch: UpdateMorphPatch): Morph {
  const nextSignature =
    patch.signature === null
      ? undefined
      : patch.signature !== undefined
      ? patch.signature
      : current.shape.signature;

  const composition =
    patch.composition !== undefined
      ? MorphComposition.parse(patch.composition)
      : current.shape.composition;

  return MorphSchema.parse({
    ...current,
    shape: {
      ...current.shape,
      core: MorphCore.parse({ ...current.shape.core, ...(patch.core ?? {}) }),
      state: MorphState.parse({ ...current.shape.state, ...(patch.state ?? {}) }),
      composition,
      config: patch.config ?? current.shape.config,
      meta: patch.meta ?? current.shape.meta,
      signature: nextSignature,
      facets: patch.facets ?? current.shape.facets,
    },
    revision: (current.revision ?? 0) + 1,
    version: patch.version ?? current.version,
    ext: { ...current.ext, ...(patch.ext ?? {}) },
  });
}

// Ergonomics
export function defineMorph(config: {
  id?: string;
  type: z.input<typeof Type>;
  name?: string;
  description?: string;
  transformFn?: string;
  inputType?: z.input<typeof Label>;
  outputType?: z.input<typeof Label>;
  config?: Record<string, unknown>;
  meta?: Record<string, unknown>;
}): Morph {
  return createMorph({
    ...config,
    composition: { kind: "single", steps: [] },
  });
}

export function defineMorphPipeline(
  id: string,
  name: string,
  stepIds: z.input<typeof Id>[],
  options?: {
    type?: z.input<typeof Type>;
    description?: string;
    inputType?: z.input<typeof Label>;
    outputType?: z.input<typeof Label>;
    transformFn?: string;
    config?: Record<string, unknown>;
    meta?: Record<string, unknown>;
    version?: string;
    ext?: Record<string, unknown>;
  }
): Morph {
  return createMorph({
    id,
    type: options?.type ?? ("system.Morph" as z.input<typeof Type>),
    name,
    description: options?.description,
    inputType: options?.inputType ?? "FormShape",
    outputType: options?.outputType ?? "FormShape",
    transformFn: options?.transformFn,
    composition: { kind: "pipeline", mode: "sequential", steps: stepIds },
    config: options?.config,
    meta: options?.meta,
    version: options?.version,
    ext: options?.ext,
  });
}

/**
 * Helper: Get container structure from Morph facets
 */
export function getMorphContainer(morph: Morph): any | undefined {
  return (morph.shape.facets as any)?.container;
}

/**
 * Helper: Get transformation structure from Morph facets
 */
export function getMorphTransformation(morph: Morph): any | undefined {
  return (morph.shape.facets as any)?.transformation;
}

