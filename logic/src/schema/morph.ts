import { z } from 'zod';
import { BaseState, Label, Type } from './base';

// Composition (pipeline/composite descriptor)
export const MorphComposition = z
  .object({
    kind: z.enum(['single', 'pipeline', 'composite']).default('single'),
    mode: z.enum(['sequential', 'parallel']).optional(),
    steps: z.array(z.string()).default([]),
  })
  .default({ kind: 'single', steps: [] });
export type MorphComposition = z.infer<typeof MorphComposition>;

// Facets structure for ground/transformation data
export const MorphFacets = z
  .object({
    dialecticState: z.any().optional(),
    phase: z.string().optional(),
    container: z
      .object({
        holds: z.array(z.string()),
        activeUnity: z.array(
          z.object({
            name: z.string(),
            definition: z.string(),
            contains: z.string().optional(),
          }),
        ),
      })
      .optional(),
    transformation: z
      .object({
        principle: z.string().optional(),
        mechanism: z.string().optional(),
        transitions: z.array(
          z.object({
            from: z.string(),
            to: z.string(),
            mechanism: z.string().optional(),
            reason: z.string().optional(),
          }),
        ),
      })
      .optional(),
    context: z.any().optional(),
  })
  .catchall(z.any());
export type MorphFacets = z.infer<typeof MorphFacets>;

// Record-style Morph shape for repositories and engines
export const MorphShapeSchema = z.object({
  id: z.string(),
  type: Type,
  name: Label.optional(),
  description: z.string().optional(),
  inputType: z.string().default('FormShape'),
  outputType: z.string().default('FormShape'),
  transformFn: z.string().optional(),

  state: BaseState.optional().default({}),
  signature: z.object({}).catchall(z.any()).optional(),
  facets: MorphFacets.optional().default({}),

  composition: MorphComposition,
  config: z.record(z.string(), z.unknown()).optional().default({}),

  status: z.string().optional(),
  tags: z.array(z.string()).optional(),
  meta: z.record(z.string(), z.unknown()).optional(),

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
export type MorphShapeRepo = z.infer<typeof MorphShapeSchema>;
export type MorphShape = MorphShapeRepo;
export const MorphSchema = MorphShapeSchema;
export type Morph = MorphShapeRepo;

export function getMorphContainer(morph: Morph): any | undefined {
  return (morph.facets as any)?.container;
}

export function getMorphTransformation(morph: Morph): any | undefined {
  return (morph.facets as any)?.transformation;
}
