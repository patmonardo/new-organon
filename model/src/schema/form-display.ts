import { z } from 'zod';
import { DisplayShapeSchema, FieldDisplayShapeSchema } from './display';

/**
 * FormDisplay Schema - DisplayShape-based Form Schema
 * 
 * This refactors the form schema toward DisplayShape-based structure.
 * Forms are now primarily DisplayShapes with additional form-specific
 * metadata and behavior.
 * 
 * This makes the MVC schema more Dashboard Display Controller oriented
 * toward ordinary EDA dataframes.
 */

// Form field as DisplayShape
export const FormFieldDisplaySchema = FieldDisplayShapeSchema.extend({
  // Form-specific metadata
  name: z.string().optional(),
  label: z.string().optional(),
  required: z.boolean().optional().default(false),
  validation: z.object({
    required: z.boolean().optional(),
    min: z.number().optional(),
    max: z.number().optional(),
    minLength: z.number().optional(),
    maxLength: z.number().optional(),
    pattern: z.string().optional(),
    custom: z.function().optional(),
    message: z.string().optional(),
  }).optional(),
  // Data access
  data: z.object({
    path: z.string().optional(),
    transform: z.function().optional(),
    default: z.any().optional(),
  }).optional(),
});

// Form as DisplayShape
export const FormDisplaySchema = DisplayShapeSchema.extend({
  // Form-specific metadata
  name: z.string(),
  title: z.string().optional(),
  description: z.string().optional(),
  mode: z.enum(['create', 'edit', 'view']).default('create'),
  // Form fields as DisplayShapes
  fields: z.array(FormFieldDisplaySchema).optional(),
  // Form state
  state: z.object({
    status: z.enum(['idle', 'submitting', 'success', 'error']),
    errors: z.record(z.array(z.string())).optional(),
    message: z.string().optional(),
  }).optional(),
  // Form actions
  actions: z.array(z.object({
    id: z.string(),
    type: z.enum(['submit', 'reset', 'cancel', 'delete']),
    label: z.string().optional(),
    primary: z.boolean().optional().default(false),
  })).optional(),
  // Layout
  layout: z.object({
    columns: z.enum(['single', 'double']).optional(),
    sections: z.array(z.object({
      id: z.string(),
      title: z.string().optional(),
      fields: z.array(z.string()).optional(), // Field IDs
    })).optional(),
  }).optional(),
});

// Dashboard Display Controller - EDA DataFrame oriented
export const DashboardDisplayControllerSchema = z.object({
  id: z.string(),
  name: z.string(),
  // Data source (EDA DataFrame)
  dataSource: z.object({
    type: z.enum(['dataframe', 'query', 'api', 'function']),
    source: z.string(), // Path to data or query
    schema: z.any().optional(), // DataFrame schema
  }),
  // Display configuration
  display: DisplayShapeSchema,
  // Controller behavior
  controller: z.object({
    // Transformations (Morph-based)
    transformations: z.array(z.string()).optional(), // Morph IDs
    // Relationships (Malloy-based)
    relationships: z.array(z.string()).optional(), // Join/relationship IDs
    // Actions
    actions: z.array(z.object({
      id: z.string(),
      type: z.string(),
      handler: z.string().optional(), // Function name
    })).optional(),
  }).optional(),
});

// Export types
export type FormFieldDisplay = z.infer<typeof FormFieldDisplaySchema>;
export type FormDisplay = z.infer<typeof FormDisplaySchema>;
export type DashboardDisplayController = z.infer<typeof DashboardDisplayControllerSchema>;

// Helper functions
export function createFormDisplay(input: {
  id: string;
  name: string;
  component: string;
  props: z.infer<typeof DisplayShapeSchema>['props'];
  fields?: FormFieldDisplay[];
}): FormDisplay {
  return FormDisplaySchema.parse({
    type: 'form',
    component: input.component,
    props: {
      id: input.id,
      ...input.props,
      fields: input.fields?.map(f => ({
        id: f.id,
        component: f.component,
        props: f.props,
      })),
    },
    name: input.name,
    fields: input.fields,
  });
}

export function createDashboardDisplayController(input: {
  id: string;
  name: string;
  dataSource: z.infer<typeof DashboardDisplayControllerSchema>['dataSource'];
  display: z.infer<typeof DisplayShapeSchema>;
}): DashboardDisplayController {
  return DashboardDisplayControllerSchema.parse({
    id: input.id,
    name: input.name,
    dataSource: input.dataSource,
    display: input.display,
  });
}

