import { z } from 'zod';
import { FormShapeSchema } from './shape';
import { ListShapeSchema } from './list';
import { LinkShapeSchema } from './link';
import { DashboardShapeSchema, DashboardComponentSchema } from './dashboard';

/**
 * Application Schema - "Desktop Neo4j"
 *
 * The Application schema represents a complete FormApp definition,
 * including dashboard, navigation, views, models, and forms.
 *
 * This is the "Desktop Neo4j" concept - a complete application
 * that combines:
 * - Malloy-inspired semantic data models
 * - D3/Recharts visualizations
 * - Navigation (Lists/Links)
 * - Forms (FormShape)
 * - Dashboard layout
 */

// Dashboard section schema
export const DashboardSectionSchema = z.object({
  id: z.string(),
  title: z.string().optional(),
  type: z.enum(['metrics', 'chart', 'table', 'list', 'form']),
  content: z.any(), // Can be chart config, table data, form shape, etc.
  layout: z.object({
    column: z.number().optional(),
    row: z.number().optional(),
    span: z.number().optional(),
  }).optional(),
});

// Dashboard schema
export const DashboardSchema = z.object({
  id: z.string(),
  name: z.string(),
  title: z.string().optional(),
  description: z.string().optional(),
  sections: z.array(DashboardSectionSchema),
  layout: z.enum(['grid', 'stack', 'tabs']).default('grid'),
  columns: z.number().optional().default(2),
});

// Navigation schema (using LinkShape)
export const NavigationSchema = z.object({
  id: z.string(),
  name: z.string(),
  type: z.enum(['navbar', 'sidebar', 'breadcrumbs']),
  items: z.array(LinkShapeSchema),
  layout: z.object({
    orientation: z.enum(['horizontal', 'vertical']).optional(),
    position: z.enum(['top', 'bottom', 'left', 'right']).optional(),
  }).optional(),
});

// View schema (Malloy-inspired)
export const ViewSchema = z.object({
  id: z.string(),
  name: z.string(),
  title: z.string().optional(),
  description: z.string().optional(),
  type: z.enum(['malloy', 'chart', 'table', 'form', 'custom', 'dashboard']),
  source: z.string(), // Reference to DataModel
  query: z.any().optional(), // ViewQuery
  config: z.record(z.string(), z.any()).optional(), // View-specific config
  // Malloy View technology - TS module ready to execute MVC IR
  module: z.string().optional(), // Path to TS module
  // DisplayShape for rendering
  // Dashboard component (if type is 'dashboard')
  dashboardComponent: DashboardComponentSchema.optional(),
});

// DataModel reference schema
export const DataModelRefSchema = z.object({
  id: z.string(),
  name: z.string(),
  source: z.string().optional(), // Path to model definition
});

// Application schema
export const ApplicationSchema = z.object({
  id: z.string(),
  name: z.string(),
  title: z.string().optional(),
  description: z.string().optional(),

  // Dashboard - main interface (extends FormShape)
  dashboard: DashboardShapeSchema,

  // Navigation - Lists/Links
  navigation: z.array(NavigationSchema).optional(),

  // Views - Malloy views, charts, tables, forms
  views: z.array(ViewSchema).optional(),

  // Data Models - Malloy-inspired semantic models
  models: z.array(DataModelRefSchema).optional(),

  // Forms - FormShape definitions
  forms: z.array(FormShapeSchema).optional(),

  // Lists - For breadcrumbs, navbars, etc.
  lists: z.array(ListShapeSchema).optional(),

  // Metadata
  metadata: z.object({
    version: z.string().optional(),
    author: z.string().optional(),
    createdAt: z.number().optional(),
    updatedAt: z.number().optional(),
  }).optional(),
});

// Export types
export type DashboardSection = z.infer<typeof DashboardSectionSchema>;
export type Dashboard = z.infer<typeof DashboardSchema>;
export type DashboardShape = z.infer<typeof DashboardShapeSchema>;
export type Navigation = z.infer<typeof NavigationSchema>;
export type View = z.infer<typeof ViewSchema>;
export type DataModelRef = z.infer<typeof DataModelRefSchema>;
export type Application = z.infer<typeof ApplicationSchema>;

// Helper functions
export function createApplication(input: {
  id?: string;
  name: string;
  title?: string;
  description?: string;
  dashboard: DashboardShape; // Use DashboardShape instead of Dashboard
  navigation?: Navigation[];
  views?: View[];
  models?: DataModelRef[];
  forms?: z.infer<typeof FormShapeSchema>[];
  lists?: z.infer<typeof ListShapeSchema>[];
}): Application {
  const id = input.id ?? `app:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6).toString(36).padStart(4, '0')}`;

  return ApplicationSchema.parse({
    id,
    name: input.name,
    title: input.title,
    description: input.description,
    dashboard: input.dashboard,
    navigation: input.navigation ?? [],
    views: input.views ?? [],
    models: input.models ?? [],
    forms: input.forms ?? [],
    lists: input.lists ?? [],
    metadata: {
      createdAt: Date.now(),
      updatedAt: Date.now(),
    },
  });
}

