/**
 * Being Schema - "Logic of Being" (Quality, Quantity, Measure)
 *
 * Being represents the first phase of the dialectic: Immediacy.
 * It is the logic of "what is" before it is reflected upon.
 *
 * Dialectical Role:
 * - Quality: Determinateness identical with being (Reality/Negation)
 * - Quantity: Determinateness external to being (Number/Degree)
 * - Measure: Unity of Quality and Quantity (Specific Quantum)
 *
 * Relationship to Shape Engines:
 * - Being is the logical domain for the **Entity** Shape Engine
 * - Entity instances manifest the logic of Being
 * - Being -> Essence (Transition via Measure/Indifference)
 *
 * Schema Structure:
 * - Extends Shape (Logic in itself)
 * - Facets: Quality, Quantity, Measure structures
 * - Signature: Immediate transitions
 */

import { z } from 'zod';
import { BaseState, Type, Label } from './base';
import { ShapeCore, ShapeSchema, ShapeSignature } from './shape';

// Core
export const BeingCore = ShapeCore.extend({
  type: Type.default("concept.Being"),
});
export type BeingCore = z.infer<typeof BeingCore>;

// Facets for Being logic
export const BeingFacets = z.object({
  // Core dialectical state
  dialecticState: z.any().optional(),

  // Quality: Internal determination
  quality: z.object({
    reality: z.array(z.string()),   // Affirmative determinations
    negation: z.array(z.string()),  // Negative determinations
    limit: z.string().optional(),   // The boundary (Something/Other)
  }).optional(),

  // Quantity: External determination
  quantity: z.object({
    magnitude: z.number().optional(),
    unit: z.string().optional(),
    continuity: z.enum(['discrete', 'continuous']).optional(),
  }).optional(),

  // Measure: Qualitative quantity
  measure: z.object({
    ratio: z.string().optional(),
    standard: z.string().optional(),
    indifference: z.string().optional(), // Point of transition to Essence
  }).optional(),

  // Integration with Entity Engine
  entityRef: z.string().optional(), // Reference to concrete Entity

}).catchall(z.any());
export type BeingFacets = z.infer<typeof BeingFacets>;

// Signature: Immediate operations
export const BeingSignature = ShapeSignature.extend({
  // Being-specific operations can be added here
});
export type BeingSignature = z.infer<typeof BeingSignature>;

// Document structure
const BeingDoc = z.object({
  core: BeingCore,
  state: BaseState.default({}),
  signature: BeingSignature.optional(),
  facets: z.record(z.string(), z.any()).default({}),
});

export const BeingSchema = ShapeSchema.extend({
  shape: BeingDoc,
});
export type Being = z.infer<typeof BeingSchema>;

// Helpers
function genId() {
  return `being:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, '0')}`;
}

type CreateBeingInput = {
  id?: string;
  name?: string;
  facets?: Record<string, unknown>;
  state?: z.input<typeof BaseState>;
};

export function createBeing(input: CreateBeingInput): Being {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: "concept.Being", name: input.name },
      state: input.state ?? {},
      facets: input.facets ?? {},
    },
  };
  return BeingSchema.parse(draft);
}

/**
 * Helper: Extract Quality structure
 */
export function getQuality(being: Being): any | undefined {
  return (being.shape.facets as any)?.quality;
}

/**
 * Helper: Extract Quantity structure
 */
export function getQuantity(being: Being): any | undefined {
  return (being.shape.facets as any)?.quantity;
}

/**
 * Helper: Extract Measure structure
 */
export function getMeasure(being: Being): any | undefined {
  return (being.shape.facets as any)?.measure;
}

