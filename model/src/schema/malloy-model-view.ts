import { z } from 'zod';
import { DisplayShapeSchema } from './display';
import { MorphSchema } from './shape-context-morph';

/**
 * Malloy Model-View Schema - Dyadic Structure
 * 
 * Malloy Model-View is Dyadic in focus:
 * - Model = State : Structure (Data + Schema)
 * - View = Representation : Perspective (Query + Display)
 * 
 * This merges Morph-Relationships into Malloy-based Model-View.
 * The Morph system provides transformations and relationships,
 * which align perfectly with Malloy's focus on Transformations
 * and Relationships.
 * 
 * Rich Morphology: The complete system of transformations and
 * relationships that enable data modeling and visualization.
 */

// Malloy Source (Model)
export const MalloySourceSchema = z.object({
  name: z.string(),
  type: z.enum(['table', 'query']),
  sql: z.string().optional(),
  // Measures - aggregations
  measures: z.record(z.object({
    type: z.enum(['sum', 'count', 'avg', 'min', 'max', 'custom']),
    field: z.string().optional(),
    expression: z.string().optional(),
  })).optional(),
  // Dimensions - groupings
  dimensions: z.record(z.object({
    field: z.string(),
    truncation: z.enum(['year', 'quarter', 'month', 'day']).optional(),
    expression: z.string().optional(),
  })).optional(),
  // Joins - relationships
  joins: z.record(z.object({
    model: z.string(), // Reference to another source
    on: z.string(), // Join condition
    type: z.enum(['left', 'right', 'inner', 'full']).default('left'),
  })).optional(),
});

// Malloy Query (View)
export const MalloyQuerySchema = z.object({
  source: z.string(), // Reference to source
  // Group by - dimensions
  group_by: z.array(z.string()).optional(),
  // Aggregate - measures
  aggregate: z.array(z.string()).optional(),
  // Filter
  filter: z.any().optional(),
  // Limit
  limit: z.number().optional(),
  // Order by
  order_by: z.array(z.object({
    field: z.string(),
    direction: z.enum(['asc', 'desc']).default('asc'),
  })).optional(),
});

// Malloy View (Dyadic: Representation : Perspective)
export const MalloyViewSchema = z.object({
  id: z.string(),
  name: z.string(),
  // Representation - the data structure
  representation: z.object({
    source: MalloySourceSchema,
    query: MalloyQuerySchema.optional(),
  }),
  // Perspective - how it's displayed
  perspective: z.object({
    display: DisplayShapeSchema,
    layout: z.any().optional(), // Layout configuration
    visualizations: z.array(z.string()).optional(), // Chart types
  }),
  // Morph integration - transformations and relationships
  morph: MorphSchema.optional(),
  // Module - TS module ready to execute MVC IR
  module: z.string().optional(),
});

// Malloy Model (Dyadic: State : Structure)
export const MalloyModelSchema = z.object({
  id: z.string(),
  name: z.string(),
  // State - the data state
  state: z.object({
    sources: z.record(MalloySourceSchema),
    currentSource: z.string().optional(),
  }),
  // Structure - the schema structure
  structure: z.object({
    measures: z.record(z.any()).optional(),
    dimensions: z.record(z.any()).optional(),
    joins: z.record(z.any()).optional(),
  }),
  // Morph integration - transformations
  morph: MorphSchema.optional(),
});

// Malloy Model-View Bundle
export const MalloyModelViewSchema = z.object({
  model: MalloyModelSchema,
  views: z.array(MalloyViewSchema),
  // Rich Morphology - complete transformation system
  morphology: z.object({
    transformations: z.array(MorphSchema).optional(),
    relationships: z.array(z.object({
      id: z.string(),
      type: z.string(),
      source: z.string(),
      target: z.string(),
      morph: MorphSchema.optional(),
    })).optional(),
  }).optional(),
});

// Export types
export type MalloySource = z.infer<typeof MalloySourceSchema>;
export type MalloyQuery = z.infer<typeof MalloyQuerySchema>;
export type MalloyView = z.infer<typeof MalloyViewSchema>;
export type MalloyModel = z.infer<typeof MalloyModelSchema>;
export type MalloyModelView = z.infer<typeof MalloyModelViewSchema>;

// Helper functions
export function createMalloyModel(input: {
  id: string;
  name: string;
  sources: Record<string, z.infer<typeof MalloySourceSchema>>;
}): MalloyModel {
  return MalloyModelSchema.parse({
    id: input.id,
    name: input.name,
    state: {
      sources: input.sources,
      currentSource: Object.keys(input.sources)[0],
    },
    structure: {},
  });
}

export function createMalloyView(input: {
  id: string;
  name: string;
  source: z.infer<typeof MalloySourceSchema>;
  query?: z.infer<typeof MalloyQuerySchema>;
  display: z.infer<typeof DisplayShapeSchema>;
}): MalloyView {
  return MalloyViewSchema.parse({
    id: input.id,
    name: input.name,
    representation: {
      source: input.source,
      query: input.query,
    },
    perspective: {
      display: input.display,
    },
  });
}

