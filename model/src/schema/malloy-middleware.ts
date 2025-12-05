import { z } from 'zod';
import { MalloyModelSchema, MalloyViewSchema } from './malloy-model-view';
import { MorphSchema } from './shape-context-morph';

/**
 * Malloy Middleware Iron - Structural Support for React/Next Server Actions
 * 
 * Malloy serves as the "Middleware Iron" - the strong, foundational layer
 * that supports heavy UI layers like React/Next with their intense ServerAction
 * RPC patterns.
 * 
 * This schema provides the middleware abstraction that sits between:
 * - React/Next (UI Layer) - Intense ServerAction RPC plates
 * - Data Layer (Polars/Arrow) - DataFrames, EDA, Analytics
 */

// Server Action Request
export const ServerActionRequestSchema = z.object({
  id: z.string(),
  type: z.string(),
  action: z.string(), // Action name
  params: z.record(z.any()).optional(),
  // Malloy context
  malloyContext: z.object({
    modelId: z.string().optional(),
    viewId: z.string().optional(),
    transformations: z.array(z.string()).optional(), // Morph IDs
  }).optional(),
});

// Server Action Response
export const ServerActionResponseSchema = z.object({
  id: z.string(),
  success: z.boolean(),
  data: z.any().optional(),
  error: z.string().optional(),
  // Malloy result
  malloyResult: z.object({
    view: MalloyViewSchema.optional(),
    transformations: z.array(MorphSchema).optional(),
  }).optional(),
});

// Malloy Middleware Configuration
export const MalloyMiddlewareConfigSchema = z.object({
  id: z.string(),
  name: z.string(),
  // Models
  models: z.record(MalloyModelSchema),
  // Views
  views: z.record(MalloyViewSchema),
  // Transformations (Morphs)
  transformations: z.record(MorphSchema).optional(),
  // Caching
  cache: z.object({
    enabled: z.boolean().default(true),
    ttl: z.number().optional(), // Time to live in seconds
    strategy: z.enum(['memory', 'redis', 'file']).default('memory'),
  }).optional(),
  // Performance
  performance: z.object({
    parallelExecution: z.boolean().default(true),
    maxConcurrency: z.number().optional(),
    timeout: z.number().optional(), // Timeout in milliseconds
  }).optional(),
});

// Malloy Middleware Handler
export const MalloyMiddlewareHandlerSchema = z.object({
  id: z.string(),
  name: z.string(),
  // Handler function (string reference or function)
  handler: z.string(), // Function name or path
  // Input schema
  input: z.any().optional(),
  // Output schema
  output: z.any().optional(),
  // Malloy configuration
  malloy: z.object({
    model: z.string(), // Model ID
    view: z.string().optional(), // View ID
    transformations: z.array(z.string()).optional(), // Morph IDs
  }),
});

// Malloy Middleware Pipeline
export const MalloyMiddlewarePipelineSchema = z.object({
  id: z.string(),
  name: z.string(),
  // Pipeline steps
  steps: z.array(z.object({
    id: z.string(),
    type: z.enum(['transform', 'query', 'aggregate', 'filter', 'join']),
    config: z.any(),
    morph: MorphSchema.optional(), // Morph transformation
  })),
  // Input
  input: z.any().optional(),
  // Output
  output: z.any().optional(),
});

// Export types
export type ServerActionRequest = z.infer<typeof ServerActionRequestSchema>;
export type ServerActionResponse = z.infer<typeof ServerActionResponseSchema>;
export type MalloyMiddlewareConfig = z.infer<typeof MalloyMiddlewareConfigSchema>;
export type MalloyMiddlewareHandler = z.infer<typeof MalloyMiddlewareHandlerSchema>;
export type MalloyMiddlewarePipeline = z.infer<typeof MalloyMiddlewarePipelineSchema>;

// Helper functions
export function createMalloyMiddlewareConfig(input: {
  id: string;
  name: string;
  models: Record<string, z.infer<typeof MalloyModelSchema>>;
  views: Record<string, z.infer<typeof MalloyViewSchema>>;
}): MalloyMiddlewareConfig {
  return MalloyMiddlewareConfigSchema.parse({
    id: input.id,
    name: input.name,
    models: input.models,
    views: input.views,
    cache: {
      enabled: true,
      strategy: 'memory',
    },
    performance: {
      parallelExecution: true,
    },
  });
}

export function createMalloyMiddlewareHandler(input: {
  id: string;
  name: string;
  handler: string;
  malloy: {
    model: string;
    view?: string;
    transformations?: string[];
  };
}): MalloyMiddlewareHandler {
  return MalloyMiddlewareHandlerSchema.parse({
    id: input.id,
    name: input.name,
    handler: input.handler,
    malloy: input.malloy,
  });
}

