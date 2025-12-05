import { z } from 'zod';
import { DisplayShapeSchema } from './display';

/**
 * Shape:Context:Morph Schema - Form Processor API
 * 
 * This schema surfaces Shape:Context:Morph from the Logic Form Processor
 * into the MVC DSL and FormApp abstraction.
 * 
 * Shape:Context:Morph is part of the Form Processor API, mirroring
 * Entity:Property:Relation. Together they form the complete Form API:
 * - Shape:Context:Morph - Pure Form processing (principles)
 * - Entity:Property:Relation - Concrete existence (instances)
 * 
 * Morph = Being-for-self (Ground) as the active unity of Shape ∧ Context.
 * Context defines determinations (Reflection) and the Property catalog (predicates);
 * Shape supplies types; Morph composes admissible constructions over Entities as
 * instances of Shapes under a given Context.
 */

// Shape reference (from logic package)
export const ShapeRefSchema = z.object({
  id: z.string().min(1),
  type: z.string(),
  name: z.string().optional(),
});

// Context reference (from logic package)
export const ContextRefSchema = z.object({
  id: z.string().min(1),
  type: z.string(),
  name: z.string().optional(),
  scope: z.string().optional(), // Modal scope
});

// Morph reference (from logic package)
export const MorphRefSchema = z.object({
  id: z.string().min(1),
  type: z.string(),
  name: z.string().optional(),
  inputType: z.string().optional(),
  outputType: z.string().optional(),
});

// Shape schema (surfaced in MVC DSL)
export const ShapeSchema = z.object({
  id: z.string(),
  type: z.string(),
  name: z.string().optional(),
  // Shape characteristics
  dialecticState: z.any().optional(), // DialecticState from logic
  phase: z.string().optional(), // Current dialectical phase
  // DisplayShape reference - Everything is a Form
  displayShape: DisplayShapeSchema.optional(),
});

// Context schema (surfaced in MVC DSL)
export const ContextSchema = z.object({
  id: z.string(),
  type: z.string(),
  name: z.string().optional(),
  scope: z.string().optional(), // Modal scope
  // Context characteristics
  determinations: z.array(z.string()).optional(), // Determinations (Reflection)
  propertyCatalog: z.array(z.string()).optional(), // Property catalog (predicates)
  // DisplayShape reference - Everything is a Form
  displayShape: DisplayShapeSchema.optional(),
});

// Morph schema (surfaced in MVC DSL)
export const MorphSchema = z.object({
  id: z.string(),
  type: z.string(),
  name: z.string().optional(),
  inputType: z.string().optional().default('FormShape'),
  outputType: z.string().optional().default('FormShape'),
  // Morph characteristics
  transformation: z.object({
    principle: z.string().optional(), // Transformation principle (Reason/Ratio)
    mechanism: z.string().optional(), // Mechanism of change
    middleTerm: z.string().optional(), // Middle term (mediation)
  }).optional(),
  container: z.object({
    holds: z.array(z.string()).optional(), // What it holds (active unity)
    activeUnity: z.array(z.object({
      name: z.string(),
      definition: z.string().optional(),
      contains: z.array(z.string()).optional(),
    })).optional(),
  }).optional(),
  ground: z.object({
    form: z.string().optional(), // From Shape
    scope: z.string().optional(), // From Context
    principle: z.string().optional(), // The reason
  }).optional(),
  // DisplayShape reference - Everything is a Form
  displayShape: DisplayShapeSchema.optional(),
});

// Shape:Context:Morph bundle (for FormApp)
export const ShapeContextMorphSchema = z.object({
  shape: ShapeSchema,
  context: ContextSchema,
  morph: MorphSchema,
  // DisplayShape reference - Everything is a Form
  displayShape: DisplayShapeSchema.optional(),
});

// Export types
export type ShapeRef = z.infer<typeof ShapeRefSchema>;
export type ContextRef = z.infer<typeof ContextRefSchema>;
export type MorphRef = z.infer<typeof MorphRefSchema>;
export type Shape = z.infer<typeof ShapeSchema>;
export type Context = z.infer<typeof ContextSchema>;
export type Morph = z.infer<typeof MorphSchema>;
export type ShapeContextMorph = z.infer<typeof ShapeContextMorphSchema>;

// Helper functions for MVC DSL
export function createShape(input: {
  id: string;
  type: string;
  name?: string;
}): Shape {
  return ShapeSchema.parse({
    id: input.id,
    type: input.type,
    name: input.name,
  });
}

export function createContext(input: {
  id: string;
  type: string;
  name?: string;
  scope?: string;
}): Context {
  return ContextSchema.parse({
    id: input.id,
    type: input.type,
    name: input.name,
    scope: input.scope,
  });
}

export function createMorph(input: {
  id: string;
  type: string;
  name?: string;
  inputType?: string;
  outputType?: string;
}): Morph {
  return MorphSchema.parse({
    id: input.id,
    type: input.type,
    name: input.name,
    inputType: input.inputType ?? 'FormShape',
    outputType: input.outputType ?? 'FormShape',
  });
}

