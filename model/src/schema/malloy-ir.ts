import { z } from 'zod';

/**
 * Malloy IR Schema - Rich Intermediate Representation
 * 
 * Malloy has a rich set of IR that we can directly embed in our TS-JSON schemas.
 * This schema represents the Malloy IR structure, which we can use to:
 * - Embed Malloy IR in TS-JSON schemas
 * - Generate TS-JSON from Malloy IR
 * - Modify Malloy to generate TS-JSON instead of SQL
 */

// Measure IR
export const MeasureIRSchema = z.object({
  type: z.enum(['sum', 'count', 'avg', 'min', 'max', 'custom']),
  field: z.string().optional(),
  expression: z.string().optional(),
  // Additional measure properties
  filter: z.any().optional(),
  distinct: z.boolean().optional(),
});

// Dimension IR
export const DimensionIRSchema = z.object({
  field: z.string(),
  truncation: z.enum(['year', 'quarter', 'month', 'day', 'hour', 'minute']).optional(),
  expression: z.string().optional(),
  // Additional dimension properties
  format: z.string().optional(),
  timezone: z.string().optional(),
});

// Join IR
export const JoinIRSchema = z.object({
  model: z.string(), // Reference to another source/model
  on: z.string(), // Join condition
  type: z.enum(['left', 'right', 'inner', 'full']).default('left'),
  // Additional join properties
  relationship: z.string().optional(), // Relationship type
  required: z.boolean().optional().default(false),
});

// Filter IR
export const FilterIRSchema = z.object({
  field: z.string(),
  operator: z.enum(['=', '!=', '>', '<', '>=', '<=', 'in', 'not in', 'like', 'not like']),
  value: z.any(),
  // Additional filter properties
  logic: z.enum(['and', 'or']).optional(),
  conditions: z.array(z.any()).optional(), // Nested conditions
});

// Source IR
export const SourceIRSchema = z.object({
  type: z.enum(['table', 'query']),
  sql: z.string().optional(), // SQL definition (if type is 'table')
  // Measures
  measures: z.record(MeasureIRSchema).optional(),
  // Dimensions
  dimensions: z.record(DimensionIRSchema).optional(),
  // Joins
  joins: z.record(JoinIRSchema).optional(),
  // Additional source properties
  primaryKey: z.string().optional(),
  description: z.string().optional(),
});

// Query IR
export const QueryIRSchema = z.object({
  source: z.string(), // Reference to source
  // Group by (dimensions)
  group_by: z.array(z.string()).optional(),
  // Aggregate (measures)
  aggregate: z.array(z.string()).optional(),
  // Filter
  filter: FilterIRSchema.optional(),
  // Limit
  limit: z.number().optional(),
  // Order by
  order_by: z.array(z.object({
    field: z.string(),
    direction: z.enum(['asc', 'desc']).default('asc'),
  })).optional(),
  // Additional query properties
  having: FilterIRSchema.optional(),
  nest: z.record(QueryIRSchema).optional(), // Nested queries
});

// Complete Malloy IR
export const MalloyIRSchema = z.object({
  // Sources
  sources: z.record(SourceIRSchema),
  // Queries
  queries: z.record(QueryIRSchema).optional(),
  // Metadata
  metadata: z.object({
    version: z.string().optional(),
    description: z.string().optional(),
    author: z.string().optional(),
  }).optional(),
});

// TS-JSON Schema with embedded Malloy IR
export const TSJSONWithMalloyIRSchema = z.object({
  // Standard TS-JSON fields
  name: z.string(),
  source: z.string(),
  fields: z.record(z.any()).optional(),
  
  // Measures (TS-JSON format)
  measures: z.record(MeasureIRSchema).optional(),
  
  // Dimensions (TS-JSON format)
  dimensions: z.record(DimensionIRSchema).optional(),
  
  // Joins (TS-JSON format)
  joins: z.record(JoinIRSchema).optional(),
  
  // Malloy IR embedded directly
  malloyIR: MalloyIRSchema,
  
  // Metadata
  metadata: z.object({
    version: z.string().optional(),
    createdAt: z.number().optional(),
    updatedAt: z.number().optional(),
  }).optional(),
});

// Export types
export type MeasureIR = z.infer<typeof MeasureIRSchema>;
export type DimensionIR = z.infer<typeof DimensionIRSchema>;
export type JoinIR = z.infer<typeof JoinIRSchema>;
export type FilterIR = z.infer<typeof FilterIRSchema>;
export type SourceIR = z.infer<typeof SourceIRSchema>;
export type QueryIR = z.infer<typeof QueryIRSchema>;
export type MalloyIR = z.infer<typeof MalloyIRSchema>;
export type TSJSONWithMalloyIR = z.infer<typeof TSJSONWithMalloyIRSchema>;

// Helper functions
export function createMalloyIR(input: {
  sources: Record<string, z.infer<typeof SourceIRSchema>>;
  queries?: Record<string, z.infer<typeof QueryIRSchema>>;
}): MalloyIR {
  return MalloyIRSchema.parse({
    sources: input.sources,
    queries: input.queries,
  });
}

export function embedMalloyIRInTSJSON(input: {
  name: string;
  source: string;
  fields?: Record<string, any>;
  measures?: Record<string, z.infer<typeof MeasureIRSchema>>;
  dimensions?: Record<string, z.infer<typeof DimensionIRSchema>>;
  joins?: Record<string, z.infer<typeof JoinIRSchema>>;
  malloyIR: MalloyIR;
}): TSJSONWithMalloyIR {
  return TSJSONWithMalloyIRSchema.parse({
    name: input.name,
    source: input.source,
    fields: input.fields,
    measures: input.measures,
    dimensions: input.dimensions,
    joins: input.joins,
    malloyIR: input.malloyIR,
  });
}

