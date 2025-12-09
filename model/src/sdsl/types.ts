/**
 * MVC SDSL Core Types
 *
 * These types define the language that the MVC SDSL speaks.
 * They are received from @logic/FactStore and transformed
 * through the Model→View→Controller pipeline.
 */

import { z } from 'zod';

// ============================================================
// FORM SHAPE - The Universal Data Shape
// ============================================================

/**
 * FormField: A single field in a form
 * This is what the FactStore speaks as Property
 */
export const FormFieldSchema = z.object({
  id: z.string(),
  type: z.string().default('text'),
  label: z.string().optional(),
  value: z.unknown().optional(),
  required: z.boolean().default(false),
  disabled: z.boolean().default(false),
  placeholder: z.string().optional(),
  validation: z.record(z.string(), z.unknown()).optional(),
});

export type FormField = z.infer<typeof FormFieldSchema>;

/**
 * FormShape: The universal shape that flows through the SDSL
 * This is what the FactStore speaks as Entity
 */
export const FormShapeSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  title: z.string().optional(),
  description: z.string().optional(),
  fields: z.array(FormFieldSchema).default([]),
  meta: z.record(z.string(), z.unknown()).optional(),
});

export type FormShape = z.infer<typeof FormShapeSchema>;

// ============================================================
// FORM MODE - How the Form is Being Used
// ============================================================

export const FormModeSchema = z.enum(['create', 'edit', 'view']);
export type FormMode = z.infer<typeof FormModeSchema>;

// ============================================================
// FORM CONTENT - Generic Display Language
// ============================================================

/**
 * FormContent: The output format of the View
 * This is the Generic Display Language that Adapters consume
 *
 * - jsx: React/JSX components
 * - json: API/data serialization
 * - html: Server-rendered HTML
 * - xml: XML serialization
 */
export const FormContentSchema = z.enum(['jsx', 'json', 'html', 'xml']);
export type FormContent = z.infer<typeof FormContentSchema>;

// ============================================================
// FORM HANDLER - Action Callbacks
// ============================================================

/**
 * FormHandler: Callbacks for form actions
 * The Controller uses these to respond to user interactions
 */
export interface FormHandler {
  onSubmit?: (data: FormShape) => Promise<void> | void;
  onCancel?: () => void;
  onAction?: (actionId: string, data?: unknown) => Promise<void> | void;
  onChange?: (fieldId: string, value: unknown) => void;
}

// ============================================================
// OPERATION RESULT - Success/Error Wrapper
// ============================================================

/**
 * OperationResult: Wraps the result of any SDSL operation
 */
export interface OperationResult<T> {
  status: 'success' | 'error' | 'pending';
  data: T | null;
  message?: string;
  errors?: Record<string, string[]>;
}

// ============================================================
// CONTROLLER RESULT - Transport-agnostic response
// ============================================================

/**
 * ControllerResult: What controllers return (transport-agnostic)
 * 
 * - tRPC returns this directly
 * - HTTP wraps in JSON response
 * - Next.js can redirect based on this
 */
export interface ControllerResult<T = unknown> {
  success: boolean;
  data?: T;
  error?: string;
  redirect?: string;
  meta?: Record<string, unknown>;
}

/**
 * FormDefinition: What the form looks like (for rendering)
 */
export interface FormDefinition {
  shape: FormShape;
  values: Record<string, unknown>;
  mode: FormMode;
  actions: string[];
}

/**
 * ListResult: Paginated list response
 */
export interface ListResult<T> {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
  totalPages: number;
}

// ============================================================
// DISPLAY PRIMITIVES - Generic Display Language Elements
// ============================================================
// NOTE: DisplayDocument/DisplayElement are being phased out in favor of
// direct Shape rendering (ButtonShape, ListShape, CardShape, etc.)
// See react-shape-adapter.tsx and react-shape-view.tsx for the new approach.
// These types are kept for backward compatibility with existing FormView/ReactView.

/**
 * DisplayElement: A generic display element
 * This is what the View produces and Adapters consume
 * @deprecated Use Shape objects directly (ButtonShape, ListShape, etc.)
 */
export interface DisplayElement {
  type: string;
  props?: Record<string, unknown>;
  children?: DisplayElement[];
  text?: string;
}

export const DisplayElementSchema: z.ZodType<DisplayElement> = z.object({
  type: z.string(),
  props: z.record(z.string(), z.unknown()).optional(),
  children: z.lazy(() => z.array(DisplayElementSchema)).optional(),
  text: z.string().optional(),
});

/**
 * DisplayLayout: Layout container for display elements
 * @deprecated Use Shape objects directly (ButtonShape, ListShape, etc.)
 */
export const DisplayLayoutSchema = z.object({
  type: z.enum(['stack', 'row', 'grid', 'card', 'page']),
  gap: z.number().optional(),
  columns: z.number().optional(),
  padding: z.number().optional(),
  children: z.array(DisplayElementSchema).default([]),
});

export type DisplayLayout = z.infer<typeof DisplayLayoutSchema>;

/**
 * DisplayDocument: Complete display output from the View
 * This is the Generic Display Language document
 * @deprecated Use Shape objects directly (ButtonShape, ListShape, etc.)
 */
export const DisplayDocumentSchema = z.object({
  title: z.string().optional(),
  layout: DisplayLayoutSchema,
  meta: z.record(z.string(), z.unknown()).optional(),
});

export type DisplayDocument = z.infer<typeof DisplayDocumentSchema>;
