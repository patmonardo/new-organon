import { z } from 'zod';
import { FormShapeSchema } from './shape';

/**
 * Document Schema: Word-like Rich Text
 * ------------------------------------
 * Represents a rich text document with blocks, formatting, and structure.
 */

// Inline style (bold, italic, etc.)
export const InlineStyleSchema = z.object({
  bold: z.boolean().optional(),
  italic: z.boolean().optional(),
  underline: z.boolean().optional(),
  strike: z.boolean().optional(),
  code: z.boolean().optional(),
  color: z.string().optional(),
  background: z.string().optional(),
  link: z.string().optional(), // URL
});

// Text span with style
export const TextSpanSchema = z.object({
  text: z.string(),
  style: InlineStyleSchema.optional(),
});

// Block types
export const BlockTypeSchema = z.enum([
  'paragraph',
  'heading-1',
  'heading-2',
  'heading-3',
  'bullet-list',
  'numbered-list',
  'quote',
  'code-block',
  'image',
  'divider',
  'table', // Could reference GridShape
]);

// Block definition with explicit type for recursion
export type Block = {
  id: string;
  type: BlockType;
  content?: TextSpan[];
  children?: Block[];
  attributes?: Record<string, any>;
  style?: Record<string, any>;
};

export const BlockSchema: z.ZodType<Block> = z.object({
  id: z.string(),
  type: BlockTypeSchema,
  content: z.array(TextSpanSchema).optional(),
  children: z.array(z.lazy(() => BlockSchema)).optional(), // For nested blocks (lists)
  attributes: z.record(z.string(), z.any()).optional(), // Metadata (e.g., image src, code language)
  style: z.record(z.string(), z.any()).optional(), // Block-level styles (align, indent)
});

// Document definition
export const DocumentShapeSchema = FormShapeSchema.extend({
  type: z.literal('document').default('document'),
  title: z.string().optional(),
  blocks: z.array(BlockSchema),
  authors: z.array(z.string()).optional(),
  version: z.string().optional(),
});

// Export types
export type InlineStyle = z.infer<typeof InlineStyleSchema>;
export type TextSpan = z.infer<typeof TextSpanSchema>;
export type BlockType = z.infer<typeof BlockTypeSchema>;
// Block is already exported above
export type DocumentShape = z.infer<typeof DocumentShapeSchema>;
