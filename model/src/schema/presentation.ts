import { z } from 'zod';
import { FormShapeSchema } from './shape';
import { DashboardComponentSchema } from './dashboard';

/**
 * Presentation Schema: PPT-like Slide Decks
 * -----------------------------------------
 * Represents a sequence of slides with layouts and components.
 */

// Slide Layout types
export const SlideLayoutTypeSchema = z.enum([
  'title',
  'title-content',
  'two-column',
  'blank',
  'custom'
]);

// Slide Component (reusing Dashboard components + text/image)
export const SlideComponentSchema = DashboardComponentSchema.or(
  z.object({
    id: z.string(),
    type: z.literal('text-box'),
    content: z.string(), // HTML/Markdown
    position: z.object({
      x: z.number(), y: z.number(), w: z.number(), h: z.number()
    }),
    style: z.record(z.string(), z.any()).optional(),
  })
).or(
  z.object({
    id: z.string(),
    type: z.literal('image'),
    src: z.string(),
    alt: z.string().optional(),
    position: z.object({
      x: z.number(), y: z.number(), w: z.number(), h: z.number()
    }),
    style: z.record(z.string(), z.any()).optional(),
  })
);

// Slide definition
export const SlideSchema = z.object({
  id: z.string(),
  title: z.string().optional(),
  layout: SlideLayoutTypeSchema.default('title-content'),
  components: z.array(SlideComponentSchema).default([]),
  notes: z.string().optional(), // Speaker notes
  background: z.string().optional(), // Color or image URL
  transition: z.enum(['none', 'fade', 'slide', 'zoom']).default('none'),
});

// Presentation definition
export const PresentationShapeSchema = FormShapeSchema.extend({
  type: z.literal('presentation').default('presentation'),
  slides: z.array(SlideSchema),
  theme: z.string().optional(), // Theme ID or name
  aspectRatio: z.enum(['16:9', '4:3']).default('16:9'),
});

// Export types
export type SlideLayoutType = z.infer<typeof SlideLayoutTypeSchema>;
export type SlideComponent = z.infer<typeof SlideComponentSchema>;
export type Slide = z.infer<typeof SlideSchema>;
export type PresentationShape = z.infer<typeof PresentationShapeSchema>;
