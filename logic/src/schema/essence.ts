/**
 * Essence Schema - "Logic of Essence" (Reflection, Appearance, Actuality)
 *
 * Essence represents the second phase of the dialectic: Mediation.
 * It is the logic of "what is true" behind the immediate being.
 *
 * Dialectical Role:
 * - Reflection: Positing, Presupposing, and Determining reflection
 * - Appearance: Existence emerging from Ground (Phenomenon)
 * - Actuality: Unity of Essence and Existence (Substance/Cause)
 *
 * Relationship to Form Engines:
 * - Essence is the logical domain for **Property** and **Aspect** Form Engines
 * - Property manifests Essential Laws (Reflection)
 * - Aspect manifests Spectral Relations (Appearance)
 * - Essence -> Concept (Transition via Reciprocity/Freedom)
 *
 * Schema Structure:
 * - Extends Shape (Logic in itself)
 * - Facets: Reflection, Appearance, Actuality structures
 * - Signature: Mediated transitions
 */

import { z } from 'zod';
import { BaseState, Type, Label } from './base';
import { ShapeCore, ShapeSchema, ShapeSignature } from './shape';

// Core
export const EssenceCore = ShapeCore.extend({
  type: Type.default("concept.Essence"),
});
export type EssenceCore = z.infer<typeof EssenceCore>;

// Facets for Essence logic
export const EssenceFacets = z.object({
  // Core dialectical state
  dialecticState: z.any().optional(),

  // Reflection: Internal mediation
  reflection: z.object({
    positing: z.array(z.string()),      // What is posited (Illusion)
    presupposing: z.array(z.string()),  // What is presupposed
    determining: z.array(z.string()),   // Essential determinations
  }).optional(),

  // Appearance: External manifestation
  appearance: z.object({
    world: z.string().optional(),       // World of appearance
    content: z.array(z.string()),       // Phenomenal content
    relation: z.string().optional(),    // Correlation (Whole/Part, Force/Expression)
  }).optional(),

  // Actuality: Concrete unity
  actuality: z.object({
    substance: z.string().optional(),   // Substantiality
    cause: z.string().optional(),       // Causality
    reciprocity: z.string().optional(), // Reciprocal action
  }).optional(),

  // Integration with Form Engines
  propertyRef: z.string().optional(),   // Reference to Property (Law)
  aspectRef: z.string().optional(),     // Reference to Aspect (Relation)

}).catchall(z.any());
export type EssenceFacets = z.infer<typeof EssenceFacets>;

// Signature: Mediated operations
export const EssenceSignature = ShapeSignature.extend({
  // Essence-specific operations
});
export type EssenceSignature = z.infer<typeof EssenceSignature>;

// Document structure
const EssenceDoc = z.object({
  core: EssenceCore,
  state: BaseState.default({}),
  signature: EssenceSignature.optional(),
  facets: z.record(z.string(), z.any()).default({}),
});

export const EssenceSchema = ShapeSchema.extend({
  shape: EssenceDoc,
});
export type Essence = z.infer<typeof EssenceSchema>;

// Helpers
function genId() {
  return `essence:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, '0')}`;
}

type CreateEssenceInput = {
  id?: string;
  name?: string;
  facets?: Record<string, unknown>;
  state?: z.input<typeof BaseState>;
};

export function createEssence(input: CreateEssenceInput): Essence {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: "concept.Essence", name: input.name },
      state: input.state ?? {},
      facets: input.facets ?? {},
    },
  };
  return EssenceSchema.parse(draft);
}

/**
 * Helper: Extract Reflection structure
 */
export function getReflection(essence: Essence): any | undefined {
  return (essence.shape.facets as any)?.reflection;
}

/**
 * Helper: Extract Appearance structure
 */
export function getAppearance(essence: Essence): any | undefined {
  return (essence.shape.facets as any)?.appearance;
}

/**
 * Helper: Extract Actuality structure
 */
export function getActuality(essence: Essence): any | undefined {
  return (essence.shape.facets as any)?.actuality;
}

