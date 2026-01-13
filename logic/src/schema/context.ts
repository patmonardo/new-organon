/**
 * Context Schema - "Scope of Validity"
 *
 * Context represents the scope and conditions under which dialectical logic operates.
 * It is the foundational schema for situating evaluation and establishing validity boundaries.
 *
 * Dialectical Role:
 * - Defines presuppositions (what is taken as given)
 * - Establishes modal scope (actual/possible/necessary)
 * - Specifies conditioning constraints (what must hold)
 * - Tracks domain of discourse (what concepts are in play)
 *
 * Relationship to Other Schemas:
 * - Shape + Context = Morph (Ground)
 * - Context provides the "where" to Shape's "what"
 * - Used by ContextEngine to manage evaluation scope
 *
 * Foundation Pattern:
 * - Context.facets.presuppositions: What is posited as given
 * - Context.facets.scope: Modal and domain boundaries
 * - Context.facets.conditions: Constraints that must hold
 */

import { z } from 'zod';
import { BaseState, Type, Id, Label } from './base';
import { EntityRef } from './entity';

// Modal scope types for dialectical evaluation
export const ModalScope = z.enum([
  'actual',
  'possible',
  'necessary',
  'contingent',
]);
export type ModalScope = z.infer<typeof ModalScope>;

// Presupposition: what is posited as given in this context
export const Presupposition = z.object({
  name: z.string(),
  definition: z.string(),
  posited: z.boolean().default(true),
});
export type Presupposition = z.infer<typeof Presupposition>;

// Scope specification: modal and domain boundaries
export const ScopeSpec = z.object({
  modal: ModalScope,
  domain: z.array(z.string()), // Concept IDs in scope
  phase: z.string().optional(), // CpuGpuPhase
});
export type ScopeSpec = z.infer<typeof ScopeSpec>;

// Conditioning constraint: what must hold for validity
export const ContextCondition = z.object({
  id: z.string(),
  constraint: z.string(),
  predicate: z.string().optional(),
});
export type ContextCondition = z.infer<typeof ContextCondition>;

// Core/state (uniform)
export const ContextState = BaseState;
export type ContextState = z.infer<typeof ContextState>;

// Facets structure for foundational data
export const ContextFacets = z
  .object({
    // Presuppositions: what is taken as given
    presuppositions: z.array(Presupposition).optional(),

    // Scope: modal and domain boundaries
    scope: ScopeSpec.optional(),

    // Conditions: constraints for validity
    conditions: z.array(ContextCondition).optional(),

    // Parent context reference (for nested scopes)
    parentContext: z.string().optional(),

    // Additional context-specific facets via catchall
  })
  .catchall(z.any());
export type ContextFacets = z.infer<typeof ContextFacets>;

// Representation record (live schema shape)
export const ContextShapeSchema = z.object({
  id: z.string(),
  type: Type,
  name: Label.optional(),
  description: z.string().optional(),
  state: ContextState.optional().default({}),
  entities: z.array(EntityRef).optional().default([]),
  relations: z.array(Id).optional().default([]),
  signature: z.record(z.string(), z.any()).optional(),
  facets: z.record(z.string(), z.any()).optional(),
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
export type ContextShapeRepo = z.infer<typeof ContextShapeSchema>;
// export type ContextShape = ContextShapeRepo;

// Type exports
export type ContextCoreOut = z.output<typeof Type>;
export type ContextStateOut = z.output<typeof ContextState>;
