//@graphics/schema/link.ts
import { z } from 'zod';

/**
 * Link schema - the fundamental navigation element
 * Used for all navigational elements across the system
 */
export const LinkSchema = z.object({
  label: z.string(),
  href: z.string(),
  active: z.boolean().optional(),
  relation: z.enum(['navigate', 'action', 'reference']).optional(),
  icon: z.string().optional(),
});

export type Link = z.infer<typeof LinkSchema>;

/**
 * BreadcrumbItem - specialized link for breadcrumb navigation
 * Reuses LinkSchema to maintain schema coherence
 */
export type BreadcrumbItem = Link;

/**
 * Breadcrumb schema - defines a complete breadcrumb navigation path
 * Composed of multiple BreadcrumbItems representing a navigation hierarchy
 */
export const BreadcrumbSchema = z.object({
  path: z.array(LinkSchema),
  separator: z.string().optional().default('/'),
  font: z.string().optional(),
});

export type Breadcrumb = z.infer<typeof BreadcrumbSchema>;
