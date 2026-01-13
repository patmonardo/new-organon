/**
 * Property Schema - "Middle Term / Functor"
 *
 * Property represents Law/Invariant as a Middle Term that mediates between worlds.
 * It is the functor mapping between ideal (essence) and real (existence).
 *
 * Dialectical Role:
 * - Sublates PropertyStore (ideal) and PropertyValue (real)
 * - Acts as middle term connecting Entity ↔ Aspect
 * - Embodies universal law through invariants
 * - Mediates Thing ↔ Relation (Entity ↔ Aspect)
 *
 * Relationship to Shape Engines:
 * - Used by PropertyEngine to manage invariant laws
 * - Property.facets.law contains invariants and universality
 * - Property.facets.facticity contains grounding evidence
 * - Property.facets.mediates shows Entity ↔ Aspect connections
 *
 * Shape Engine Pattern:
 * - Facets: Law structure (invariants, facticity, mediation)
 * - Signature: Operational interface (what this property does)
 * - State: Runtime status
 */

import { z } from 'zod';
import { BaseCore, BaseState, Type, Label } from './base';

// Core mirrors ShapeCore (skeletal)
export const PropertyCore = BaseCore.extend({
  type: Type, // e.g., "system.Property"
  name: Label.optional(),
});
export type PropertyCore = z.infer<typeof PropertyCore>;

// Open signature for extensibility
export const PropertySignature = z.object({}).catchall(z.any());
export type PropertySignature = z.infer<typeof PropertySignature>;

// Facets structure for law/invariant data
export const PropertyFacets = z
  .object({
    // Core dialectical state
    dialecticState: z.any().optional(),

    // Current phase
    phase: z.string().optional(),

    // Law structure: invariants and universality
    law: z
      .object({
        invariants: z.array(
          z.object({
            id: z.string(),
            constraint: z.string(),
            predicate: z.string().optional(),
            universality: z.enum(['necessary', 'conditional']),
          }),
        ),
        universality: z.enum(['necessary', 'conditional']),
      })
      .optional(),

    // Facticity: grounding evidence/witnesses
    facticity: z
      .object({
        grounds: z.array(z.string()), // Moment names that ground this property
        conditions: z.array(z.string()), // Conditions for validity
        evidence: z.array(
          z.object({
            name: z.string(),
            definition: z.string(),
            type: z.string(),
          }),
        ),
      })
      .optional(),

    // Mediation structure: Entity ↔ Aspect
    mediates: z
      .object({
        fromEntities: z.array(z.string()), // Entity IDs
        toAspects: z.array(z.string()), // Aspect IDs
      })
      .optional(),

    // Evaluation context
    context: z.any().optional(),

    // Additional property-specific facets via catchall
  })
  .catchall(z.any());
export type PropertyFacets = z.infer<typeof PropertyFacets>;

// ==========================================
// PROPERTY SHAPE (Pure Form for Neo4j)
// ==========================================
export const PropertyShapeSchema = z.object({
  id: z.string(),
  type: Type,
  name: Label.optional(),

  // Runtime-ish state (open map for discursive engines)
  state: BaseState.optional().default({}),

  // Signature: operational interface
  signature: PropertySignature.optional(),

  // Facets: law/invariant structure
  facets: PropertyFacets.optional().default({}),

  // State metadata
  status: z.string().optional(),
  tags: z.array(z.string()).optional(),
  meta: z.record(z.string(), z.unknown()).optional(),

  // Timestamps
  createdAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
  updatedAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
});
export type PropertyShapeRepo = z.infer<typeof PropertyShapeSchema>;
// export type PropertyShape = PropertyShapeRepo;

// Type exports
export type PropertyCoreOut = z.output<typeof PropertyCore>;
export type BaseStateOut = z.output<typeof BaseState>;
