import { z } from 'zod';
import { FormShapeSchema } from './shape';

/**
 * Entity:Property:Relation Schema - Form Processor API
 * 
 * This schema surfaces Entity:Property:Relation from the Logic Form Processor
 * into the MVC DSL and FormApp abstraction.
 * 
 * Entity:Property:Relation is part of the Form Processor API, but we surface
 * it as part of the MVC DSL so that FormApps can work with it directly.
 * 
 * This maintains "Everything is a Form" while allowing MVC to work with
 * the concrete existence structure (Entity:Property:Relation).
 */

// Entity reference (from logic package)
export const EntityRefSchema = z.object({
  id: z.string().min(1),
  type: z.string(),
});

// Property reference (from logic package)
export const PropertyRefSchema = z.object({
  id: z.string().min(1),
  type: z.string(),
  name: z.string().optional(),
  contextId: z.string().optional(),
});

// Relation reference (from logic package)
export const RelationRefSchema = z.object({
  id: z.string().min(1),
  type: z.string(),
  sourceId: z.string(),
  targetId: z.string(),
  direction: z.enum(['directed', 'undirected', 'bidirectional']).optional(),
});

// Entity schema (surfaced in MVC DSL)
export const EntitySchema = z.object({
  id: z.string(),
  type: z.string(),
  name: z.string().optional(),
  properties: z.array(PropertyRefSchema).optional(),
  relations: z.array(RelationRefSchema).optional(),
  // FormShape reference - Everything is a Form
  formShape: FormShapeSchema.optional(),
});

// Property schema (surfaced in MVC DSL)
export const PropertySchema = z.object({
  id: z.string(),
  type: z.string(),
  name: z.string(),
  entityId: z.string().optional(),
  contextId: z.string().optional(),
  // Property characteristics
  qualitative: z.object({
    essential: z.boolean().optional(),
    observable: z.boolean().optional(),
    mutable: z.boolean().optional(),
    inherent: z.boolean().optional(),
  }).optional(),
  quantitative: z.object({
    dataType: z.string().optional(),
    unit: z.string().optional(),
    precision: z.number().optional(),
    range: z.object({
      min: z.any().optional(),
      max: z.any().optional(),
    }).optional(),
  }).optional(),
  // FormShape reference - Everything is a Form
  formShape: FormShapeSchema.optional(),
});

// Relation schema (surfaced in MVC DSL)
export const RelationSchema = z.object({
  id: z.string(),
  type: z.string(),
  sourceId: z.string(),
  targetId: z.string(),
  direction: z.enum(['directed', 'undirected', 'bidirectional']).default('directed'),
  // Relation characteristics
  essential: z.boolean().optional(),
  absolute: z.boolean().optional(),
  // FormShape reference - Everything is a Form
  formShape: FormShapeSchema.optional(),
});

// Entity:Property:Relation bundle (for FormApp)
export const EntityPropertyRelationSchema = z.object({
  entity: EntitySchema,
  properties: z.array(PropertySchema).optional(),
  relations: z.array(RelationSchema).optional(),
  // Context reference (from logic package)
  contextId: z.string().optional(),
  // FormShape reference - Everything is a Form
  formShape: FormShapeSchema.optional(),
});

// Export types
export type EntityRef = z.infer<typeof EntityRefSchema>;
export type PropertyRef = z.infer<typeof PropertyRefSchema>;
export type RelationRef = z.infer<typeof RelationRefSchema>;
export type Entity = z.infer<typeof EntitySchema>;
export type Property = z.infer<typeof PropertySchema>;
export type Relation = z.infer<typeof RelationSchema>;
export type EntityPropertyRelation = z.infer<typeof EntityPropertyRelationSchema>;

// Helper functions for MVC DSL
export function createEntity(input: {
  id: string;
  type: string;
  name?: string;
  properties?: PropertyRef[];
  relations?: RelationRef[];
}): Entity {
  return EntitySchema.parse({
    id: input.id,
    type: input.type,
    name: input.name,
    properties: input.properties ?? [],
    relations: input.relations ?? [],
  });
}

export function createProperty(input: {
  id: string;
  type: string;
  name: string;
  entityId?: string;
  contextId?: string;
}): Property {
  return PropertySchema.parse({
    id: input.id,
    type: input.type,
    name: input.name,
    entityId: input.entityId,
    contextId: input.contextId,
  });
}

export function createRelation(input: {
  id: string;
  type: string;
  sourceId: string;
  targetId: string;
  direction?: 'directed' | 'undirected' | 'bidirectional';
}): Relation {
  return RelationSchema.parse({
    id: input.id,
    type: input.type,
    sourceId: input.sourceId,
    targetId: input.targetId,
    direction: input.direction ?? 'directed',
  });
}

