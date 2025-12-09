import { z } from 'zod';

// Basic data types without presentation concerns
export const DataTypes = [
  'string',
  'number',
  'boolean',
  'date',
  'object',
  'array',
  'reference',
  'enum',
] as const;

// Property definition without presentation details
export const DataPropertySchema = z.object({
  type: z.enum(DataTypes),
  required: z.boolean().default(false),
  unique: z.boolean().default(false),
  default: z.any().optional(),
  validation: z.record(z.any()).optional(),
  // Reference to other entities (Link structure)
  reference: z.object({
    entity: z.string(),
    property: z.string(),
  }).optional(),
});

// Entity definition - pure data structure
export const DataEntitySchema = z.object({
  name: z.string(),
  properties: z.record(DataPropertySchema),
  primaryKey: z.string().default('id'),
});

export type DataProperty = z.infer<typeof DataPropertySchema>;
export type DataEntity = z.infer<typeof DataEntitySchema>;

// Domain schema - collection of entities
export const DataSchemaDefinition = z.object({
  entities: z.record(DataEntitySchema),
});

export type DataSchema = z.infer<typeof DataSchemaDefinition>;

// Define a data schema - concerned only with structure, not presentation
export function defineSchema(config: Partial<DataSchema>): DataSchema {
  return DataSchemaDefinition.parse({
    entities: config.entities || {}
  });
}

// Define a data entity
export function defineEntity(config: Partial<DataEntity>): DataEntity {
  return DataEntitySchema.parse({
    name: config.name || 'Unnamed Entity',
    properties: config.properties || {},
    primaryKey: config.primaryKey || 'id',
  });
}
