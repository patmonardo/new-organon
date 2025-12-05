import { z } from 'zod';
import { MalloyIRSchema, MeasureIRSchema, DimensionIRSchema, JoinIRSchema, QueryIRSchema } from './malloy-ir';

/**
 * Malloy Expression Language - Common Analytics Language
 * 
 * Malloy serves as a common "Expression" analytics language that can be used
 * in either GDSL (Rust) or MVC (TypeScript), helping us unify GDS Rust and TS worlds.
 * 
 * This schema defines the common expression language that works across both worlds.
 */

// Expression context (which world it's used in)
export const ExpressionContextSchema = z.enum(['gdsl', 'mvc', 'common']);

// Malloy Expression (common across both worlds)
export const MalloyExpressionSchema = z.object({
  // Expression identifier
  id: z.string(),
  name: z.string().optional(),
  
  // Source
  source: z.string(),
  
  // Measures (aggregations) - common semantics
  measures: z.record(MeasureIRSchema).optional(),
  
  // Dimensions (groupings) - common semantics
  dimensions: z.record(DimensionIRSchema).optional(),
  
  // Joins (relationships) - common semantics
  joins: z.record(JoinIRSchema).optional(),
  
  // Query expression
  query: QueryIRSchema.optional(),
  
  // Context (which world it's used in)
  context: ExpressionContextSchema.optional().default('common'),
  
  // Malloy IR (full IR structure)
  malloyIR: MalloyIRSchema.optional(),
  
  // Compilation targets
  compilation: z.object({
    gdsl: z.any().optional(), // Compiled to GDSL (Rust)
    mvc: z.any().optional(),   // Compiled to MVC (TypeScript)
  }).optional(),
});

// Expression compilation result
export const ExpressionCompilationSchema = z.object({
  expressionId: z.string(),
  target: ExpressionContextSchema,
  compiled: z.any(), // Compiled expression for target world
  metadata: z.object({
    compilationTime: z.number().optional(),
    optimizations: z.array(z.string()).optional(),
  }).optional(),
});

// Cross-world expression mapping
export const CrossWorldExpressionSchema = z.object({
  // Common expression
  expression: MalloyExpressionSchema,
  
  // GDSL (Rust) compilation
  gdslCompilation: ExpressionCompilationSchema.optional(),
  
  // MVC (TypeScript) compilation
  mvcCompilation: ExpressionCompilationSchema.optional(),
  
  // Unified semantics
  unifiedSemantics: z.object({
    measures: z.record(z.string()).optional(), // Measure name mappings
    dimensions: z.record(z.string()).optional(), // Dimension name mappings
    joins: z.record(z.string()).optional(), // Join name mappings
  }).optional(),
});

// Export types
export type ExpressionContext = z.infer<typeof ExpressionContextSchema>;
export type MalloyExpression = z.infer<typeof MalloyExpressionSchema>;
export type ExpressionCompilation = z.infer<typeof ExpressionCompilationSchema>;
export type CrossWorldExpression = z.infer<typeof CrossWorldExpressionSchema>;

// Helper functions
export function createMalloyExpression(input: {
  id: string;
  name?: string;
  source: string;
  measures?: Record<string, z.infer<typeof MeasureIRSchema>>;
  dimensions?: Record<string, z.infer<typeof DimensionIRSchema>>;
  joins?: Record<string, z.infer<typeof JoinIRSchema>>;
  query?: z.infer<typeof QueryIRSchema>;
  context?: ExpressionContext;
}): MalloyExpression {
  return MalloyExpressionSchema.parse({
    id: input.id,
    name: input.name,
    source: input.source,
    measures: input.measures,
    dimensions: input.dimensions,
    joins: input.joins,
    query: input.query,
    context: input.context ?? 'common',
  });
}

export function createCrossWorldExpression(input: {
  expression: MalloyExpression;
  gdslCompilation?: ExpressionCompilation;
  mvcCompilation?: ExpressionCompilation;
}): CrossWorldExpression {
  return CrossWorldExpressionSchema.parse({
    expression: input.expression,
    gdslCompilation: input.gdslCompilation,
    mvcCompilation: input.mvcCompilation,
  });
}

