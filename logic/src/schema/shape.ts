/**
 * Shape Schema - "Logic in Itself"
 *
 * Shape represents the pure dialectical form/structure independent of application.
 * It is the foundational schema for storing dialectical states and their transformations.
 *
 * Dialectical Role:
 * - Holds the complete DialecticState (moments, invariants, forces, transitions)
 * - Supports all dialectic commands (state.transition, moment.activate, force.apply, invariant.check)
 * - Represents "what the logic is" without "where it applies" (that's Context)
 *
 * Relationship to Other Schemas:
 * - Shape + Context = Morph (Ground)
 * - Shape is used by all Form Engines to store dialectical structure
 * - Shape.facets.dialecticState contains the full IR representation
 */

import { z } from 'zod';
import { BaseCore, BaseSchema, BaseState, Type, Label } from './base';
import type { DialecticState, CpuGpuPhase } from './dialectic';

// Principle-level Shape (no UI concerns)
export const ShapeCore = BaseCore.extend({
  type: Type, // e.g., "system.Shape", "form.Entity", "concept.Being"
  name: Label.optional(), // optional display name
});
export type ShapeCore = z.infer<typeof ShapeCore>;

// Open signature for extensibility (operational interface)
// Typically stores moments as the signature structure
export const ShapeSignature = z.object({}).catchall(z.any());
export type ShapeSignature = z.infer<typeof ShapeSignature>;

// Facets structure for dialectical data
// This is where the "in-itself" dialectical structure lives
export const ShapeFacets = z.object({
  // Core dialectical state (from Dialectic IR)
  dialecticState: z.any().optional(), // DialecticState - avoiding circular dependency

  // Current phase in dialectical progression
  phase: z.string().optional(), // CpuGpuPhase

  // Evaluation context reference
  context: z.any().optional(),

  // Additional form-specific facets via catchall
}).catchall(z.any());
export type ShapeFacets = z.infer<typeof ShapeFacets>;

const ShapeDoc = z.object({
  core: ShapeCore,
  state: BaseState.default({}),
  signature: ShapeSignature.optional(),
  facets: z.record(z.string(), z.any()).default({}), // Using record for flexibility
});

export const ShapeSchema = BaseSchema.extend({
  shape: ShapeDoc,
});
export type Shape = z.infer<typeof ShapeSchema>;

// Helpers
function genId() {
  return `shape:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, '0')}`;
}

type CreateShapeInput = {
  id?: string;
  type: string;
  name?: string;
  signature?: z.input<typeof ShapeSignature>;
  facets?: Record<string, unknown>;
  state?: z.input<typeof BaseState>;
};

export function createShape(input: CreateShapeInput): Shape {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: input.type, name: input.name },
      state: input.state ?? {},
      signature: input.signature,
      facets: input.facets ?? {},
    },
  };
  return ShapeSchema.parse(draft);
}

export type ShapeCoreOut = z.output<typeof ShapeCore>;

type UpdateShapePatch = Partial<{
  core: Partial<z.output<typeof ShapeCore>>;
  state: Partial<z.output<typeof BaseState>>;
  signature: Record<string, unknown> | null | undefined; // null => clear, undefined => preserve
  facets: Record<string, unknown>;
  version: string;
  ext: Record<string, unknown>;
}>;

export function updateShape(doc: Shape, patch: UpdateShapePatch): Shape {
  const nextSignature =
    patch.signature === null
      ? undefined // explicit clear
      : patch.signature !== undefined
      ? patch.signature // replace
      : doc.shape.signature; // preserve

  const next = {
    ...doc,
    shape: {
      ...doc.shape,
      core: { ...(doc.shape.core as ShapeCore), ...(patch.core ?? {}) },
      state: {
        ...(doc.shape.state as z.output<typeof BaseState>),
        ...(patch.state ?? {}),
      },
      signature: nextSignature,
      facets: patch.facets ?? doc.shape.facets,
    },
    version: patch.version ?? doc.version,
    ext: patch.ext ?? doc.ext,
    revision: (doc.revision ?? 0) + 1,
  };
  return ShapeSchema.parse(next);
}

/**
 * Helper: Extract dialectic state from Shape facets
 * Used by Form Engines to access the embedded dialectical structure
 */
export function getDialecticState(shape: Shape): any | undefined {
  return (shape.shape.facets as any)?.dialecticState;
}

/**
 * Helper: Get current dialectical phase
 */
export function getDialecticPhase(shape: Shape): string | undefined {
  return (shape.shape.facets as any)?.phase;
}

