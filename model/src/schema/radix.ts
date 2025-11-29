import { z } from 'zod';
import { FormShapeSchema } from './shape';

/**
 * Radix UI Schema: The "Real" TS-JSON Language
 * --------------------------------------------
 * Defines the data structure for Radix UI primitives.
 * This is the "Language" that the RadixAdapter speaks.
 */

// Common props for all Radix components
export const RadixCommonProps = z.object({
  id: z.string().optional(),
  className: z.string().optional(),
  style: z.record(z.string(), z.any()).optional(),
  asChild: z.boolean().optional(),
});

// --- Primitives ---

// Dialog
export const DialogSchema = z.object({
  type: z.literal('dialog'),
  trigger: z.any(), // DisplayElement
  title: z.string().optional(),
  description: z.string().optional(),
  content: z.any(), // DisplayElement
  open: z.boolean().optional(),
  defaultOpen: z.boolean().optional(),
  modal: z.boolean().default(true),
}).merge(RadixCommonProps);

// Popover
export const PopoverSchema = z.object({
  type: z.literal('popover'),
  trigger: z.any(),
  content: z.any(),
  open: z.boolean().optional(),
  defaultOpen: z.boolean().optional(),
  side: z.enum(['top', 'right', 'bottom', 'left']).optional(),
  align: z.enum(['start', 'center', 'end']).optional(),
}).merge(RadixCommonProps);

// Dropdown Menu
export type DropdownMenuItem = {
  label: string;
  shortcut?: string;
  icon?: string;
  disabled?: boolean;
  onSelect?: string;
  subContent?: DropdownMenuItem[];
};

export const DropdownMenuItemSchema: z.ZodType<DropdownMenuItem> = z.object({
  label: z.string(),
  shortcut: z.string().optional(),
  icon: z.string().optional(),
  disabled: z.boolean().optional(),
  onSelect: z.string().optional(), // Action ID
  subContent: z.array(z.lazy(() => DropdownMenuItemSchema)).optional(), // For submenus
});

export const DropdownMenuSchema = z.object({
  type: z.literal('dropdown-menu'),
  trigger: z.any(),
  items: z.array(DropdownMenuItemSchema),
  open: z.boolean().optional(),
  defaultOpen: z.boolean().optional(),
}).merge(RadixCommonProps);

// Tabs
export const TabsTriggerSchema = z.object({
  value: z.string(),
  label: z.string(),
  icon: z.string().optional(),
});

export const TabsContentSchema = z.object({
  value: z.string(),
  content: z.any(),
});

export const TabsSchema = z.object({
  type: z.literal('tabs'),
  defaultValue: z.string(),
  value: z.string().optional(),
  triggers: z.array(TabsTriggerSchema),
  contents: z.array(TabsContentSchema),
  orientation: z.enum(['horizontal', 'vertical']).default('horizontal'),
}).merge(RadixCommonProps);

// Accordion
export const AccordionItemSchema = z.object({
  value: z.string(),
  trigger: z.string(), // Text label
  content: z.any(),
  disabled: z.boolean().optional(),
});

export const AccordionSchema = z.object({
  type: z.literal('accordion'),
  typeProp: z.enum(['single', 'multiple']).default('single'),
  collapsible: z.boolean().optional(),
  defaultValue: z.string().or(z.array(z.string())).optional(),
  value: z.string().or(z.array(z.string())).optional(),
  items: z.array(AccordionItemSchema),
}).merge(RadixCommonProps);

// Tooltip
export const TooltipSchema = z.object({
  type: z.literal('tooltip'),
  trigger: z.any(),
  content: z.string(),
  delayDuration: z.number().optional(),
  side: z.enum(['top', 'right', 'bottom', 'left']).optional(),
}).merge(RadixCommonProps);

// Scroll Area
export const ScrollAreaSchema = z.object({
  type: z.literal('scroll-area'),
  content: z.any(),
  typeProp: z.enum(['auto', 'always', 'scroll', 'hover']).default('hover'),
  orientation: z.enum(['vertical', 'horizontal', 'both']).default('vertical'),
}).merge(RadixCommonProps);

// --- Union ---

export const RadixComponentSchema = z.discriminatedUnion('type', [
  DialogSchema,
  PopoverSchema,
  DropdownMenuSchema,
  TabsSchema,
  AccordionSchema,
  TooltipSchema,
  ScrollAreaSchema,
]);

// Export types
export type DialogShape = z.infer<typeof DialogSchema>;
export type PopoverShape = z.infer<typeof PopoverSchema>;
// DropdownMenuItemShape is exported above
export type DropdownMenuShape = z.infer<typeof DropdownMenuSchema>;
export type TabsTriggerShape = z.infer<typeof TabsTriggerSchema>;
export type TabsContentShape = z.infer<typeof TabsContentSchema>;
export type TabsShape = z.infer<typeof TabsSchema>;
export type AccordionItemShape = z.infer<typeof AccordionItemSchema>;
export type AccordionShape = z.infer<typeof AccordionSchema>;
export type TooltipShape = z.infer<typeof TooltipSchema>;
export type ScrollAreaShape = z.infer<typeof ScrollAreaSchema>;
export type RadixComponentShape = z.infer<typeof RadixComponentSchema>;
