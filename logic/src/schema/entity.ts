/**
 * Entity Schema - "Fact → Existence"
 *
 * Entity represents Fact coming into Existence - the dialectical state of actualization.
 * It is the concrete instantiation of abstract logical structures.
 *
 * Dialectical Role:
 * - Embodies the passage from essence to existence
 * - Stores dialectical state in facets (the "in-itself")
 * - Exposes operational interface via signature (the "for-another")
 * - Represents "what is" as opposed to "what could be"
 *
 * Relationship to Shape Engines:
 * - Used by EntityEngine to manage existential instances
 * - Entity.facets.dialecticState contains the embedded DialecticState
 * - Entity.signature contains moments as operational interface
 *
 * Shape Engine Pattern:
 * - Facets: Internal dialectical structure (moments, invariants, phase)
 * - Signature: External operational interface (what this entity does)
 * - State: Runtime status and metadata
 */

import { z } from 'zod';
import { BaseCore, BaseSchema, BaseState, Type, Label } from './base';

// EntityRef
export const EntityRef = z.object({
  id: z.string().min(1),
  type: Type,
  // Optional dialectical role for tracing (e.g. preserved, negated, sublated)
  role: z.string().optional(),
});
export type EntityRef = z.infer<typeof EntityRef>;

// Core
export const EntityCore = BaseCore.extend({
  type: Type,
  name: Label.optional(),
  description: z.string().optional(),
});
export type EntityCore = z.infer<typeof EntityCore>;

// EntityState: keep common runtime fields (status/tags/meta) on top of BaseState
export const EntityState = BaseState.extend({
  status: z.string().optional(),
  tags: z.array(z.string()).optional(),
  meta: z.record(z.string(), z.unknown()).optional(),
});
export type EntityState = z.infer<typeof EntityState>;

// Signature / facets
export const EntitySignature = z.object({}).catchall(z.any());
export type EntitySignature = z.infer<typeof EntitySignature>;

// Facets structure for dialectical data
// Entity stores the complete dialectical state as it exists
export const EntityFacets = z
  .object({
    // Core dialectical state (from Dialectic IR)
    dialecticState: z.any().optional(), // DialecticState - avoiding circular dependency

    // Current phase in dialectical progression
    phase: z.string().optional(), // CpuGpuPhase

    // Moments active in this entity
    moments: z.array(z.any()).optional(),

    // Invariants that must hold
    invariants: z.array(z.any()).optional(),

    // Evaluation context
    context: z.any().optional(),

    // Additional entity-specific facets via catchall
  })
  .catchall(z.any());
export type EntityFacets = z.infer<typeof EntityFacets>;

// ==========================================
// ENTITY SHAPE (Empirical - Principled Effect)
// ==========================================
// Entity is the reciprocation of Form (Rational) and Data (Empirical)
// It holds formId (reference to Principle) and values (actual data)
export const EntityShapeSchema = z.object({
  id: z.string(),
  type: Type,
  name: Label.optional(),
  description: z.string().optional(),

  // Reference to Form Principle (Form:Entity relationship)
  formId: z.string(), // Required - every Entity has a Form

  // Particulars: sublated particulars linked to this Entity
  // These are condensed, contextualized instances that have been
  // preserved/negated/elevated in the process of sublation.
  particulars: z.array(EntityRef).optional(),

  // Actual field values (Data from Model system)
  values: z.record(z.string(), z.any()).optional().default({}),

  // Signature: operational interface
  signature: EntitySignature.optional(),

  // Facets: dialectical structure
  facets: EntityFacets.optional(),

  // Runtime state (Empirical concerns)
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
export type EntityShapeRepo = z.infer<typeof EntityShapeSchema>;
// export type EntityShape = EntityShapeRepo;

// ==========================================
// ENTITY DOCUMENT (Envelope)
// ==========================================
// Shape doc
const EntityDoc = z.object({
  core: EntityCore,
  state: EntityState.default({}),
  signature: EntitySignature.optional(),
  facets: z.record(z.string(), z.any()).default({}),
  entity: EntityShapeSchema.optional(), // EMBED EntityShape
});

export const EntitySchema = BaseSchema.extend({
  shape: EntityDoc,
});
// export type Entity = z.infer<typeof EntitySchema>;

// Type exports
export type EntityCoreOut = z.output<typeof EntityCore>;
export type EntityStateOut = z.output<typeof EntityState>;
