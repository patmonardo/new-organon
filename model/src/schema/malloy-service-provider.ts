import { z } from 'zod';
import { MalloyModelSchema, MalloyViewSchema } from './malloy-model-view';
import { MorphSchema } from './shape-context-morph';

/**
 * Malloy Service Provider - Simpler NestJS Alternative
 * 
 * Malloy serves as a general Middleware Provider, replacing NestJS
 * with a simpler, more focused approach on transformations and relationships.
 * 
 * Architecture:
 * - Service Registry - Service registration and discovery
 * - Transformation Pipeline - Morph-based transformations
 * - Relationship Manager - Join and relationship management
 * - Model-View Coordinator - Dyadic structure coordination
 */

// Service Method Definition
export const ServiceMethodSchema = z.object({
  name: z.string(),
  // Input/output schemas
  input: z.any().optional(),
  output: z.any().optional(),
  // Malloy configuration
  malloy: z.object({
    view: z.string().optional(), // View ID
    transformations: z.array(z.string()).optional(), // Morph IDs
    model: z.string().optional(), // Model ID
  }).optional(),
  // Method metadata
  description: z.string().optional(),
  cache: z.boolean().optional().default(false),
  timeout: z.number().optional(),
});

// Service Definition
export const MalloyServiceSchema = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string().optional(),
  // Service configuration
  config: z.object({
    model: z.string(), // Malloy model ID
    view: z.string().optional(), // Default view ID
    transformations: z.array(z.string()).optional(), // Default Morph IDs
  }),
  // Service methods
  methods: z.record(ServiceMethodSchema),
  // Service metadata
  version: z.string().optional().default('1.0.0'),
  tags: z.array(z.string()).optional(),
});

// Service Registry
export const ServiceRegistrySchema = z.object({
  id: z.string(),
  name: z.string(),
  services: z.record(MalloyServiceSchema),
  // Registry metadata
  createdAt: z.number().optional(),
  updatedAt: z.number().optional(),
});

// Service Call Request
export const ServiceCallRequestSchema = z.object({
  serviceId: z.string(),
  method: z.string(),
  params: z.record(z.any()).optional(),
  // Call metadata
  requestId: z.string().optional(),
  timeout: z.number().optional(),
});

// Service Call Response
export const ServiceCallResponseSchema = z.object({
  requestId: z.string().optional(),
  success: z.boolean(),
  data: z.any().optional(),
  error: z.string().optional(),
  // Performance metadata
  duration: z.number().optional(),
  transformations: z.array(z.string()).optional(), // Applied Morph IDs
});

// Service Provider Configuration
export const ServiceProviderConfigSchema = z.object({
  id: z.string(),
  name: z.string(),
  // Models and Views
  models: z.record(MalloyModelSchema).optional(),
  views: z.record(MalloyViewSchema).optional(),
  transformations: z.record(MorphSchema).optional(),
  // Service registry
  registry: ServiceRegistrySchema.optional(),
  // Performance
  performance: z.object({
    maxConcurrency: z.number().optional(),
    timeout: z.number().optional(),
    cacheEnabled: z.boolean().default(true),
  }).optional(),
});

// Export types
export type ServiceMethod = z.infer<typeof ServiceMethodSchema>;
export type MalloyService = z.infer<typeof MalloyServiceSchema>;
export type ServiceRegistry = z.infer<typeof ServiceRegistrySchema>;
export type ServiceCallRequest = z.infer<typeof ServiceCallRequestSchema>;
export type ServiceCallResponse = z.infer<typeof ServiceCallResponseSchema>;
export type ServiceProviderConfig = z.infer<typeof ServiceProviderConfigSchema>;

// Helper functions
export function createMalloyService(input: {
  id: string;
  name: string;
  config: {
    model: string;
    view?: string;
    transformations?: string[];
  };
  methods: Record<string, {
    input?: any;
    output?: any;
    malloy?: {
      view?: string;
      transformations?: string[];
      model?: string;
    };
  }>;
}): MalloyService {
  return MalloyServiceSchema.parse({
    id: input.id,
    name: input.name,
    config: input.config,
    methods: Object.entries(input.methods).reduce((acc, [name, method]) => {
      acc[name] = {
        name,
        input: method.input,
        output: method.output,
        malloy: method.malloy,
      };
      return acc;
    }, {} as Record<string, ServiceMethod>),
  });
}

export function createServiceProviderConfig(input: {
  id: string;
  name: string;
  models?: Record<string, z.infer<typeof MalloyModelSchema>>;
  views?: Record<string, z.infer<typeof MalloyViewSchema>>;
  transformations?: Record<string, z.infer<typeof MorphSchema>>;
}): ServiceProviderConfig {
  return ServiceProviderConfigSchema.parse({
    id: input.id,
    name: input.name,
    models: input.models,
    views: input.views,
    transformations: input.transformations,
    performance: {
      maxConcurrency: 10,
      timeout: 5000,
      cacheEnabled: true,
    },
  });
}

