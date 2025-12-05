import { z } from 'zod';

/**
 * Morph System - Transformation pipeline for forms
 * 
 * Salvaged from sankara/app/form/morph/core/types.ts
 * 
 * Morph represents transformations that can be composed into pipelines.
 * The morph system supports:
 * - Pure transformations (same input → same output)
 * - Fusible transformations (can be combined/optimized)
 * - Memoizable transformations (results can be cached)
 */

/**
 * Basic morph transformation function signature
 */
export type MorphTransformer<T, U> = (input: T, context?: any) => U;

/**
 * Post-processing function for composed morphs
 */
export type PostProcessor<T> = (result: T, context?: any) => T;

/**
 * Options for morph configuration
 */
export const MorphOptionsSchema = z.object({
  pure: z.boolean().default(true),         // Is this morph pure (same input always produces same output)?
  fusible: z.boolean().default(false),      // Can this morph be combined with others in optimization?
  cost: z.number().default(1),              // Relative computational cost
  memoizable: z.boolean().optional(),       // Can results be cached?
  description: z.string().optional(),      // Human-readable description
  tags: z.array(z.string()).optional(),    // Categorization tags
});

export type MorphOptions = z.infer<typeof MorphOptionsSchema>;

/**
 * Core morph interface schema
 */
export const MorphSchema = z.object({
  name: z.string(),
  options: MorphOptionsSchema,
  // transform is a function, so we can't fully validate it in Zod
  // but we can document it
}).passthrough(); // Allow additional properties like transform function

export type Morph<T, U> = {
  readonly name: string;
  readonly options: MorphOptions;
  transform: MorphTransformer<T, U>;
};

/**
 * Condition function type for conditionally applying morphs
 */
export type MorphCondition<T> = (input: T, context?: any) => boolean;

/**
 * Pipeline step types
 */
export const MorphStepSchema = z.discriminatedUnion('type', [
  z.object({
    type: z.literal('morph'),
    morph: MorphSchema,
  }),
  z.object({
    type: z.literal('map'),
    fn: z.function(), // Function type
  }),
  z.object({
    type: z.literal('conditional'),
    condition: z.function(), // Function type
    morph: MorphSchema,
  }),
]);

export type MorphStep<T, U> =
  | { type: "morph"; morph: Morph<T, U> }
  | { type: "map"; fn: (input: T, context?: any) => U }
  | { type: "conditional"; condition: MorphCondition<T>; morph: Morph<T, U> };

/**
 * Morph Pipeline - A sequence of morph transformations
 */
export const MorphPipelineSchema = z.object({
  id: z.string(),
  name: z.string(),
  steps: z.array(MorphStepSchema),
  options: MorphOptionsSchema.optional(),
});

export type MorphPipeline = z.infer<typeof MorphPipelineSchema>;

/**
 * Helper function to create a morph
 */
export function createMorph<T, U>(
  name: string,
  transformer: MorphTransformer<T, U>,
  options: Partial<MorphOptions> = {}
): Morph<T, U> {
  const mergedOptions: MorphOptions = {
    pure: true,
    fusible: false,
    cost: 1,
    memoizable: false,
    ...options
  };

  return {
    name,
    options: mergedOptions,
    transform: transformer
  };
}

/**
 * Compose two morphs into a new morph
 */
export function composeMorphs<T, U, V>(
  first: Morph<T, U>,
  second: Morph<U, V>,
  name = `${first.name}➝${second.name}`
): Morph<T, V> {
  return createMorph<T, V>(
    name,
    (input: T, context?: any): V => {
      const intermediate = first.transform(input, context);
      return second.transform(intermediate, context);
    },
    {
      pure: first.options.pure && second.options.pure,
      fusible: first.options.fusible && second.options.fusible,
      cost: first.options.cost + second.options.cost,
      memoizable: first.options.memoizable && second.options.memoizable
    }
  );
}

