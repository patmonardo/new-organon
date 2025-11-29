import { z } from 'zod';
import { FormShapeSchema } from './shape';

/**
 * Grid Schema: Excel-like Data Grid
 * --------------------------------
 * Represents tabular data with cell-level control, formulas, and formatting.
 */

// Cell coordinates
export const CellAddressSchema = z.object({
  row: z.number().int().nonnegative(),
  col: z.number().int().nonnegative(),
  sheet: z.string().optional(),
});

// Cell style/formatting
export const CellStyleSchema = z.object({
  bold: z.boolean().optional(),
  italic: z.boolean().optional(),
  underline: z.boolean().optional(),
  align: z.enum(['left', 'center', 'right']).optional(),
  background: z.string().optional(),
  color: z.string().optional(),
  format: z.string().optional(), // e.g., 'currency', 'percent', 'date'
});

// Cell data
export const CellSchema = z.object({
  id: z.string(),
  address: CellAddressSchema,
  value: z.any(),
  formula: z.string().optional(), // e.g., "=SUM(A1:A10)"
  displayValue: z.string().optional(), // Calculated/Formatted value
  style: CellStyleSchema.optional(),
  type: z.enum(['string', 'number', 'boolean', 'date', 'error']).default('string'),
});

// Column definition (for metadata/defaults)
export const GridColumnSchema = z.object({
  id: z.string(),
  index: z.number().int().nonnegative(),
  width: z.number().optional(),
  title: z.string().optional(),
  hidden: z.boolean().optional(),
});

// Row definition (for metadata/defaults)
export const GridRowSchema = z.object({
  id: z.string(),
  index: z.number().int().nonnegative(),
  height: z.number().optional(),
  hidden: z.boolean().optional(),
});

// Sheet definition
export const GridSheetSchema = z.object({
  id: z.string(),
  name: z.string(),
  cells: z.record(z.string(), CellSchema), // Key: "row:col" or "A1"
  columns: z.record(z.string(), GridColumnSchema).optional(),
  rows: z.record(z.string(), GridRowSchema).optional(),
  config: z.object({
    frozenRows: z.number().optional(),
    frozenCols: z.number().optional(),
    showGridLines: z.boolean().default(true),
  }).optional(),
});

// Workbook (Grid) definition
export const GridShapeSchema = FormShapeSchema.extend({
  type: z.literal('grid').default('grid'),
  sheets: z.array(GridSheetSchema),
  activeSheetId: z.string().optional(),
});

// Export types
export type CellAddress = z.infer<typeof CellAddressSchema>;
export type CellStyle = z.infer<typeof CellStyleSchema>;
export type Cell = z.infer<typeof CellSchema>;
export type GridColumn = z.infer<typeof GridColumnSchema>;
export type GridRow = z.infer<typeof GridRowSchema>;
export type GridSheet = z.infer<typeof GridSheetSchema>;
export type GridShape = z.infer<typeof GridShapeSchema>;
