import { z } from 'zod';

// Add support for confirmation dialogs and HeroIcons
export const ButtonVariantSchema = z.enum(['primary', 'secondary', 'ghost', 'danger']);
export const ButtonSizeSchema = z.enum(['sm', 'md', 'lg']).default('md');
export const IconSourceSchema = z.enum(['heroicons', 'custom']).default('heroicons');

// Button schema definition with enhanced features
export const ButtonShapeSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  variant: ButtonVariantSchema,
  size: ButtonSizeSchema.optional(),
  icon: z.string().optional(),
  iconSource: IconSourceSchema.optional(),
  label: z.string(),
  href: z.string().optional(),
  disabled: z.boolean().optional().default(false),
  confirmMessage: z.string().optional(),
  refreshAfterAction: z.boolean().optional().default(false),
  srOnly: z.boolean().optional().default(false),
  customClass: z.string().optional()
});

export type ButtonShape = z.infer<typeof ButtonShapeSchema>;

export function defineButton(config: Partial<ButtonShape> & { id: string; label: string }): ButtonShape {
  return ButtonShapeSchema.parse({
    id: config.id,
    name: config.name,
    variant: config.variant || 'primary',
    label: config.label,
    size: config.size,
    icon: config.icon,
    iconSource: config.iconSource,
    href: config.href,
    disabled: config.disabled ?? false,
    confirmMessage: config.confirmMessage,
    refreshAfterAction: config.refreshAfterAction ?? false,
    srOnly: config.srOnly ?? false,
    customClass: config.customClass
  });
}
